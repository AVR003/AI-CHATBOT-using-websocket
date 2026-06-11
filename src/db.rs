use sqlx::PgPool;
use crate::models::Chat;

pub async fn save_message(
    pool: &PgPool,
    sender: &str,
    message: &str,
) {
    sqlx::query(
        "INSERT INTO chats(sender,message)
         VALUES($1,$2)"
    )
    .bind(sender)
    .bind(message)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn get_messages(
    pool: &PgPool,
) -> Vec<Chat> {

    sqlx::query_as::<_, Chat>(
        "SELECT sender,message
         FROM chats
         ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .unwrap()
}