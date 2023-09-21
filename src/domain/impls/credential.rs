use std::{fmt::Display, str::FromStr};

use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Postgres,
};

use crate::domain::{credential::CredentialType, error::FromStrError};

impl FromStr for CredentialType {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PhoneNumber" => Ok(Self::PhoneNumber),
            "Email" => Ok(Self::Email),
            "Username" => Ok(Self::Username),
            _ => Err(FromStrError),
        }
    }
}
impl Display for CredentialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialType::PhoneNumber => write!(f, "PhoneNumber"),
            CredentialType::Email => write!(f, "Email"),
            CredentialType::Username => write!(f, "Username"),
        }
    }
}

//
// Sqlx implementations so that the CredentialType enum can be inserted & retrieved from the database
//

impl sqlx::Encode<'_, Postgres> for CredentialType {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let binding = self.to_string();
        <&str as sqlx::Encode<Postgres>>::encode(&binding, buf)
    }
}

impl sqlx::Decode<'_, Postgres> for CredentialType {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let column = value.as_str()?;
        match Self::from_str(column) {
            Ok(listing_state) => Ok(listing_state),
            Err(error) => Err(Box::new(error)),
        }
    }
}

impl sqlx::Type<Postgres> for CredentialType {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }

    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        *ty == Self::type_info()
    }
}
