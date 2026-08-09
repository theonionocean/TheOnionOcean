use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::{entity::Organization, User};

#[derive(Deserialize, ToSchema)]
pub struct CreateOrganizationCommand {
    pub name: String,
    pub legal_name: Option<String>,
    pub logo: Option<String>,
    pub owner: User,
    pub slug: String,
}

impl Command for CreateOrganizationCommand {
    type Response = Organization;
}

pub struct CreateOrganizationCommandHandler {
    pub db: Surreal<Client>,
}

// TODO: Add optional teams
impl CommandHandler<CreateOrganizationCommand> for CreateOrganizationCommandHandler {
    async fn handle(
        &self,
        command: CreateOrganizationCommand,
    ) -> Result<Organization, MediatrError> {
        let organization = Organization::new(
            command.name,
            command.legal_name,
            command.logo,
            command.owner,
            command.slug,
        )
        .create(&self.db)
        .await
        .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(organization)
    }
}
