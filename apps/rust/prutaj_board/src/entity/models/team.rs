use chrono::{DateTime, Utc};
use macros::AuditibleEntity;
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

#[derive(SurrealValue, Serialize, Deserialize, Debug, Clone, ToSchema, AuditibleEntity)]
pub struct Team {
    #[schema(value_type = Option<String>)]
    pub id: Option<RecordId>,
    pub name: String,
    #[schema(no_recursion)]
    pub organization: Organization,
    #[schema(no_recursion)]
    pub parent: Box<Option<Team>>,
    pub projects: Vec<Project>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_by: String,
    pub modified_at: DateTime<Utc>,
}

impl Team {
    pub fn new(name: String, organization: Organization) -> Self {
        Self {
            name,
            organization,
            ..Default::default()
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

    pub async fn update(
        self,
        db: &Surreal<Client>,
        id: &str,
        content: Team,
    ) -> Result<Team, SurrealDbError> {
        let result = content
            .clone()
            .update_record(db, TABLE_NAME, id, content)
            .await?;
        Ok(result)
    }

    pub async fn delete(self, db: &Surreal<Client>, id: &str) -> Result<(), SurrealDbError> {
        let result = self.delete_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }
}
