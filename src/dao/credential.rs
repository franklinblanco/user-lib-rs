use crate::domain::credential::Credential;
use crate::dto::credential::CredentialDto;
use chrono::Utc;
use sqlx::{Error, PgConnection, PgPool};

pub async fn insert_credential(
    conn: &mut PgConnection,
    credential_dto: CredentialDto,
    user_id: &i32,
) -> Result<Credential, Error> {
    let insert_query_base = r#"INSERT INTO "credential"
    (user_id, credential_type, credential, validated, time_created, last_updated)
    VALUES ($1, $2, $3, $4, $5, $5) RETURNING user_id, credential_type, credential, validated, time_created, last_updated"#;
    sqlx::query_as(insert_query_base)
        .bind(user_id)
        .bind(credential_dto.credential_type)
        .bind(credential_dto.credential)
        .bind(false)
        .bind(Utc::now())
        .fetch_one(conn)
        .await
}

pub async fn fetch_user_credentials(
    conn: &mut PgConnection,
    user_id: &i32,
) -> Result<Vec<Credential>, Error> {
    sqlx::query_as(r#"SELECT user_id, credential_type, credential, validated, time_created, last_updated FROM "credential" WHERE user_id = $1 "#).bind(user_id).fetch_all(conn).await
}

pub async fn get_credential(
    conn: &mut PgConnection,
    credential: String,
) -> Result<Option<Credential>, Error> {
    sqlx::query_as(r#"SELECT user_id, credential_type, credential, validated, time_created, last_updated FROM "credential" WHERE credential = $1"#).bind(credential).fetch_optional(conn).await
}
