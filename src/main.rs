mod db;
mod ai;
mod models;
mod routes;
mod websocket;
use axum::routing::{get, post};
use routes::{fetch_messages, create_message};
use websocket::ws_handler;
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

use axum::{
    Router,
};

use sqlx::PgPool;

async fn hello() -> &'static str {
    "Backend Running!"
}

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    println!(
    "GROQ KEY EXISTS: {:?}",
    std::env::var("GROQ_API_KEY")
    );

    let database_url =
        "postgres://postgres:firefox@localhost/chatbot_db";

    let pool = PgPool::connect(database_url)
        .await
        .unwrap();

    println!("Connected to PostgreSQL!");

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000"
    )
    .await
    .unwrap();

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([
        Method::GET,
        Method::POST,
    ])
    .allow_headers(Any);

    let app = Router::new()
    .route("/", get(hello))
    .route("/messages", get(fetch_messages).post(create_message))
    .route("/ws", get(ws_handler))
    .layer(cors)
    .with_state(pool.clone());

    println!("Server started on port 3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}