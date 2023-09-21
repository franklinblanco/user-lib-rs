use crate::dto::users::UserRegisterPayload;

pub async fn register_user(db_conn: &sqlx::PgPool, user: UserRegisterPayload) -> Result<(), ()> {
    Ok(())
}
