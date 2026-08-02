use utoipa::OpenApi;

#[allow(unused_imports)]
use crate::{
    __path_create_organization, __path_create_project, __path_create_task, __path_create_team,
    __path_create_user, __path_delete_organization, __path_delete_project, __path_delete_task,
    __path_delete_team, __path_delete_user, __path_get_organization, __path_get_project,
    __path_get_task, __path_get_team, __path_get_user, __path_update_organization,
    __path_update_project, __path_update_task, __path_update_team, __path_update_user,
    CreateOrganizationCommand, CreateProjectCommand, CreateTaskCommand, CreateTeamCommand,
    CreateUserCommand, DeleteOrganizationCommand, DeleteProjectCommand, DeleteTaskCommand,
    DeleteTeamCommand, DeleteUserCommand, GetOrganizationQuery, GetTeamQuery, Organization,
    Project, Task, Team, UpdateOrganizationCommand, UpdateProjectCommand, UpdateTaskCommand,
    UpdateTeamCommand, UpdateUserCommand, User,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Prutaj Board API",
        description = "REST API for Prutaj Board project, task, and user management",
        version = "0.1.0"
    ),
    paths(
        create_project,
        get_project,
        update_project,
        delete_project,
        create_task,
        get_task,
        update_task,
        delete_task,
        create_user,
        get_user,
        update_user,
        delete_user,
        create_organization,
        get_organization,
        update_organization,
        delete_organization,
        create_team,
        get_team,
        update_team,
        delete_team,
    ),
    components(schemas(
        Project,
        Task,
        User,
        CreateProjectCommand,
        UpdateProjectCommand,
        DeleteProjectCommand,
        CreateTaskCommand,
        UpdateTaskCommand,
        DeleteTaskCommand,
        CreateUserCommand,
        UpdateUserCommand,
        DeleteUserCommand,
        CreateOrganizationCommand,
        UpdateOrganizationCommand,
        DeleteOrganizationCommand,
        CreateTeamCommand,
        UpdateTeamCommand,
        DeleteTeamCommand,
        Organization,
        Team,
    )),
    tags(
        (name = "project", description = "Project management"),
        (name = "task", description = "Task management"),
        (name = "user", description = "User management"),
        (name = "organization", description = "Organization management"),
        (name = "team", description = "Team management"),
    )
)]
pub struct ApiDoc;
