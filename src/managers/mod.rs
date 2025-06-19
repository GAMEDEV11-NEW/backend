pub mod connection;
pub mod validation;
pub mod events;

use socketioxide::SocketIo;
use tracing::info;

pub struct GameManager;

impl GameManager {
    pub fn initialize(io: &SocketIo) {
        info!("🎮 Initializing Game Manager...");
        
        // Register all custom events
        events::EventManager::register_custom_events(io);
        
        info!("✅ Game Manager initialized successfully!");
    }
}
