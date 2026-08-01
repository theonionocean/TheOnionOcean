use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::{entity::Task, Project, User};

#[derive(Deserialize, ToSchema)]
pub struct CreateTaskCommand {
    pub title: String,
    pub description: String,
    pub status: Option<String>,
}

impl Command for CreateTaskCommand {
    type Response = Task;
}

pub struct CreateTaskCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<CreateTaskCommand> for CreateTaskCommandHandler {
    async fn handle(&self, command: CreateTaskCommand) -> Result<Task, MediatrError> {
        let user = User::find(&self.db, "01KYCKGYEPH91ADW1TP5J19AXB")
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        // TODO: Change id to existing project id
        let project = Project::find(&self.db, "01KYCKGYEPH91ADW1TP5J19AXB")
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let task = Task::new(
            command.title,
            command.description,
            command.status,
            None,
            project,
            user.clone(),
            user,
        );

        let task = task
            .create(&self.db)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(task)
    }
}
