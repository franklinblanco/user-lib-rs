use crate::domain::credential::Credential;
use crate::dto::credential::CredentialDto;
use chrono::Utc;
use sqlx::{Error, PgPool};

pub async fn insert_credentials(
    conn: &PgPool,
    credentials: Vec<CredentialDto>,
    user_id: &i32,
) -> Result<Vec<Credential>, Error> {
    let insert_query_base = r#"INSERT INTO credential
    (user_id, credential_type, credential, validated, time_created, last_updated)
    VALUES ($1, $2, $3, $4, $5, $5) RETURNING user_id, credential_type as "credential_type: _", credential, validated, time_created, last_updated"#;
    let mut persisted_credentials = Vec::new();
    for credential_dto in credentials {
        let persisted_credential: Credential = sqlx::query_as(insert_query_base)
            .bind(user_id)
            .bind(credential_dto.credential_type)
            .bind(credential_dto.credential)
            .bind(false)
            .bind(Utc::now())
            .fetch_one(conn)
            .await?;
        persisted_credentials.push(persisted_credential);
    }
    Ok(persisted_credentials)
}

pub async fn fetch_user_credentials(
    conn: &PgPool,
    user_id: &i32,
) -> Result<Vec<Credential>, Error> {
    sqlx::query_as(r#"SELECT user_id, credential_type as "credential_type: _", credential, validated, time_created, last_updated FROM credential WHERE user_id = $1 "#).bind(user_id).fetch_all(conn).await
}
