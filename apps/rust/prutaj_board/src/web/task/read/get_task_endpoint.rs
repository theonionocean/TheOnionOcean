use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::GetTaskQuery;

#[get("/task/get/{id}")]
pub async fn get_task(mediatr: Data<Mediatr>, path: Path<String>) -> impl Responder {
    let query = GetTaskQuery {
        id: path.into_inner(),
    };
    let result = mediatr.send_query(query).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
