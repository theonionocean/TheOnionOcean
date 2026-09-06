use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{Task, UpdateTaskCommand, ProblemDetails};

#[utoipa::path(
    put,
    path = "/task/update",
    context_path = "/api",
    tag = "task",
    request_body = UpdateTaskCommand,
    responses(
        (status = 200, description = "Task updated successfully", body = Task),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[put("/update")]
pub async fn update_task(mediatr: Data<Mediatr>, req: Json<UpdateTaskCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => crate::problem_details::into_response(e),
    }
}
