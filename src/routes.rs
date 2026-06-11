use axum::Json;
use serde::{Deserialize, Serialize};
use axum::{
    extract::State,
};

#[derive(Deserialize)]
pub struct CreateMessage {
    pub sender: String,
    pub message: String,
}

use crate::db::save_message;

use sqlx::PgPool;

use crate::db::get_messages;
use crate::models::Chat;

pub async fn create_message(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateMessage>,
) -> &'static str {

    save_message(
        &pool,
        &payload.sender,
        &payload.message,
    )
    .await;

    "Message Saved"
}

pub async fn fetch_messages(
    State(pool): State<PgPool>,
) -> Json<Vec<Chat>> {

    let messages =
        get_messages(&pool).await;

    Json(messages)
}