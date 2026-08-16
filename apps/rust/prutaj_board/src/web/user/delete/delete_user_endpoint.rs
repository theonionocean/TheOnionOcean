use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::DeleteUserCommand;

#[utoipa::path(
    delete,
    path = "/user/delete",
    context_path = "/api",
    tag = "user",
    request_body = DeleteUserCommand,
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[delete("/delete")]
pub async fn delete_user(mediatr: Data<Mediatr>, req: Json<DeleteUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
