use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::entity::Project;

#[derive(Deserialize)]
pub struct DeleteProjectCommand {
    pub id: String,
}

impl Command for DeleteProjectCommand {
    type Response = ();
}

pub struct DeleteProjectCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<DeleteProjectCommand> for DeleteProjectCommandHandler {
    async fn handle(&self, command: DeleteProjectCommand) -> Result<(), MediatrError> {
        let project = Project::find(&self.db, &command.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        project
            .delete(&self.db, &command.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(())
    }
}
