use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{DeleteTaskCommand, ProblemDetails};

#[utoipa::path(
    delete,
    path = "/task/delete",
    context_path = "/api",
    tag = "task",
    request_body = DeleteTaskCommand,
    responses(
        (status = 200, description = "Task deleted successfully"),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[delete("/delete")]
pub async fn delete_task(mediatr: Data<Mediatr>, req: Json<DeleteTaskCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => crate::problem_details::into_response(e),
    }
}
