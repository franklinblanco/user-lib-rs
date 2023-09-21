use crate::dto::hash_result::HashResult;
use data_encoding::BASE64;
use ring::{
    digest, pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use std::num::NonZeroU32;
use tokio::task::JoinError;

const SALT_ROUNDS: u32 = 1000;

pub async fn generate_multiple_random_token_with_rng(amount: u8) -> Result<Vec<String>, JoinError> {
    //  Get a new instance of a Random Number Generator
    let rng = SystemRandom::new();

    let mut tokens = Vec::with_capacity(amount.into());

    for _i in 0..amount {
        let cloned_rng = rng.clone();
        let future_token = async move {
            let mut token_arr = [0u8; digest::SHA512_OUTPUT_LEN];
            match cloned_rng.fill(&mut token_arr) {
                Ok(()) => BASE64.encode(&token_arr), //TODO: Remove this panic, make your own error and fix this
                Err(_e) => {
                    panic!("Failed to generate random token for some reason.")
                }
            }
        };
        tokens.push(tokio::spawn(future_token));
    }

    let all_tokens = futures_util::future::join_all(tokens).await;
    let all_tokens_solved: Vec<String> = all_tokens
        .into_iter()
        .map(|result| match result {
            Ok(string) => string,
            Err(_e) => "".to_string(),
        })
        .rev()
        .collect();

    Ok(all_tokens_solved)
}

pub fn hash_password_with_existing_salt(password: &String, input_salt: &String) -> HashResult {
    //  Get output length from a sha512 hash
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(SALT_ROUNDS).unwrap();

    let salt = BASE64.decode(input_salt.as_bytes()).unwrap();

    //  Create empty 64-bit byte array for the hash + salt
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    //  Fills byte array with hashed values
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    //  Return an object containing the salt and the hash
    HashResult::new(BASE64.encode(&salt), BASE64.encode(&pbkdf2_hash))
}

pub fn hash_password(password: &String) -> HashResult {
    //  Get output length from a sha512 hash
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(SALT_ROUNDS).unwrap();
    let rng = SystemRandom::new();

    //  Create empty 64-byte array for the salt
    let mut salt = [0u8; CREDENTIAL_LEN];

    //  Fill array with random-generated salt
    match rng.fill(&mut salt) {
        Ok(()) => {}
        Err(_e) => {
            panic!("Failed to generate random salt for some reason.")
        }
    }

    //  Create empty 64-bit byte array for the hash + salt
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    //  Fills byte array with hashed values
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    //  Return an object containing the salt and the hash
    HashResult::new(BASE64.encode(&salt), BASE64.encode(&pbkdf2_hash))
}
