use crate::{entity::Team, Organization};
use chrono::Utc;
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

        let content = Team {
            id: team.id.clone(),
            name: command.name,
            organization: command.organization,
            parent: team.parent.clone(),
            projects: team.projects.clone(),
            created_by: team.created_by.clone(),
            created_at: team.created_at,
            ..Default::default()
        }
        .set_modified("Anonymous");

        let updated = team
            .update(&self.db, command.id.as_str(), content)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
