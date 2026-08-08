use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Project;

#[derive(Deserialize, ToSchema)]
pub struct CreateProjectCommand {
    pub name: String,
    pub description: String,
    pub slug: String,
}

impl Command for CreateProjectCommand {
    type Response = Project;
}

pub struct CreateProjectCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<CreateProjectCommand> for CreateProjectCommandHandler {
    async fn handle(&self, command: CreateProjectCommand) -> Result<Project, MediatrError> {
        let project = Project::new(command.name, command.description, command.slug);

        let project = project
            .set_created("Anonymous")
            .create(&self.db)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(project)
    }
}
