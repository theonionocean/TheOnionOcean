use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{Team, UpdateTeamCommand, ProblemDetails};

#[utoipa::path(
    put,
    path = "/team/update",
    context_path = "/api",
    tag = "team",
    request_body = UpdateTeamCommand,
    responses(
        (status = 200, description = "Team updated successfully", body = Team),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[put("/update")]
pub async fn update_team(mediatr: Data<Mediatr>, req: Json<UpdateTeamCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(e) => crate::problem_details::into_response(e),
    }
}
