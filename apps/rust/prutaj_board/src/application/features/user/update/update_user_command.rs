use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::User;

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserCommand {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Command for UpdateUserCommand {
    type Response = User;
}

pub struct UpdateUserCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<UpdateUserCommand> for UpdateUserCommandHandler {
    async fn handle(&self, command: UpdateUserCommand) -> Result<User, MediatrError> {
        let user = User::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        let updated = user
            .update(&self.db, command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(updated)
    }
}
