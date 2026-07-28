use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::entity::User;

#[derive(Deserialize)]
pub struct DeleteUserCommand {
    pub id: String,
}

impl Command for DeleteUserCommand {
    type Response = ();
}

pub struct DeleteUserCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<DeleteUserCommand> for DeleteUserCommandHandler {
    async fn handle(&self, command: DeleteUserCommand) -> Result<(), MediatrError> {
        let user = User::find(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        user.delete(&self.db, &command.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(())
    }
}
