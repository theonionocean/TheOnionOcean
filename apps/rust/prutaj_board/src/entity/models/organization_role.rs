use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Organization, OrganizationPermission, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationRole {
    pub id: RecordId,
    pub created_by: User,
    pub description: Option<String>,
    pub kind: String,
    pub name: String,
    pub organization: Organization,
    pub permissions: Vec<OrganizationPermission>,
    pub sort_order: i32,
}
