use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::DeleteProjectCommand;

#[utoipa::path(
    delete,
    path = "/project/delete",
    context_path = "/api",
    tag = "project",
    request_body = DeleteProjectCommand,
    responses(
        (status = 200, description = "Project deleted successfully"),
        (status = 500, description = "Internal server error", body = String)
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
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
