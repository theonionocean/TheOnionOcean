use utoipa::OpenApi;

#[allow(unused_imports)]
use crate::{
    __path_create_project, __path_create_task, __path_create_user, __path_delete_project,
    __path_delete_task, __path_delete_user, __path_get_project, __path_get_task, __path_get_user,
    __path_update_project, __path_update_task, __path_update_user, CreateProjectCommand,
    CreateTaskCommand, CreateUserCommand, DeleteProjectCommand, DeleteTaskCommand,
    DeleteUserCommand, Project, Task, UpdateProjectCommand, UpdateTaskCommand, UpdateUserCommand,
    User,
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
    )),
    tags(
        (name = "project", description = "Project management"),
        (name = "task", description = "Task management"),
        (name = "user", description = "User management"),
    )
)]
pub struct ApiDoc;
