use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{GetOrganizationQuery, Organization};

#[utoipa::path(
    get,
    path = "/organization/get/{id}",
    context_path = "/api",
    tag = "organization",
    params(
        ("id" = String, Path, description = "Organization id")
    ),
    responses(
        (status = 200, description = "Organization retrieved successfully", body = Organization),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[get("/get/{id}")]
pub async fn get_organization(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetOrganizationQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
