use mediatr::{MediatrError, Query, QueryHandler};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Task;

#[derive(Deserialize, ToSchema)]
pub struct GetTaskQuery {
    pub id: String,
}

impl Query for GetTaskQuery {
    type Response = Task;
}

pub struct GetTaskQueryHandler {
    pub db: Surreal<Client>,
}

impl QueryHandler<GetTaskQuery> for GetTaskQueryHandler {
    async fn handle(&self, query: GetTaskQuery) -> Result<Task, MediatrError> {
        let task = Task::find(&self.db, &query.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(task)
    }
}
