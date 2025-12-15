use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
