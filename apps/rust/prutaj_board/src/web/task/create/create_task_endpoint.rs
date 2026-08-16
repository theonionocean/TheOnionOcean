use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateTaskCommand, Task};

#[utoipa::path(
    post,
    path = "/task/create",
    context_path = "/api",
    tag = "task",
    request_body = CreateTaskCommand,
    responses(
        (status = 200, description = "Task created successfully", body = Task),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("/create")]
pub async fn create_task(mediatr: Data<Mediatr>, req: Json<CreateTaskCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
