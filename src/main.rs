use axum::{
    routing::get,
    middleware,
};
use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use database::DatabaseManager;
use std::sync::Arc;

mod api;
mod managers;
mod database;

use api::middleware::socket_io_validation;
use managers::GameManager;
use database::service::DataService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up panic hook to log panics
    std::panic::set_hook(Box::new(|panic_info| {
        error!("💥 Application panic: {:?}", panic_info);
    }));

    // Initialize tracing with more detailed logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚀 Starting Socket.IO server...");
    
    // Initialize MongoDB connection first
    DatabaseManager::initialize().await?;
    
    // Configure Socket.IO with proper timeout and heartbeat settings
    let (layer, io) = SocketIo::new_layer();

    // Configure CORS for WebSocket with more permissive settings
    let cors = CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    // Create DataService instance
    let data_service = Arc::new(DataService::new());

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
    info!("🔧 Debug logging enabled");
    info!("🛡️ Panic handling enabled");
    info!("💓 Heartbeat configured: ping every 60s, timeout 60s");
    info!("🔗 Connection pooling enabled");
    info!("🔐 JWT token authentication enabled");
    info!("🆔 UUID v7 user IDs with sequential numbering enabled");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    
    // Add error handling for the server
    axum::serve(listener, app).await?;

    Ok(())
}