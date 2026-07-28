use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::GetUserQuery;

#[get("/user/get/{id}")]
async fn get_user(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetUserQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
