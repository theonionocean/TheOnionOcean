use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{entity::Project, User};

#[derive(Deserialize)]
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
        let user = User::find(&self.db, "01KYCKGYEPH91ADW1TP5J19AXB")
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let project = Project::new(
            command.name,
            command.description,
            command.slug,
            user.clone(),
            user,
        );

        let project = project
            .create(&self.db)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(project)
    }
}
