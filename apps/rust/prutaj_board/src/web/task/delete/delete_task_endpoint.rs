use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::DeleteTaskCommand;

#[utoipa::path(
    delete,
    path = "/task/delete",
    context_path = "/api",
    tag = "task",
    request_body = DeleteTaskCommand,
    responses(
        (status = 200, description = "Task deleted successfully"),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[delete("/task/delete")]
pub async fn delete_task(mediatr: Data<Mediatr>, req: Json<DeleteTaskCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
