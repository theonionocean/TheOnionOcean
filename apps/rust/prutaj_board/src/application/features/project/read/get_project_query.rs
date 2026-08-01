use mediatr::{MediatrError, Query, QueryHandler};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Project;

#[derive(Deserialize, ToSchema)]
pub struct GetProjectQuery {
    pub id: String,
}

impl Query for GetProjectQuery {
    type Response = Project;
}

pub struct GetProjectQueryHandler {
    pub db: Surreal<Client>,
}

impl QueryHandler<GetProjectQuery> for GetProjectQueryHandler {
    async fn handle(&self, query: GetProjectQuery) -> Result<Project, MediatrError> {
        let project = Project::find(&self.db, &query.id)
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(project)
    }
}
