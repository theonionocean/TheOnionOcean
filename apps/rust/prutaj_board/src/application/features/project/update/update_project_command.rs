use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Project;

#[derive(Deserialize, ToSchema)]
pub struct UpdateProjectCommand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slug: String,
}

impl Command for UpdateProjectCommand {
    type Response = Project;
}

pub struct UpdateProjectCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<UpdateProjectCommand> for UpdateProjectCommandHandler {
    async fn handle(&self, command: UpdateProjectCommand) -> Result<Project, MediatrError> {
        let project = Project::find(&self.db, &command.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let updated = project
            .update(&self.db, &command.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
