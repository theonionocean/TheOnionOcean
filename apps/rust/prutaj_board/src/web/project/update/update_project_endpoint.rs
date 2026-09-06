use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{Project, UpdateProjectCommand, ProblemDetails};

#[utoipa::path(
    put,
    path = "/project/update",
    context_path = "/api",
    tag = "project",
    request_body = UpdateProjectCommand,
    responses(
        (status = 200, description = "Project updated successfully", body = Project),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[put("/update")]
pub async fn update_project(
    mediatr: Data<Mediatr>,
    req: Json<UpdateProjectCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => crate::problem_details::into_response(e),
    }
}
