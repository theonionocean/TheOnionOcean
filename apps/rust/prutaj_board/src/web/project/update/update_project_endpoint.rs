use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{Project, UpdateProjectCommand};

#[utoipa::path(
    put,
    path = "/project/update",
    context_path = "/api",
    tag = "project",
    request_body = UpdateProjectCommand,
    responses(
        (status = 200, description = "Project updated successfully", body = Project),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[put("/project/update")]
pub async fn update_project(
    mediatr: Data<Mediatr>,
    req: Json<UpdateProjectCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
