use crate::{create_task, delete_task, get_task, update_task};
use actix_web::web;
use actix_web::Scope;

pub fn register_task_endpoints() -> Scope {
    web::scope("/api/task")
        .service(create_task)
        .service(get_task)
        .service(update_task)
        .service(delete_task)
}
