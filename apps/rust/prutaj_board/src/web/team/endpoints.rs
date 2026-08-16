use crate::{create_team, delete_team, get_team, update_team};
use actix_web::web;
use actix_web::Scope;

pub fn register_team_endpoints() -> Scope {
    web::scope("/api/team")
        .service(create_team)
        .service(get_team)
        .service(update_team)
        .service(delete_team)
}
