use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    types::{RecordId, SurrealValue},
    Surreal,
};
use utoipa::ToSchema;

use crate::{Project, User};
use surrealdb_extensions::{
    CreateRecord, DeleteRecord, ReadAllRecords, ReadRecord, SurrealDbError, UpdateRecord,
};

const TABLE_NAME: &str = "task";

#[derive(SurrealValue, Serialize, Deserialize, Clone, ToSchema)]
pub struct Task {
    #[schema(value_type = Option<String>)]
    pub id: Option<RecordId>,
    pub title: String,
    pub description: String,
    pub status: Option<String>,
    pub assigned_to: Option<User>,
    pub project: Project,
    pub created_by: User,
    pub modified_by: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        status: Option<String>,
        assigned_to: Option<User>,
        project: Project,
        created_by: User,
        modified_by: User,
    ) -> Self {
        Self {
            id: None,
            title,
            description,
            status,
            assigned_to,
            project,
            created_by,
            modified_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub async fn create(self, db: &Surreal<Client>) -> Result<Task, SurrealDbError> {
        let result = self.create_record(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn find(db: &Surreal<Client>, id: &str) -> Result<Task, SurrealDbError> {
        let result = Task::read_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }

    // TODO: Find all tasks by project id
    pub async fn find_all(db: &Surreal<Client>) -> Result<Vec<Task>, SurrealDbError> {
        let result = Task::read_all_records(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn update(
        self,
        db: &Surreal<Client>,
        id: &str,
        content: Task,
    ) -> Result<Task, SurrealDbError> {
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
