use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::{UpdateUserCommand, User, ProblemDetails};

#[utoipa::path(
    put,
    path = "/user/update",
    context_path = "/api",
    tag = "user",
    request_body = UpdateUserCommand,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 400, description = "Validation failed", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 501, description = "Not implemented", body = ProblemDetails)
    )
)]
#[put("/update")]
pub async fn update_user(mediatr: Data<Mediatr>, req: Json<UpdateUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => crate::problem_details::into_response(e),
    }
}
