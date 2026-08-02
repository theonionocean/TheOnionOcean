use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{Organization, UpdateOrganizationCommand};

#[utoipa::path(
    put,
    path = "/organization/update",
    context_path = "/api",
    tag = "organization",
    request_body = UpdateOrganizationCommand,
    responses(
        (status = 200, description = "Organization updated successfully", body = Organization),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[put("/organization/update")]
pub async fn update_organization(
    mediatr: Data<Mediatr>,
    req: Json<UpdateOrganizationCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
