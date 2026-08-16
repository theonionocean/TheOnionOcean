use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateOrganizationCommand, Organization};

#[utoipa::path(
    post,
    path = "/organization/create",
    context_path = "/api",
    tag = "organization",
    request_body = CreateOrganizationCommand,
    responses(
        (status = 200, description = "Organization created successfully", body = Organization),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("/create")]
pub async fn create_organization(
    mediatr: Data<Mediatr>,
    req: Json<CreateOrganizationCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
