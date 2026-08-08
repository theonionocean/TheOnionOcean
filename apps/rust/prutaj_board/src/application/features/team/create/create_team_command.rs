use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::{entity::Team, Organization};

#[derive(Deserialize, ToSchema)]
pub struct CreateTeamCommand {
    pub name: String,
    pub organization: Organization,
}

impl Command for CreateTeamCommand {
    type Response = Team;
}

pub struct CreateTeamCommandHandler {
    pub db: Surreal<Client>,
}

// TODO: Add optional parent team and project
impl CommandHandler<CreateTeamCommand> for CreateTeamCommandHandler {
    async fn handle(&self, command: CreateTeamCommand) -> Result<Team, MediatrError> {
        let team = Team::new(command.name, command.organization)
            .set_created("Anonymous")
            .create(&self.db)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(team)
    }
}
