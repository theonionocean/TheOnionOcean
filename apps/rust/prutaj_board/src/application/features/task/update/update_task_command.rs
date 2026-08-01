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

        let updated = task
            .update(&self.db, &command.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
