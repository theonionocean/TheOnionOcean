use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateTeamCommand, Team, ProblemDetails};

#[utoipa::path(
    post,
    path = "/team/create",
    context_path = "/api",
    tag = "team",
    request_body = CreateTeamCommand,
    responses(
        (status = 200, description = "Team created successfully", body = Team),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[post("/create")]
pub async fn create_team(mediatr: Data<Mediatr>, req: Json<CreateTeamCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(e) => crate::problem_details::into_response(e),
    }
}
