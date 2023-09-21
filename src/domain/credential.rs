use crate::resources::variable_lengths::{
    MAX_EMAIL_LENGTH, MAX_NAME_LENGTH, MAX_PHONE_NUMBER_LENGTH, MAX_USERNAME_LENGTH,
    MIN_EMAIL_LENGTH, MIN_PHONE_NUMBER_LENGTH, MIN_USERNAME_LENGTH,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
#[derive(
    Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub user_id: i32,
    pub credential_type: CredentialType,
    pub credential: String,
    pub validated: bool,
    pub time_created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl CredentialType {
    pub fn get_max_length(&self) -> usize {
        match self {
            CredentialType::PhoneNumber => MAX_PHONE_NUMBER_LENGTH,
            CredentialType::Email => MAX_EMAIL_LENGTH,
            CredentialType::Username => MAX_USERNAME_LENGTH,
        }
    }
    pub fn get_min_length(&self) -> usize {
        match self {
            CredentialType::PhoneNumber => MIN_PHONE_NUMBER_LENGTH,
            CredentialType::Email => MIN_EMAIL_LENGTH,
            CredentialType::Username => MIN_USERNAME_LENGTH,
        }
    }
}
