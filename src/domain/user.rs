use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(
    FromRow, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub salt: String,
    #[serde(rename = "timeCreated")]
    pub time_created: DateTime<Utc>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: DateTime<Utc>,
}
