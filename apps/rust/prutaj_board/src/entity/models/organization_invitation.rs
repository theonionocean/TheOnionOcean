use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use crate::{OrganizationRole, Team, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationInvitation {
    pub id: RecordId,
    pub organization_role: OrganizationRole,
    pub email: String,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub teams: Vec<Team>,
    pub used_at: Option<DateTime<Utc>>,
    pub used_by: Option<User>,
    pub created_by: User,
}
