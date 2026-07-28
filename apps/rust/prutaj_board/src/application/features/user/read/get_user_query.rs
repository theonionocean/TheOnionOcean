use mediatr::{MediatrError, Query, QueryHandler};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::entity::User;

#[derive(Deserialize)]
pub struct GetUserQuery {
    pub id: String,
}

impl Query for GetUserQuery {
    type Response = User;
}

pub struct GetUserQueryHandler {
    pub db: Surreal<Client>,
}

impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
    async fn handle(&self, query: GetUserQuery) -> Result<User, MediatrError> {
        let user = User::find(&self.db, &query.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(user)
    }
}
