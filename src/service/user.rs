use chrono::Utc;
use log::{debug, error, log};
use sqlx::{Error, Postgres, Transaction};
use sqlx::pool::PoolConnection;
use crate::dao::credential::{get_credential, insert_credential};
use crate::dao::token::{insert_token, validate_user_token};
use crate::dao::user::{get_user_with_id, insert_user};
use crate::domain::token::Token;
use crate::domain::user::User;
use crate::dto::token::AuthenticateUserDto;
use crate::dto::users::UserRegisterPayload;
use crate::resources::error_messages::{ERROR_EXPIRED_TOKEN, ERROR_INCORRECT_TOKEN, ERROR_TOKEN_NOT_CREATED, ERROR_TOO_MANY_CREDENTIALS, ERROR_USER_ALREADY_EXISTS, ERROR_USER_DOES_NOT_EXIST, ErrorResource};
use crate::resources::expirations::AUTH_TOKEN_EXPIRATION_TIME_MILLIS;
use crate::utils::hasher::{generate_multiple_random_token_with_rng, hash_password};
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
                error!("1{}", e);
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
            error!("2{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }};

    // Insert Credentials
    for credential in user.credentials {
        match insert_credential(transaction, credential, &persisted_user.id).await {
            Ok(_) => {}
            Err(e) => {
                error!("3{}", e);
                error_resources.push(("ERROR.DATABASE_ERROR", ""));
                return Err(error_resources);
            }
        };
    }
    //  Create token and send it back.
    let tokens: Vec<String> = match generate_multiple_random_token_with_rng(2).await {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("4{}", e);
            error_resources.push(("ERROR.JOIN_ERROR", ""));
            return Err(error_resources);
        }
    };
    let token_to_insert =
        Token {
            id: 0,
            user_id: persisted_user.id,
            auth_token: match tokens.get(0) {
                None => {
                    error!("Tokens were not created.", );
                    error_resources.push(ERROR_TOKEN_NOT_CREATED);
                    return Err(error_resources);
                }
                Some(token) => token.clone()
            },
            refresh_token: match tokens.get(1) {
                None => {
                    error!("Tokens were not created.", );
                    error_resources.push(ERROR_TOKEN_NOT_CREATED);
                    return Err(error_resources);
                }
                Some(token) => token.clone()
            },
            time_created: now,
            last_updated: now,
        };

    //  Insert token in DB
    match insert_token(transaction, token_to_insert).await {
        Ok(persisted_token) => {
            Ok(persisted_token)
        },
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            Err(error_resources)
        }
    }
}
pub async fn authenticate_user(db_conn: &sqlx::PgPool, user: AuthenticateUserDto) -> Result<User, Vec<ErrorResource>> {
    let mut error_resources = Vec::new();
    let mut conn = match db_conn.acquire().await {
        Ok(conn) => conn,
        Err(error) => {
            error!("{:?}", error);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }
    };
    let persisted_user = match get_user_with_id(&mut conn, &user.id).await {
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

    match validate_user_token(&mut conn, &user.id, user.auth_token).await {
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

pub async fn refresh_auth_token() {}

pub async fn reset_password() {}

pub async fn password_login() {}