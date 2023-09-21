use crate::domain::credential::CredentialType;
use crate::dto::credential::CredentialDto;
use serde::{Deserialize, Serialize};

/// Used for logging in when you don't have a token.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginPayload {
    pub credential: String,
    pub credential_type: CredentialType,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterPayload {
    pub credentials: Vec<CredentialDto>,
    pub password: String,
    pub name: String,
}
