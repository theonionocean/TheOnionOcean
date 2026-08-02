use mediatr::{MediatrError, Query, QueryHandler};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

use crate::entity::Organization;

#[derive(Deserialize, ToSchema)]
pub struct GetOrganizationQuery {
    pub id: String,
}

impl Query for GetOrganizationQuery {
    type Response = Organization;
}

pub struct GetOrganizationQueryHandler {
    pub db: Surreal<Client>,
}

impl QueryHandler<GetOrganizationQuery> for GetOrganizationQueryHandler {
    async fn handle(&self, query: GetOrganizationQuery) -> Result<Organization, MediatrError> {
        let organization = Organization::find(&self.db, &query.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(organization)
    }
}
