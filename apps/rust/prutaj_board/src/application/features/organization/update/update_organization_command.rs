use chrono::Utc;
use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::{entity::Organization, User};

#[derive(Deserialize, ToSchema)]
pub struct UpdateOrganizationCommand {
    pub id: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub logo: Option<String>,
    pub owner: User,
    pub slug: String,
}

impl Command for UpdateOrganizationCommand {
    type Response = Organization;
}

pub struct UpdateOrganizationCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<UpdateOrganizationCommand> for UpdateOrganizationCommandHandler {
    async fn handle(
        &self,
        command: UpdateOrganizationCommand,
    ) -> Result<Organization, MediatrError> {
        let organization = Organization::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let content = Organization {
            id: organization.id.clone(),
            name: command.name,
            legal_name: command.legal_name,
            logo: command.logo,
            owner: command.owner,
            slug: command.slug,
            teams: organization.teams.clone(),
            created_by: organization.created_by.clone(),
            created_at: organization.created_at,
            modified_by: organization.modified_by.clone(),
            modified_at: Utc::now(),
        };

        let updated = organization
            .update(&self.db, command.id.as_str(), content)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
