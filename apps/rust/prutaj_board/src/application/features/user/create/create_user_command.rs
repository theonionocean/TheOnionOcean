use mediatr::{Command, CommandHandler, MediatrError};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::entity::User;

#[derive(Deserialize)]
pub struct CreateUserCommand {
    pub name: String,
    pub email: String,
}

impl Command for CreateUserCommand {
    type Response = User;
}

pub struct CreateUserCommandHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    async fn handle(&self, command: CreateUserCommand) -> Result<User, MediatrError> {
        let user = User::new(command.name, command.email)
            .create(&self.db)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(user)
    }
}
