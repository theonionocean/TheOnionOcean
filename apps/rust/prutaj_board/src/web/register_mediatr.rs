use crate::{
    CreateOrganizationCommandHandler, CreateProjectCommandHandler, CreateTaskCommandHandler,
    CreateTeamCommandHandler, CreateUserCommandHandler, DeleteOrganizationCommandHandler,
    DeleteProjectCommandHandler, DeleteTaskCommandHandler, DeleteTeamCommandHandler,
    DeleteUserCommandHandler, GetOrganizationQueryHandler, GetProjectQueryHandler,
    GetTaskQueryHandler, GetTeamQueryHandler, GetUserQueryHandler,
    UpdateOrganizationCommandHandler, UpdateProjectCommandHandler, UpdateTaskCommandHandler,
    UpdateTeamCommandHandler, UpdateUserCommandHandler,
};
use mediatr::Mediatr;
use surrealdb_extensions::DatabaseContext;

pub fn register_endpoints(context: DatabaseContext) -> Mediatr {
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

    return mediatr;
}
