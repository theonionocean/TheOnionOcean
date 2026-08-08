use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Task;

#[derive(Deserialize, ToSchema)]
pub struct UpdateTaskCommand {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: Option<String>,
}

impl Command for UpdateTaskCommand {
    type Response = Task;
}

pub struct UpdateTaskCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<UpdateTaskCommand> for UpdateTaskCommandHandler {
    async fn handle(&self, command: UpdateTaskCommand) -> Result<Task, MediatrError> {
        let task = Task::find(&self.db, &command.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let content = Task {
            id: task.id.clone(),
            title: command.title,
            description: command.description,
            status: command.status,
            assigned_to: task.assigned_to.clone(),
            project: task.project.clone(),
            created_by: task.created_by.clone(),
            created_at: task.created_at,
            ..Default::default()
        }
        .set_modified("Anonymous");

        let updated = task
            .update(&self.db, &command.id, content)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
