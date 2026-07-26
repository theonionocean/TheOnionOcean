use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationPermission {
    pub id: Option<RecordId>,
    pub name: String,
    pub description: Option<String>,
}

impl OrganizationPermission {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: None,
            name,
            description,
        }
    }
}
