use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{Task, UpdateTaskCommand};

#[utoipa::path(
    put,
    path = "/task/update",
    context_path = "/api",
    tag = "task",
    request_body = UpdateTaskCommand,
    responses(
        (status = 200, description = "Task updated successfully", body = Task),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[put("/task/update")]
pub async fn update_task(mediatr: Data<Mediatr>, req: Json<UpdateTaskCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
