use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::UpdateUserCommand;

#[put("/user/update")]
async fn update_user(mediatr: Data<Mediatr>, req: Json<UpdateUserCommand>) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
