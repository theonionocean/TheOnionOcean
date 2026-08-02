use crate::entity::Team;
use mediatr::{MediatrError, Query, QueryHandler};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct GetTeamQuery {
    pub id: String,
}

impl Query for GetTeamQuery {
    type Response = Team;
}

pub struct GetTeamQueryHandler {
    pub db: Surreal<Client>,
}

impl QueryHandler<GetTeamQuery> for GetTeamQueryHandler {
    async fn handle(&self, query: GetTeamQuery) -> Result<Team, MediatrError> {
        let team = Team::find(&self.db, &query.id.as_str())
            .await
            .map_err(|e| MediatrError::HandlerFailed(e.to_string()))?;

        Ok(team)
    }
}
