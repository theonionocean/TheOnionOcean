use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{CreateUserCommand, User};

#[utoipa::path(
    post,
    path = "/user/create",
    context_path = "/api",
    tag = "user",
    request_body = CreateUserCommand,
    responses(
        (status = 200, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("/create")]
pub async fn create_user(mediatr: Data<Mediatr>, req: Json<CreateUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
