use axum::{
    routing::get,
    middleware,
};
use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;
use tracing::info;
use database::{DatabaseManager, service::DataService};
use std::sync::Arc;

mod api;
mod managers;
mod database;

use api::middleware::socket_io_validation;
use managers::GameManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Socket.IO server...");
    
    // Initialize MongoDB connection
    DatabaseManager::initialize().await?;
    let data_service = Arc::new(DataService::new());
    let (layer, io) = SocketIo::new_layer();

    // Configure CORS for WebSocket
    let cors = CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    // Initialize Game Manager with Socket.IO handlers
    GameManager::initialize(&io, data_service);

    let app = axum::Router::new()
        .route("/", get(|| async { "Socket.IO Game Admin Server - Socket.IO Only" }))
        .layer(cors)
        .layer(layer)
        .layer(middleware::from_fn(socket_io_validation));

    info!("✨ Server listening on 0.0.0.0:3002");
    info!("🛡️ Only accepting Socket.IO connections");
    info!("🗄️ MongoDB connection established");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    axum::serve(listener, app).await?;

    Ok(())
}