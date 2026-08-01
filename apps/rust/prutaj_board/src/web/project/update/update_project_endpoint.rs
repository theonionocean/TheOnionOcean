use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mediatr::Mediatr;

use crate::UpdateProjectCommand;

#[put("/project/update")]
pub async fn update_project(
    mediatr: Data<Mediatr>,
    req: Json<UpdateProjectCommand>,
) -> impl Responder {
    let command = req.into_inner();
    let result = mediatr.send_command(command).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
