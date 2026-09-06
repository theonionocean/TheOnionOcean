use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{DeleteProjectCommand, ProblemDetails};

#[utoipa::path(
    delete,
    path = "/project/delete",
    context_path = "/api",
    tag = "project",
    request_body = DeleteProjectCommand,
    responses(
        (status = 200, description = "Project deleted successfully"),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[delete("/delete")]
pub async fn delete_project(
    mediatr: Data<Mediatr>,
    req: Json<DeleteProjectCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => crate::problem_details::into_response(e),
    }
}
