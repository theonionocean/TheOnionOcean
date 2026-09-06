use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{DeleteTeamCommand, ProblemDetails};

#[utoipa::path(
    delete,
    path = "/team/delete",
    context_path = "/api",
    tag = "team",
    request_body = DeleteTeamCommand,
    responses(
        (status = 200, description = "Team deleted successfully"),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[delete("/delete")]
pub async fn delete_team(mediatr: Data<Mediatr>, req: Json<DeleteTeamCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(e) => crate::problem_details::into_response(e),
    }
}
