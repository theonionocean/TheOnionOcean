use actix_web::{web, App, HttpServer};
use mediatr::Mediatr;
use prutaj_board::{
    create_organization, create_project, create_task, create_team, create_user,
    delete_organization, delete_project, delete_task, delete_team, delete_user, get_organization,
    get_project, get_task, get_team, get_user, update_organization, update_project, update_task,
    update_team, update_user, CreateOrganizationCommandHandler, CreateProjectCommandHandler,
    CreateTaskCommandHandler, CreateTeamCommandHandler, CreateUserCommandHandler,
    DeleteOrganizationCommandHandler, DeleteProjectCommandHandler, DeleteTaskCommandHandler,
    DeleteTeamCommandHandler, DeleteUserCommandHandler, GetOrganizationQueryHandler,
    GetProjectQueryHandler, GetTaskQueryHandler, GetTeamQueryHandler, GetUserQueryHandler,
    UpdateOrganizationCommandHandler, UpdateProjectCommandHandler, UpdateTaskCommandHandler,
    UpdateTeamCommandHandler, UpdateUserCommandHandler,
};

use surrealdb_extensions::DatabaseContext;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("infra/prutaj_board/prutaj-board.env").ok();

    let host = std::env::var("SURREALDB_HOST")
        .unwrap_or_else(|e| panic!("SURREALDB_HOST is not set: {e}"));
    let username = std::env::var("SURREALDB_USER")
        .unwrap_or_else(|e| panic!("SURREALDB_USER is not set: {e}"));
    let password = std::env::var("SURREALDB_PASSWORD")
        .unwrap_or_else(|e| panic!("SURREALDB_PASSWORD is not set: {e}"));
    let ns = std::env::var("SURREALDB_NAMESPACE")
        .unwrap_or_else(|e| panic!("SURREALDB_NAMESPACE is not set: {e}"));
    let db = std::env::var("SURREALDB_NAME")
        .unwrap_or_else(|e| panic!("SURREALDB_NAME is not set: {e}"));

    let app_host = std::env::var("APP_HOST").unwrap_or_else(|e| panic!("APP_HOST is not set: {e}"));
    let app_port: u16 = std::env::var("APP_PORT")
        .unwrap_or_else(|e| panic!("APP_PORT is not set: {e}"))
        .parse()
        .expect("APP_PORT must be a valid port number");

    let context = DatabaseContext::new(host, username, password, ns, db).await?;

    let mut mediatr = Mediatr::default();

    // organization
    mediatr.register_command(CreateOrganizationCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_query(GetOrganizationQueryHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(UpdateOrganizationCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(DeleteOrganizationCommandHandler {
        db: context.db().clone(),
    });
    // project
    mediatr.register_command(CreateProjectCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_query(GetProjectQueryHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(UpdateProjectCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(DeleteProjectCommandHandler {
        db: context.db().clone(),
    });
    // task
    mediatr.register_command(CreateTaskCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_query(GetTaskQueryHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(UpdateTaskCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(DeleteTaskCommandHandler {
        db: context.db().clone(),
    });
    // team
    mediatr.register_command(CreateTeamCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_query(GetTeamQueryHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(UpdateTeamCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(DeleteTeamCommandHandler {
        db: context.db().clone(),
    });
    // user
    mediatr.register_command(CreateUserCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_query(GetUserQueryHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(UpdateUserCommandHandler {
        db: context.db().clone(),
    });
    mediatr.register_command(DeleteUserCommandHandler {
        db: context.db().clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mediatr.clone()))
            .service(
                web::scope("/api")
                    // organization
                    .service(create_organization)
                    .service(get_organization)
                    .service(update_organization)
                    .service(delete_organization)
                    // project
                    .service(create_project)
                    .service(get_project)
                    .service(update_project)
                    .service(delete_project)
                    // task
                    .service(create_task)
                    .service(get_task)
                    .service(update_task)
                    .service(delete_task)
                    // team
                    .service(create_team)
                    .service(get_team)
                    .service(update_team)
                    .service(delete_team)
                    // user
                    .service(create_user)
                    .service(get_user)
                    .service(update_user)
                    .service(delete_user),
            )
    })
    .bind((app_host.as_str(), app_port))?
    .run()
    .await?;

    Ok(())
}
