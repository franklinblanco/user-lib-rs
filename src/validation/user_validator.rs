use crate::domain::credential::CredentialType;
use crate::dto::users::{UserLoginPayload, UserRegisterPayload};
use crate::resources::error_messages::{ErrorResource, ERROR_INVALID_USERNAME};
use crate::resources::{
    error_messages::{
        ERROR_INVALID_EMAIL, ERROR_INVALID_NAME, ERROR_INVALID_PASSWORD, ERROR_INVALID_PHONE_NUMBER,
    },
    variable_lengths::{
        MAX_EMAIL_LENGTH, MAX_NAME_LENGTH, MAX_PASSWORD_LENGTH, MIN_EMAIL_LENGTH, MIN_NAME_LENGTH,
        MIN_PASSWORD_LENGTH,
    },
};

fn validate_user_email(email: &String) -> bool {
    email.len() >= MIN_EMAIL_LENGTH.into()
        && email.len() <= MAX_EMAIL_LENGTH.into()
        && email.contains('@')
        && email.contains('.')
}
fn validate_user_phone_number(phone_number: &String) -> bool {
    phone_number.len() <= CredentialType::get_max_length(&CredentialType::PhoneNumber)
        && phone_number.len() >= CredentialType::get_min_length(&CredentialType::PhoneNumber)
}

fn validate_user_username(username: &String) -> bool {
    username.len() >= CredentialType::get_max_length(&CredentialType::Username)
        && username.len() <= CredentialType::get_min_length(&CredentialType::Username)
}
fn validate_user_name(name: &String) -> bool {
    name.len() >= MIN_NAME_LENGTH.into() && name.len() <= MAX_NAME_LENGTH.into()
}
fn validate_user_password(password: &String) -> bool {
    password.len() >= MIN_PASSWORD_LENGTH.into() && password.len() <= MAX_PASSWORD_LENGTH.into()
}

pub(crate) fn validate_user_for_creation(
    user: &UserRegisterPayload,
    error_resources: &mut Vec<ErrorResource>,
) {
    for credential_dto in user.credentials.iter() {
        validate_credential(error_resources, &credential_dto.credential, &credential_dto.credential_type);
    }

    if !validate_user_name(&user.name) {
        error_resources.push(ERROR_INVALID_NAME);
    }
    if !validate_user_password(&user.password) {
        error_resources.push(ERROR_INVALID_PASSWORD);
    }
}
pub(crate) fn validate_user_for_password_authentication(
    user: &UserLoginPayload,
    error_resources: &mut Vec<ErrorResource>,
) {
    validate_credential(error_resources, &user.credential, &user.credential_type);
    if !validate_user_password(&user.password) {
        error_resources.push(ERROR_INVALID_PASSWORD);
    }
}

fn validate_credential(error_resources: &mut Vec<ErrorResource>, credential: &String, credential_type: &CredentialType) {
    match credential_type {
        CredentialType::Email => {
            if !validate_user_email(credential) {
                error_resources.push(ERROR_INVALID_EMAIL);
            }
        }
        CredentialType::PhoneNumber => {
            if !validate_user_phone_number(credential) {
                error_resources.push(ERROR_INVALID_PHONE_NUMBER);
            }
        }
        CredentialType::Username => {
            if !validate_user_username(credential) {
                error_resources.push(ERROR_INVALID_USERNAME);
            }
        }
    }
}