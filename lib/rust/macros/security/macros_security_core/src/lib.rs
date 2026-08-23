use actix_web::HttpResponse;
pub use macros_security::has_role;
use zitadel::actix::introspection::IntrospectedUser;

pub fn require_role(user: &IntrospectedUser, role: &str) -> Result<(), HttpResponse> {
    let has_role = user
        .project_roles
        .as_ref()
        .is_some_and(|roles| roles.contains_key(&role.to_string()));

    if has_role {
        Ok(())
    } else {
        return Err(HttpResponse::Forbidden().json("User does not have the required role"));
    }
}
