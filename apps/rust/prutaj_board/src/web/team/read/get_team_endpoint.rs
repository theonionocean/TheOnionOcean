use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{GetTeamQuery, Team};

#[utoipa::path(
    get,
    path = "/team/get/{id}",
    context_path = "/api",
    tag = "team",
    params(
        ("id" = String, Path, description = "Team id")
    ),
    responses(
        (status = 200, description = "Team retrieved successfully", body = Team),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[get("/team/get/{id}")]
pub async fn get_team(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetTeamQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
