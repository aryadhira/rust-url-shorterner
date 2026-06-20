mod config;
use config::Config;
use axum::{
    Json, Router, routing::{get, post}
};
use serde::{Deserialize,Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[derive(Deserialize)]
struct UserRequest {
    name: String,
}

#[derive(Serialize)]
struct UserResponse {
    greeting: String,
}

async fn health_check() -> &'static str {
    "OK"
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse{
        message: "Hello Rust".to_string(),
    })
}

async fn greet(
    Json(payload): Json<UserRequest>,
) -> Json<UserResponse> {
    Json(UserResponse { greeting: format!("Hello, {}!",payload.name) })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_config = Config::from_env();
    println!("server port : {}", app_config.database_url);
    println!("redis url : {}", app_config.redis_url);
    println!("server port : {}", app_config.server_port);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/hello", get(hello))
        .route("/greet", post(greet));

    let addr = SocketAddr::from(([127,0,0,1], app_config.server_port));

    println!("server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
