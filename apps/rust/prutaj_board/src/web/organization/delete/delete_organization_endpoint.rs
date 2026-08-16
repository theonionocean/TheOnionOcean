use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::DeleteOrganizationCommand;

#[utoipa::path(
    delete,
    path = "/organization/delete",
    context_path = "/api",
    tag = "organization",
    request_body = DeleteOrganizationCommand,
    responses(
        (status = 200, description = "Organization deleted successfully"),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[delete("/delete")]
pub async fn delete_organization(
    mediatr: Data<Mediatr>,
    req: Json<DeleteOrganizationCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
