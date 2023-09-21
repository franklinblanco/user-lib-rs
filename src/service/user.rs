use std::error::Error;
use log::{error, log};
use crate::dao::credential::get_credential;
use crate::domain::credential::Credential;
use crate::dto::users::UserRegisterPayload;
use crate::resources::error_messages::{ERROR_TOO_MANY_CREDENTIALS, ERROR_USER_ALREADY_EXISTS, ErrorResource};
use crate::validation::user_validator::validate_user_for_creation;

pub async fn register_user(db_conn: &sqlx::PgPool, user: UserRegisterPayload) -> Result<(), Vec<ErrorResource>> {
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

    /* TODO:
    //  Get salt and hashed password from hashing function then give the results to the user
    let hash_result = hasher::hash_password(&user_to_insert.password);
    user_to_insert.password = hash_result.hash;
    user_to_insert.salt = hash_result.salt;

    //  Insert user in DB
    match insert_user(&db_conn, &user_to_insert).await{
        Ok(resultrs) => {
            user_to_insert.id = resultrs.last_insert_id() as u32;
        },
        Err(error) => {
            println!("Error while inserting user in database from create_user method. Log: {}", error);
            return HttpResponse::InternalServerError().finish();
        }};

    //  Create token and send it back.
    let tokens: Vec<String> = hasher::generate_multiple_random_token_with_rng(2).await.expect("Error creating multiple random tokens.");
    let mut token_to_insert =
        Token::new(user_to_insert.id,
                   tokens.get(0).expect("Error. Token doesn't exist in list.").to_string(),
                   tokens.get(1).expect("Error. Token doesn't exist in list.").to_string()
        );

    //  Insert token in DB
    match insert_token(&db_conn, &token_to_insert).await{
        Ok(resultrs) => {token_to_insert.id = resultrs.last_insert_id() as u32},
        Err(_e) => {return HttpResponse::InternalServerError().finish()}
    }
*/
    Ok(())
}
