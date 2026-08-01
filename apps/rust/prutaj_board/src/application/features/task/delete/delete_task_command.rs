use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Task;

#[derive(Deserialize, ToSchema)]
pub struct DeleteTaskCommand {
    pub id: String,
}

impl Command for DeleteTaskCommand {
    type Response = ();
}

pub struct DeleteTaskCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<DeleteTaskCommand> for DeleteTaskCommandHandler {
    async fn handle(&self, command: DeleteTaskCommand) -> Result<(), MediatrError> {
        let task = Task::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        task.delete(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(())
    }
}
