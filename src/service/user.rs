use chrono::Utc;
use log::{debug, error};
use sqlx::{PgConnection, Postgres, Transaction};
use crate::dao::credential::{fetch_user_credentials, get_credential, insert_credential};
use crate::dao::token::{insert_token, validate_user_token};
use crate::dao::user::{get_user_with_id, insert_user};
use crate::domain::credential::Credential;
use crate::domain::token::Token;
use crate::domain::user::User;
use crate::dto::token::AuthenticateUserDto;
use crate::dto::users::{UserLoginPayload, UserRegisterPayload};
use crate::resources::error_messages::{ERROR_CREDENTIAL_DOES_NOT_EXIST, ERROR_EXPIRED_TOKEN, ERROR_INCORRECT_TOKEN, ERROR_TOKEN_NOT_CREATED, ERROR_TOO_MANY_CREDENTIALS, ERROR_USER_ALREADY_EXISTS, ERROR_USER_DOES_NOT_EXIST, ErrorResource};
use crate::resources::expirations::AUTH_TOKEN_EXPIRATION_TIME_MILLIS;
use crate::utils::hasher::{generate_multiple_random_token_with_rng, hash_password, hash_password_with_existing_salt};
use crate::validation::user_validator::validate_user_for_creation;

pub async fn register_user<'a>(transaction: &mut Transaction<'a, Postgres>, user: UserRegisterPayload) -> Result<Token, Vec<ErrorResource<'a>>> {
    let mut error_resources: Vec<ErrorResource> = Vec::new();
    //  Validate user
    validate_user_for_creation(&user, &mut error_resources);
    //  Find if user exists
    if user.credentials.len() > 3 {
        error_resources.push(ERROR_TOO_MANY_CREDENTIALS);
    }
    for credential_dto in user.credentials.iter() {
        match get_credential(
            transaction,
            credential_dto.credential.clone(),
        )
            .await
        {
            Ok(credential_opt) => {
                match credential_opt {
                    None => {}
                    Some(_) => {
                        error_resources.push(
                        ERROR_USER_ALREADY_EXISTS);
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
                error_resources.push(("ERROR.DATABASE_ERROR", ""));
            }
        };
    }
    //  If validation gave any errors blow up and send them back to the client
    if error_resources.len() > 0 {
        return Err(error_resources);
    }
    //  Get salt and hashed password from hashing function then give the results to the user
    let hash_result = hash_password(&user.password);
    let now = Utc::now();
    let user_to_insert = User {
        id: 0,
        name: user.name,
        password: hash_result.hash,
        salt: hash_result.salt,
        time_created: now,
        last_updated: now,
    };

    let persisted_user;
    //  Insert user in DB
    match insert_user(transaction, user_to_insert).await{
        Ok(user) => {
            persisted_user = user;
        },
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }};

    // Insert Credentials
    for credential in user.credentials {
        match insert_credential(transaction, credential, &persisted_user.id).await {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e);
                error_resources.push(("ERROR.DATABASE_ERROR", ""));
                return Err(error_resources);
            }
        };
    }

    if let Some(persisted_token) = create_token_for_user(transaction, persisted_user.id, &mut error_resources).await {
        Ok(persisted_token)
    } else {
        Err(error_resources)
    }
}
pub async fn authenticate_user<'a>(conn: &mut PgConnection, user: AuthenticateUserDto) -> Result<User, Vec<ErrorResource<'a>>> {
    let mut error_resources = Vec::new();
    let persisted_user = match get_user_with_id(conn, &user.id).await {
        Ok(persisted_user_opt) => match persisted_user_opt {
            None => {
                error_resources.push(ERROR_USER_DOES_NOT_EXIST);
                return Err(error_resources);
            },
            Some(persisted_user) => persisted_user
        },
        Err(error) => {
            error!("{:?}", error);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }
    };

    match validate_user_token(conn, &user.id, user.auth_token).await {
        Ok(persisted_token_opt) => match persisted_token_opt {
            None => {
                error_resources.push(ERROR_INCORRECT_TOKEN);
                Err(error_resources)
            },
            Some(persisted_token) => {
                // Check if persisted_token expired
                if Utc::now().timestamp_millis() - persisted_token.last_updated.timestamp_millis() > AUTH_TOKEN_EXPIRATION_TIME_MILLIS {
                    // Expired
                    debug!("Expired token: {:?}", persisted_token);
                    error_resources.push(ERROR_EXPIRED_TOKEN);
                    Err(error_resources)
                } else {
                    // Not expired
                    Ok(persisted_user)
                }
            }
        },
        Err(error) => {
            error!("{:?}", error);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            Err(error_resources)
        }
    }
}

