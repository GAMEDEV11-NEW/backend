use std::net::SocketAddr;
use axum::Router;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use serde_json::{Value, json};
use tracing::info;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::FmtSubscriber;

mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;
mod websocket;

// Handler for new socket connections
async fn on_connect(socket: SocketRef) {
    info!("Client connected: {}", socket.id);

    // Send welcome message to the connected client
    let welcome_msg = json!({
        "data": format!("Connected successfully! SID: {}", socket.id)
    });
    let _ = socket.emit("welcome", welcome_msg);

    // Handle 'notice' events
    socket.on("notice", |socket: SocketRef, Data::<Value>(data)| async move {
        info!("[notice] From {}: {:?}", socket.id, data);
        let response = json!({
            "received": true,
            "echo": data
        });
        let _ = socket.emit("reply", response);
    });

    // Handle 'msg' events
    socket.on("msg", |socket: SocketRef, Data::<Value>(data)| async move {
        info!("[msg] From {}: {:?}", socket.id, data);
        
        // Broadcast to all other clients (excluding sender)
        let broadcast_msg = json!({
            "from": socket.id,
            "msg": data
        });
        let _ = socket.broadcast().emit("chat message", broadcast_msg);
    });

    // Handle disconnection
    socket.on_disconnect(|socket: SocketRef| {
        info!("Client disconnected: {}", socket.id);
    });
}

#[tokio::main]
async fn main() {
    // Initialize tracing (logging) with debug level
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .compact()
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Create a Socket.IO layer with configuration
    let (layer, io) = SocketIo::new_layer();

    // Set up connection handler
    io.ns("/", on_connect);

    // Set up CORS to allow all origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create a new router with the Socket.IO layer and CORS
    let app = Router::new()
        .layer(cors)
        .layer(layer);

    // Get the port from environment or use default
    let port = std::env::var("PORT").unwrap_or_else(|_| "3002".to_string());
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    info!("Socket.IO server running on http://{}", addr);
    info!("Debug logging enabled");
    info!("Ping interval: 5 seconds");
    info!("Ping timeout: 15 seconds");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
