use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    types::{RecordId, SurrealValue},
    Surreal,
};
use surrealdb_extensions::{
    CreateRecord, DeleteRecord, ReadAllRecords, ReadRecord, SurrealDbError, UpdateRecord,
};
use utoipa::ToSchema;

use crate::User;

const TABLE_NAME: &str = "project";

#[derive(SurrealValue, Serialize, Deserialize, Clone, ToSchema)]
pub struct Project {
    #[schema(value_type = Option<String>)]
    pub id: Option<RecordId>,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub created_by: User,
    pub modified_by: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        slug: String,
        created_by: User,
        modified_by: User,
    ) -> Self {
        Self {
            id: None,
            name,
            description,
            slug,
            created_by,
            modified_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub async fn create(self, db: &Surreal<Client>) -> Result<Project, SurrealDbError> {
        let result = self.create_record(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn find(db: &Surreal<Client>, id: &str) -> Result<Project, SurrealDbError> {
        let result = Project::read_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }

    // TODO: Find all projects by organization id
    pub async fn find_all(db: &Surreal<Client>) -> Result<Vec<Project>, SurrealDbError> {
        let result = Project::read_all_records(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn update(self, db: &Surreal<Client>, id: &str) -> Result<Project, SurrealDbError> {
        let result = self.clone().update_record(db, TABLE_NAME, id, self).await?;
        Ok(result)
    }

    pub async fn delete(self, db: &Surreal<Client>, id: &str) -> Result<(), SurrealDbError> {
        let result = self.delete_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }
}
