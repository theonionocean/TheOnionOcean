use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateOrganizationCommand, Organization, ProblemDetails};

#[utoipa::path(
    post,
    path = "/organization/create",
    context_path = "/api",
    tag = "organization",
    request_body = CreateOrganizationCommand,
    responses(
        (status = 200, description = "Organization created successfully", body = Organization),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
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
        Err(e) => crate::problem_details::into_response(e),
    }
}
