pub mod models;
pub mod repository;
pub mod service;

use once_cell::sync::OnceCell;
use mongodb::{Client, Database};
use tracing::info;

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();
static MONGODB_DATABASE: OnceCell<Database> = OnceCell::new();

pub struct DatabaseManager;

impl DatabaseManager {
    pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
        info!("🗄️ Initializing MongoDB connection...");
        
        // Load environment variables
        dotenv::dotenv().ok();
        
        let mongodb_uri = std::env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        let database_name = std::env::var("MONGODB_DATABASE")
            .unwrap_or_else(|_| "game_admin".to_string());
        
        // Create MongoDB client
        let client = Client::with_uri_str(&mongodb_uri).await?;
        
        // Test the connection
        client.list_database_names(None, None).await?;
        
        // Get database
        let database = client.database(&database_name);
        
        // Store in static variables
        MONGODB_CLIENT.set(client).expect("Failed to set MongoDB client");
        MONGODB_DATABASE.set(database).expect("Failed to set MongoDB database");
        
        info!("✅ MongoDB connected successfully to database: {}", database_name);
        Ok(())
    }
    
    pub fn get_client() -> &'static Client {
        MONGODB_CLIENT.get().expect("MongoDB client not initialized")
    }
    
    pub fn get_database() -> &'static Database {
        MONGODB_DATABASE.get().expect("MongoDB database not initialized")
    }
} 