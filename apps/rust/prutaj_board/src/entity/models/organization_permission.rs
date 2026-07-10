use surrealdb::types::{RecordId, SurrealValue};

use serde::{Deserialize, Serialize};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationPermission {
    pub id: RecordId,
    pub name: String,
    pub description: Option<String>,
}
