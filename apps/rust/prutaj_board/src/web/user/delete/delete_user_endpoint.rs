use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{DeleteUserCommand, ProblemDetails};

#[utoipa::path(
    delete,
    path = "/user/delete",
    context_path = "/api",
    tag = "user",
    request_body = DeleteUserCommand,
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[delete("/delete")]
pub async fn delete_user(mediatr: Data<Mediatr>, req: Json<DeleteUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => crate::problem_details::into_response(e),
    }
}
