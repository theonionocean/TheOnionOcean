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

use crate::{Team, User};

const TABLE_NAME: &str = "organization";

#[derive(SurrealValue, Serialize, Deserialize, Debug, Clone, ToSchema, AuditibleEntity)]
pub struct Organization {
    #[schema(value_type = Option<String>)]
    pub id: Option<RecordId>,
    pub name: String,
    pub legal_name: Option<String>,
    pub logo: Option<String>,
    pub owner: User,
    pub slug: String,
    #[schema(no_recursion)]
    pub teams: Vec<Team>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_by: String,
    pub modified_at: DateTime<Utc>,
}

impl Organization {
    pub fn new(
        name: String,
        legal_name: Option<String>,
        logo: Option<String>,
        owner: User,
        slug: String,
    ) -> Self {
        Self {
            name,
            legal_name,
            logo,
            owner,
            slug,
            ..Default::default()
        }
    }

    pub async fn create(self, db: &Surreal<Client>) -> Result<Organization, SurrealDbError> {
        let result = self.create_record(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn find(db: &Surreal<Client>, id: &str) -> Result<Organization, SurrealDbError> {
        let result = Organization::read_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }

    pub async fn update(
        self,
        db: &Surreal<Client>,
        id: &str,
        content: Organization,
    ) -> Result<Organization, SurrealDbError> {
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
