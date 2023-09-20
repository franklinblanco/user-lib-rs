use chrono::Utc;
use sqlx::{Error, PgPool};
use crate::domain::token::Token;

pub async fn insert_token(conn: &PgPool, token: Token) -> Result<Token, Error> {
    sqlx::query_as(r#"INSERT INTO token (
    user_id, auth_token, refresh_token, time_created, last_updated) VALUES ($1, $2, $3, $4, $4) RETURNING *;"#)
        .bind(token.user_id).bind(token.auth_token).bind(token.refresh_token).bind(token.time_created)
        .fetch_one(conn).await
}

pub async fn update_token(conn: &PgPool, token_id: &i32, auth_token: String) -> Result<Token, Error> {
    sqlx::query_as(r#"UPDATE token set
    auth_token = $2, last_updated
    WHERE id = $1 RETURNING *;"#)
        .bind(token_id).bind(auth_token).bind(Utc::now())
        .fetch_one(conn).await
}
// TODO: add validate_user token (user_id, auth_token)
// Update user token (refresh_token, user_id)
// Delete user token (token_id)