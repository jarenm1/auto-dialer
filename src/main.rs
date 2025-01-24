use axum::extract::WebSocketUpgrade;
use axum::extract::connect_info::ConnectInfo;
use axum::response::IntoResponse;
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
mod vonage;

#[debug_handler]
async fn ws_handler(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<std::net::SocketAddr> ) -> impl IntoResponse {
    ws.on_upgrade(move |socket| vonage::websocket::handle_socket(socket, addr))
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
