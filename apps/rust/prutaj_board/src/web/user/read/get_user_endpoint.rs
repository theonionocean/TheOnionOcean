use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{GetUserQuery, User};

#[utoipa::path(
    get,
    path = "/user/get/{id}",
    context_path = "/api",
    tag = "user",
    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = User),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[get("/get/{id}")]
pub async fn get_user(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetUserQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
