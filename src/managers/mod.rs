pub mod connection;
pub mod validation;
pub mod events;
pub mod jwt;
pub mod gameplay_events;


use socketioxide::SocketIo;
use tracing::info;
use std::sync::Arc;
use crate::database::service::DataService;

pub struct GameManager;

impl GameManager {
    pub fn initialize(io: &SocketIo, data_service: Arc<DataService>) {
        info!("🎮 Initializing Game Manager...");
        
        // Register all custom events
        events::EventManager::register_custom_events(io, data_service.clone());

        // Register gameplay events
        gameplay_events::GameplayEventManager::register_gameplay_events(io, data_service);
        
        info!("✅ Game Manager initialized successfully!");
    }
}
