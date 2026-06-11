use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Chat {
    pub sender: String,
    pub message: String,
}