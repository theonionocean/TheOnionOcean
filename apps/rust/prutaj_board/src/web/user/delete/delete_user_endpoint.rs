use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::DeleteUserCommand;

#[delete("/user/delete")]
async fn delete_user(mediatr: Data<Mediatr>, req: Json<DeleteUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
