use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub category: Option<String>,
    pub percentage: i32,
    pub color: Option<String>,
    pub display_order: i32,
}
