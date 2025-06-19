use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
    middleware::Next,
};

pub async fn socket_io_validation(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check if the request is a Socket.IO handshake or WebSocket upgrade
    let is_socket_io = request.uri().path().starts_with("/socket.io/");
    let is_websocket = request
        .headers()
        .get("upgrade")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.to_lowercase().contains("websocket"))
        .unwrap_or(false);

    if !is_socket_io && !is_websocket {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
} 