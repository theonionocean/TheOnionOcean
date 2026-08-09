use chrono::{DateTime, Utc};
use macros_derive::AuditibleEntity;
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue, Serialize, Deserialize, AuditibleEntity)]
pub struct OrganizationPermission {
    pub id: Option<RecordId>,
    pub name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_by: String,
    pub modified_at: DateTime<Utc>,
}

impl OrganizationPermission {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            ..Default::default()
        }
    }
}
