use std::error::Error;
use chrono::Utc;
use log::{error, log};
use tokio::task::JoinError;
use crate::dao::credential::{get_credential, insert_credentials};
use crate::dao::token::insert_token;
use crate::dao::user::insert_user;
use crate::domain::credential::Credential;
use crate::domain::token::Token;
use crate::domain::user::User;
use crate::dto::users::UserRegisterPayload;
use crate::resources::error_messages::{ERROR_TOKEN_NOT_CREATED, ERROR_TOO_MANY_CREDENTIALS, ERROR_USER_ALREADY_EXISTS, ErrorResource};
use crate::utils::hasher::{generate_multiple_random_token_with_rng, hash_password};
use crate::validation::user_validator::validate_user_for_creation;

pub async fn register_user(db_conn: &sqlx::PgPool, user: UserRegisterPayload) -> Result<Token, Vec<ErrorResource>> {
    let mut error_resources: Vec<ErrorResource> = Vec::new();
    //  Validate user
    validate_user_for_creation(&user, &mut error_resources);
    //  Find if user exists
    if user.credentials.len() > 3 {
        error_resources.push(ERROR_TOO_MANY_CREDENTIALS);
    }
    for credential_dto in user.credentials.iter() {
        match get_credential(
            &db_conn,
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
    match insert_user(&db_conn, user_to_insert).await{
        Ok(user) => {
            persisted_user = user;
        },
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }};

    // Insert Credentials
    match insert_credentials(db_conn, user.credentials, &persisted_user.id).await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            error_resources.push(("ERROR.DATABASE_ERROR", ""));
            return Err(error_resources);
        }
    };

    //  Create token and send it back.
    let tokens: Vec<String> = match generate_multiple_random_token_with_rng(2).await {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("{}", e);
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
    match insert_token(&db_conn, token_to_insert).await {
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
