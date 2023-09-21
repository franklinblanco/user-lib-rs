use crate::domain::token::Token;
use chrono::Utc;
use sqlx::{Error, PgPool};

pub async fn insert_token(conn: &PgPool, token: Token) -> Result<Token, Error> {
    sqlx::query_as(r#"INSERT INTO token (
    user_id, auth_token, refresh_token, time_created, last_updated) VALUES ($1, $2, $3, $4, $4) RETURNING *;"#)
        .bind(token.user_id).bind(token.auth_token).bind(token.refresh_token).bind(token.time_created)
        .fetch_one(conn).await
}

pub async fn update_token(
    conn: &PgPool,
    token_id: &i32,
    refresh_token: String,
    new_auth_token: String,
) -> Result<Token, Error> {
    sqlx::query_as(
        r#"UPDATE token set
    auth_token = $3, last_updated = $4
    WHERE id = $1 AND refresh_token = $2 RETURNING *;"#,
    )
    .bind(token_id)
    .bind(refresh_token)
    .bind(new_auth_token)
    .bind(Utc::now())
    .fetch_one(conn)
    .await
}

pub async fn remove_token(conn: &PgPool, token_id: &i32) -> Result<Option<Token>, Error> {
    sqlx::query_as(r#"DELETE FROM token WHERE id = $1 RETURNING *;"#)
        .bind(token_id)
        .fetch_optional(conn)
        .await
}

pub async fn validate_user_token(
    conn: &PgPool,
    user_id: &i32,
    auth_token: String,
) -> Result<Option<Token>, Error> {
    sqlx::query_as(r#"SELECT * FROM token where user_id = $1 AND auth_token = $2;"#)
        .bind(user_id)
        .bind(auth_token)
        .fetch_optional(conn)
        .await
}
