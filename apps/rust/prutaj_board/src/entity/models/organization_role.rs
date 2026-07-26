use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Organization, OrganizationPermission, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationRole {
    pub id: Option<RecordId>,
    pub created_by: User,
    pub description: Option<String>,
    pub kind: String,
    pub name: String,
    pub organization: Organization,
    pub permissions: Vec<OrganizationPermission>,
    pub sort_order: i32,
}

impl OrganizationRole {
    pub fn new(
        name: String,
        organization: Organization,
        created_by: User,
        description: Option<String>,
        kind: String,
        permissions: Vec<OrganizationPermission>,
        sort_order: i32,
    ) -> Self {
        Self {
            id: None,
            name,
            organization,
            created_by,
            description,
            kind,
            permissions,
            sort_order,
        }
    }
}
