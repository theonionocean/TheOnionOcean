use crate::{create_organization, delete_organization, get_organization, update_organization};
use actix_web::web;
use actix_web::Scope;

pub fn register_organization_endpoints() -> Scope {
    web::scope("/api/organization")
        .service(create_organization)
        .service(get_organization)
        .service(update_organization)
        .service(delete_organization)
}
