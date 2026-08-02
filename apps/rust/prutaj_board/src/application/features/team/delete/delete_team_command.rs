use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Team;

#[derive(Deserialize, ToSchema)]
pub struct DeleteTeamCommand {
    pub id: String,
}

impl Command for DeleteTeamCommand {
    type Response = ();
}

pub struct DeleteTeamCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<DeleteTeamCommand> for DeleteTeamCommandHandler {
    async fn handle(&self, command: DeleteTeamCommand) -> Result<(), MediatrError> {
        let team = Team::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        team.delete(&self.db, command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(())
    }
}
