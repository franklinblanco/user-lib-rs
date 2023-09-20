use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


/// Is used in the user struct to signal what type of credential will be used in the credential Column.
/// Defaults to email.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CredentialType {
    PhoneNumber,
    #[default]
    Email,
    Username,
}

/// Can only have one per user per cred_type
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub user_id: i32,
    pub credential_type: CredentialType,
    pub credential: String,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}