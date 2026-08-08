use chrono::{DateTime, Utc};
use macros::AuditibleEntity;
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Organization, OrganizationPermission};

#[derive(SurrealValue, Serialize, Deserialize, AuditibleEntity)]
pub struct OrganizationRole {
    pub id: Option<RecordId>,
    pub description: Option<String>,
    pub kind: String,
    pub name: String,
    pub organization: Organization,
    pub permissions: Vec<OrganizationPermission>,
    pub sort_order: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_by: String,
    pub modified_at: DateTime<Utc>,
}

impl OrganizationRole {
    pub fn new(
        name: String,
        organization: Organization,
        description: Option<String>,
        kind: String,
        permissions: Vec<OrganizationPermission>,
        sort_order: i32,
    ) -> Self {
        Self {
            id: None,
            name,
            organization,
            description,
            kind,
            permissions,
            sort_order,
            created_by: "Anonymous".to_string(),
            created_at: Utc::now(),
            modified_by: "Anonymous".to_string(),
            modified_at: Utc::now(),
        }
    }
}
