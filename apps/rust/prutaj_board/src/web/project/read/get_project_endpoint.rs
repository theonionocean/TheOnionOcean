use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{GetProjectQuery, Project};

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
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[get("/project/get/{id}")]
pub async fn get_project(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetProjectQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
