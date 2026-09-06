use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{GetProjectQuery, Project, ProblemDetails};

#[utoipa::path(
    get,
    path = "/project/get/{id}",
    context_path = "/api",
    tag = "project",
    params(
        ("id" = String, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Project retrieved successfully", body = Project),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[get("/get/{id}")]
pub async fn get_project(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetProjectQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => crate::problem_details::into_response(e),
    }
}
