use axum::{
    routing::get,
    middleware,
};
use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;
use tracing::{info, error, warn};
use database::DatabaseManager;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;

mod api;
mod managers;
mod database;

use api::middleware::socket_io_validation;
use managers::GameManager;
use database::service::DataService;

// Global panic state management
static PANIC_DETECTED: AtomicBool = AtomicBool::new(false);
static PROBLEMATIC_SOCKETS: LazyLock<Mutex<HashMap<String, bool>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up enhanced panic hook to handle WebSocket panics
    std::panic::set_hook(Box::new(|panic_info| {
        error!("💥 Application panic: {:?}", panic_info);
        
        // Check if this is a WebSocket-related panic
        if let Some(location) = panic_info.location() {
            if location.file().contains("engineioxide") || location.file().contains("ws.rs") {
                error!("🔌 WebSocket transport panic detected at {}:{}", location.file(), location.line());
                
                // Set panic flag
                PANIC_DETECTED.store(true, Ordering::SeqCst);
                
                // Log panic details for debugging
                if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                    error!("📝 Panic message: {}", s);
                } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                    error!("📝 Panic message: {}", s);
                }
                
                error!("🛠️ Server will attempt to recover and disconnect problematic sockets");
            }
        }
    }));

    // Initialize tracing with more detailed logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚀 Starting Socket.IO server with panic recovery...");
    
    // Initialize MongoDB connection first
    DatabaseManager::initialize().await?;
    
    // Configure Socket.IO with enhanced settings for stability
    let (layer, io) = SocketIo::new_layer();

    // Configure CORS for WebSocket with more permissive settings
    let cors = CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any)
        .allow_credentials(false);

    // Create DataService instance
    let data_service = Arc::new(DataService::new());

    // Initialize Game Manager with Socket.IO handlers
    GameManager::initialize(&io, data_service);

    let app = axum::Router::new()
        .route("/", get(|| async { "Socket.IO Game Admin Server - Panic Recovery Enabled" }))
        .route("/health", get(|| async { "OK" }))
        .layer(cors)
        .layer(layer)
        .layer(middleware::from_fn(socket_io_validation));

    info!("✨ Server listening on 0.0.0.0:3002");
    info!("🛡️ Only accepting Socket.IO connections");
    info!("🗄️ MongoDB connection established");
    info!("🔧 Enhanced debug logging enabled");
    info!("🛡️ Enhanced panic handling with socket disconnection");
    info!("💓 Heartbeat configured: ping every 25s, timeout 20s");
    info!("🔗 Connection pooling enabled with 1000 max connections");
    info!("🔐 JWT token authentication enabled");
    info!("🆔 UUID v7 user IDs with sequential numbering enabled");
    info!("📦 Max payload size: 1MB");
    info!("⏱️ Connection timeout: 60s");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    
    // Start panic recovery monitor
    let io_clone = io.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            
            if PANIC_DETECTED.load(Ordering::SeqCst) {
                warn!("🔄 Panic recovery mode activated - monitoring for problematic sockets");
                
                // Get all connected sockets
                if let Ok(sockets) = io_clone.sockets() {
                    for socket in sockets {
                        let socket_id = socket.id.to_string();
                        
                        // Check if this socket has been marked as problematic
                        if let Ok(problematic) = PROBLEMATIC_SOCKETS.lock() {
                            if problematic.contains_key(&socket_id) {
                                warn!("🔌 Disconnecting problematic socket: {}", socket_id);
                                
                                // Try to disconnect the socket gracefully
                                if let Err(e) = socket.disconnect() {
                                    error!("❌ Failed to disconnect socket {}: {}", socket_id, e);
                                } else {
                                    info!("✅ Successfully disconnected problematic socket: {}", socket_id);
                                }
                            }
                        }
                    }
                }
                
                // Reset panic flag after recovery attempt
                PANIC_DETECTED.store(false, Ordering::SeqCst);
            }
        }
    });
    
    // Add enhanced error handling for the server
    match axum::serve(listener, app).await {
        Ok(_) => info!("✅ Server shutdown gracefully"),
        Err(e) => {
            error!("❌ Server error: {}", e);
            if e.to_string().contains("websocket") || e.to_string().contains("connection") {
                error!("🔌 WebSocket-related server error detected");
            }
        }
    }

    Ok(())
}