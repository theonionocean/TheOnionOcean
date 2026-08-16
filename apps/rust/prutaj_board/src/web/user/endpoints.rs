use crate::{create_user, delete_user, get_user, update_user};
use actix_web::web;
use actix_web::Scope;

pub fn register_user_endpoints() -> Scope {
    web::scope("/api/user")
        .service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
}
