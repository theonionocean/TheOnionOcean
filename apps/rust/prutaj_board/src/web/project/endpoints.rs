use crate::{create_project, delete_project, get_project, update_project};
use actix_web::web;
use actix_web::Scope;

pub fn register_project_endpoints() -> Scope {
    web::scope("/api/project")
        .service(create_project)
        .service(get_project)
        .service(update_project)
        .service(delete_project)
}
