use sqlx::{PgPool, query_as, query_as_with};
use crate::domain::user::User;

pub async fn insert_user(conn: &PgPool, user: User) -> Result<User, sqlx::Error> {
    sqlx::query_as(r#"
    INSERT INTO user (name, password, salt, time_created, last_updated)
    VALUES ($1, $2, $3, $4, $4) RETURNING *;
    "#)
        .bind(user.name).bind(user.password).bind(user.salt).bind(user.time_created)
        .fetch_one(conn)
        .await
}

pub async fn get_user_with_id(conn: &PgPool, user_id: &i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as(r#"
    SELECT * FROM user where id = $1;
    "#, )
        .bind(user_id)
        .fetch_optional(conn)
        .await
}

pub async fn update_user(conn: &PgPool, user: User) -> Result<User, sqlx::Error> {
    sqlx::query_as(r#"
    UPDATE user SET
    name = $2, password = $3, salt = $4, last_updated = $5
    WHERE id = $1 RETURNING *;
    "#, )
        .bind(user.id).bind(user.name).bind(user.password).bind(user.salt).bind(user.last_updated)
        .fetch_one(conn)
        .await
}

pub async fn delete_user(conn: &PgPool, user_id: &i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as(r#"
    DELETE FROM user where id = $1 RETURNING *;
    "#, )
        .bind(user_id)
        .fetch_optional(conn)
        .await
}