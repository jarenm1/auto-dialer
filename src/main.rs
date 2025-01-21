use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::routing::{get_service, post};
use axum::{routing::any, Router};

use axum_macros::debug_handler;

use std::error::Error;
use std::path::Path;

use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod twilio;
mod upload;
mod utils;
mod ws_twilio;

#[debug_handler]
async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ws_twilio::handle_socket)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let static_files = ServeDir::new(Path::new("./static"));

    let app = Router::new()
        .route("/upload", post(upload::upload_handler))
        .route("/ws", any(ws_handler))
        .fallback_service(get_service(static_files));

    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
