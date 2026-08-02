use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Organization;

#[derive(Deserialize, ToSchema)]
pub struct DeleteOrganizationCommand {
    pub id: String,
}

impl Command for DeleteOrganizationCommand {
    type Response = ();
}

pub struct DeleteOrganizationCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<DeleteOrganizationCommand> for DeleteOrganizationCommandHandler {
    async fn handle(&self, command: DeleteOrganizationCommand) -> Result<(), MediatrError> {
        let organization = Organization::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        organization
            .delete(&self.db, command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(())
    }
}
