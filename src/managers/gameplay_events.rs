use socketioxide::{SocketIo, extract::{SocketRef, Data}};
use tracing::{info, error};
use std::sync::Arc;
use crate::database::service::DataService;
use serde_json::Value;

pub struct GameplayEventManager;

impl GameplayEventManager {
    pub fn register_gameplay_events(io: &SocketIo, data_service: Arc<DataService>) {
        info!("🏀 Registering gameplay events...");
        
        // Define a namespace for gameplay-related events
        io.ns("/gameplay", move |socket: SocketRef| {
            let data_service = data_service.clone();
            async move {
                info!("Socket connected to gameplay namespace: {}", socket.id);
            
                // Example gameplay event
                socket.on("player_action", move |s: SocketRef, Data::<Value>(data)| {
                    let _data_service = data_service.clone();
                    async move {
                        info!("Received player_action event on socket {}: {:?}", s.id, data);
                        // Handle player action logic here, e.g., using _data_service
                    }
                });

                socket.on("disconnect", |socket: SocketRef| {
                    info!("Socket disconnected from gameplay namespace: {}", socket.id);
                });
            }
        });
        
        info!("✅ Gameplay events registered!");
    }
} 