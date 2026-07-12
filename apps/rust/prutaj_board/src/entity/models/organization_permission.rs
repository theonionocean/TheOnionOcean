use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use ulid::Ulid;

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationPermission {
    pub id: RecordId,
    pub name: String,
    pub description: Option<String>,
}

impl OrganizationPermission {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: RecordId::new("organization_permission", Ulid::new().to_string()),
            name,
            description,
        }
    }
}
