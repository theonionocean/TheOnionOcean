use crate::{entity::Team, Organization};
use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UpdateTeamCommand {
    pub id: String,
    pub name: String,
    pub organization: Organization,
}

impl Command for UpdateTeamCommand {
    type Response = Team;
}

pub struct UpdateTeamCommandHandler {
    pub db: Surreal<Client>,
}

// TODO: Add optional parent team and project
impl CommandHandler<UpdateTeamCommand> for UpdateTeamCommandHandler {
    async fn handle(&self, command: UpdateTeamCommand) -> Result<Team, MediatrError> {
        let team = Team::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let updated = team
            .update(&self.db, command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
