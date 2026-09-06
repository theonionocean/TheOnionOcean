use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{GetTaskQuery, Task, ProblemDetails};

#[utoipa::path(
    get,
    path = "/task/get/{id}",
    context_path = "/api",
    tag = "task",
    params(
        ("id" = String, Path, description = "Task id")
    ),
    responses(
        (status = 200, description = "Task retrieved successfully", body = Task),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[get("/get/{id}")]
pub async fn get_task(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetTaskQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => crate::problem_details::into_response(e),
    }
}
