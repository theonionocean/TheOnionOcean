use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{UpdateUserCommand, User};

#[utoipa::path(
    put,
    path = "/user/update",
    context_path = "/api",
    tag = "user",
    request_body = UpdateUserCommand,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[put("/update")]
pub async fn update_user(mediatr: Data<Mediatr>, req: Json<UpdateUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
