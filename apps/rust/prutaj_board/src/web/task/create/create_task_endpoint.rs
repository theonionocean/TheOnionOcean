use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateTaskCommand, Task, ProblemDetails};

#[utoipa::path(
    post,
    path = "/task/create",
    context_path = "/api",
    tag = "task",
    request_body = CreateTaskCommand,
    responses(
        (status = 200, description = "Task created successfully", body = Task),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[post("/create")]
pub async fn create_task(mediatr: Data<Mediatr>, req: Json<CreateTaskCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => crate::problem_details::into_response(e),
    }
}
