use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(
    FromRow, Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Token {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "timeCreated")]
    pub time_created: DateTime<Utc>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: DateTime<Utc>,
}
