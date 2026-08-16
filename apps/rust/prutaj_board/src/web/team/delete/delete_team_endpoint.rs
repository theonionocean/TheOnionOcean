use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::DeleteTeamCommand;

#[utoipa::path(
    delete,
    path = "/team/delete",
    context_path = "/api",
    tag = "team",
    request_body = DeleteTeamCommand,
    responses(
        (status = 200, description = "Team deleted successfully"),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[delete("/delete")]
pub async fn delete_team(mediatr: Data<Mediatr>, req: Json<DeleteTeamCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
