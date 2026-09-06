use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateProjectCommand, Project, ProblemDetails};

#[utoipa::path(
    post,
    path = "/project/create",
    context_path = "/api",
    tag = "project",
    request_body = CreateProjectCommand,
    responses(
        (status = 200, description = "Project created successfully", body = Project),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[post("/create")]
pub async fn create_project(
    mediatr: Data<Mediatr>,
    req: Json<CreateProjectCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => crate::problem_details::into_response(e),
    }
}
