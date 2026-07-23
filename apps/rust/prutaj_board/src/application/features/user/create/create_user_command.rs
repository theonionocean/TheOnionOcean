use mediatr::{Command, CommandHandler, MediatrError};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::entity::User;

pub struct CreateUser {
    pub name: String,
    pub email: String,
}

impl Command for CreateUser {
    type Response = User;
}

pub struct CreateUserHandler {
    pub db: Surreal<Client>,
}

impl CommandHandler<CreateUser> for CreateUserHandler {
    async fn handle(&self, command: CreateUser) -> Result<User, MediatrError> {
        let user = User::new(command.name, command.email);
        user.create(&self.db).await?;
        Ok(user)
    }
}
