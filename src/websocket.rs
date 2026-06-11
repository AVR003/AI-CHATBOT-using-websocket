use axum::{
    extract::ws::{
        Message,
        WebSocket,
    },
};
use axum::extract::State;
use sqlx::PgPool;
use axum::extract::ws::WebSocketUpgrade;
use axum::response::Response;
use crate::ai::ask_gemini;

pub async fn handle_socket(
    mut socket: WebSocket,
    pool: PgPool,
) {
    while let Some(msg) = socket.recv().await {

        if let Ok(Message::Text(text)) = msg {

            println!("Received: {}", text);

            sqlx::query(
                "INSERT INTO chats(sender, message)
                 VALUES ($1, $2)"
            )
            .bind("user")
            .bind(text.to_string())
            .execute(&pool)
            .await
            .unwrap();

            println!("Calling Gemini...");

            let ai_response = ask_gemini(&text).await;
            println!("Gemini replied: {}", ai_response);

            sqlx::query(
                "INSERT INTO chats(sender, message)
                 VALUES ($1, $2)"
            )
            .bind("ai")
            .bind(&ai_response)
            .execute(&pool)
            .await
            .unwrap();

            socket
                .send(
                    Message::Text(
                        ai_response.into()
                    ),
                )
                .await
                .unwrap();
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(pool): State<PgPool>,
) -> Response {

    ws.on_upgrade(
        move |socket| handle_socket(socket, pool)
    )
}