///
pub async fn refresh_auth_token() {}

///
pub async fn reset_password() {}

/// ## This resets a user's password without any validations!
/// Don't expose this to any public endpoint!!
pub async fn force_reset_password() {}

///
pub async fn password_login<'a>(transaction: &mut Transaction<'a, Postgres>, user: UserLoginPayload) -> Result<Token, Vec<ErrorResource<'a>>>{
    let mut error_resources = Vec::new();
    let persisted_user_credential = match get_credential(transaction, user.credential).await {
        Ok(credential_opt) => match credential_opt {
            None => {
                error!("Credential not found for password login.");
                error_resources.push(ERROR_CREDENTIAL_DOES_NOT_EXIST);
                return Err(error_resources);
            },
            Some(persisted_credential) => persisted_credential
        },
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }
    };
    let persisted_user = match get_user_with_id(transaction, &persisted_user_credential.user_id).await {
        Ok(persisted_user_opt) => match persisted_user_opt {
            None => {
                error!("Serious error. User doesn't exist but credentials pointing to the user do.");
                error_resources.push(("ERROR.DATABASE_ERROR", "Critical. User doesn't exist but credentials pointing to the user do."));
                return Err(error_resources);
            },
            Some(persisted_user) => {
                persisted_user
            }
        },
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);}
    };
    let hashed_password = hash_password_with_existing_salt(&user.password, &persisted_user.salt);
    if hashed_password.hash == persisted_user.password {
        return if let Some(persisted_token) = create_token_for_user(transaction, persisted_user.id, &mut error_resources).await {
            Ok(persisted_token)
        } else {
            Err(error_resources)
        }
    }

    Err(error_resources)
}

///
pub async fn get_user_credentials<'a>(transaction: &mut Transaction<'a, Postgres>, user: AuthenticateUserDto) -> Result<Vec<Credential>, Vec<ErrorResource<'a>>> {
    let mut error_resources = Vec::new();
    let persisted_user = authenticate_user(transaction, user).await?;
    match fetch_user_credentials(transaction, &persisted_user.id).await {
        Ok(persisted_credentials) => Ok(persisted_credentials),
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            Err(error_resources)
        }
    }
}

async fn create_token_for_user<'a>(transaction: &mut Transaction<'a, Postgres>, user_id: i32, error_resources: &mut Vec<ErrorResource<'a>>) -> Option<Token> {
    //  Create token and send it back.
    let tokens: Vec<String> = match generate_multiple_random_token_with_rng(2).await {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.JOIN_ERROR", ""));
            return None;
        }
    };
    let token_to_insert =
        Token {
            id: 0,
            user_id,
            auth_token: match tokens.get(0) {
                None => {
                    error!("Tokens were not created.", );
                    error_resources.push(ERROR_TOKEN_NOT_CREATED);
                    return None;
                }
                Some(token) => token.clone()
            },
            refresh_token: match tokens.get(1) {
                None => {
                    error!("Tokens were not created.", );
                    error_resources.push(ERROR_TOKEN_NOT_CREATED);
                    return None;
                }
                Some(token) => token.clone()
            },
            time_created: Utc::now(),
            last_updated: Utc::now(),
        };

    //  Insert token in DB
    match insert_token(transaction, token_to_insert).await {
        Ok(persisted_token) => {
            Some(persisted_token)
        },
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            None
        }
    }
}