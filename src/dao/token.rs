use crate::domain::token::Token;
use chrono::Utc;
use sqlx::{Error, PgConnection};

pub(crate) async fn insert_token(conn: &mut PgConnection, token: Token) -> Result<Token, Error> {
    sqlx::query_as(r#"INSERT INTO token (
    user_id, auth_token, refresh_token, time_created, last_updated) VALUES ($1, $2, $3, $4, $4) RETURNING *;"#)
        .bind(token.user_id).bind(token.auth_token).bind(token.refresh_token).bind(token.time_created)
        .fetch_one(conn).await
}

pub(crate) async fn update_token(
    conn: &mut PgConnection,
    refresh_token: String,
    new_auth_token: String,
) -> Result<Token, Error> {
    sqlx::query_as(
        r#"UPDATE token set
    auth_token = $3, last_updated = $4
    WHERE refresh_token = $1 RETURNING *;"#,
    )
    .bind(refresh_token)
    .bind(new_auth_token)
    .bind(Utc::now())
    .fetch_one(conn)
    .await
}

#[allow(unused)]
pub(crate) async fn remove_token(conn: &mut PgConnection, token_id: &i32) -> Result<Option<Token>, Error> {
    sqlx::query_as(r#"DELETE FROM token WHERE id = $1 RETURNING *;"#)
        .bind(token_id)
        .fetch_optional(conn)
        .await
}

pub(crate) async fn validate_user_token(
    conn: &mut PgConnection,
    user_id: &i32,
    auth_token: String,
) -> Result<Option<Token>, Error> {
    sqlx::query_as(r#"SELECT * FROM token where user_id = $1 AND auth_token = $2;"#)
        .bind(user_id)
        .bind(auth_token)
        .fetch_optional(conn)
        .await
}
