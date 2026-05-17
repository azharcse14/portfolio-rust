use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_objects::Email;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    pub subject: Option<String>,
    pub body: String,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}
