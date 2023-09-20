use serde::{Deserialize, Serialize};
use crate::domain::credential::CredentialType;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDto {
    pub credential: String,
    pub credential_type: CredentialType,
}