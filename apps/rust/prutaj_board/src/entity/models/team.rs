use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    types::{RecordId, SurrealValue},
    Surreal,
};
use surrealdb_extensions::{CreateRecord, DeleteRecord, ReadRecord, SurrealDbError, UpdateRecord};
use utoipa::ToSchema;

use crate::{Organization, Project};

const TABLE_NAME: &str = "team";

#[derive(SurrealValue, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Team {
    #[schema(value_type = Option<String>)]
    pub id: Option<RecordId>,
    pub name: String,
    #[schema(no_recursion)]
    pub organization: Organization,
    #[schema(no_recursion)]
    pub parent: Box<Option<Team>>,
    pub projects: Vec<Project>,
    pub created_at: DateTime<Utc>,
}

impl Team {
    pub fn new(name: String, organization: Organization) -> Self {
        Self {
            id: None,
            name,
            organization,
            parent: Box::new(None),
            projects: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub async fn create(self, db: &Surreal<Client>) -> Result<Team, SurrealDbError> {
        let result = self.create_record(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn find(db: &Surreal<Client>, id: &str) -> Result<Team, SurrealDbError> {
        let result = Team::read_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }

    pub async fn update(self, db: &Surreal<Client>, id: &str) -> Result<Team, SurrealDbError> {
        let result = self.clone().update_record(db, TABLE_NAME, id, self).await?;
        Ok(result)
    }

    pub async fn delete(self, db: &Surreal<Client>, id: &str) -> Result<(), SurrealDbError> {
        let result = self.delete_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }
}
