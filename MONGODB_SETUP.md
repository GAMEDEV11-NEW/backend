# MongoDB Setup Guide

## Prerequisites

1. Install MongoDB Community Edition
2. Start MongoDB service
3. Create a database for the game admin system

## Configuration

### Environment Variables

Create a `.env` file in the project root with the following variables:

```env
# MongoDB Configuration
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=game_admin

# Server Configuration
SERVER_PORT=3002
SERVER_HOST=0.0.0.0

# Logging Configuration
RUST_LOG=info
```

### MongoDB Installation

#### Windows
1. Download MongoDB Community Server from [mongodb.com](https://www.mongodb.com/try/download/community)
2. Install with default settings
3. Start MongoDB service:
   ```cmd
   net start MongoDB
   ```

#### macOS
```bash
# Using Homebrew
brew tap mongodb/brew
brew install mongodb-community
brew services start mongodb/brew/mongodb-community
```

#### Linux (Ubuntu/Debian)
```bash
# Import MongoDB public GPG key
wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | sudo apt-key add -

# Create list file for MongoDB
echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu focal/mongodb-org/6.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list

# Update package database
sudo apt-get update

# Install MongoDB
sudo apt-get install -y mongodb-org

# Start MongoDB
sudo systemctl start mongod
sudo systemctl enable mongod
```

## Database Collections

The system will automatically create the following collections:

- `users` - User information and status
- `game_sessions` - Game session tracking
- `game_events` - Event logging and analytics
- `system_metrics` - System performance metrics

## Testing the Connection

Run the server and check the logs for MongoDB connection status:

```bash
cargo run
```

You should see:
```
🗄️ Initializing MongoDB connection...
✅ MongoDB connected successfully to database: game_admin
```

## Data Models

### User
- `user_id`: Unique user identifier
- `username`: Display name
- `email`: Optional email address
- `device_info`: Device information
- `status`: Online/Offline/Away/Busy
- `created_at`: Account creation timestamp
- `updated_at`: Last update timestamp
- `last_login`: Last login timestamp
- `login_count`: Number of logins
- `is_active`: Account status

### GameSession
- `session_id`: Unique session identifier
- `user_id`: Associated user
- `game_id`: Optional game identifier
- `status`: Active/Paused/Ended/Crashed
- `created_at`: Session start timestamp
- `updated_at`: Last update timestamp
- `ended_at`: Session end timestamp
- `duration_seconds`: Session duration
- `metadata`: Additional session data

### GameEvent
- `event_id`: Unique event identifier
- `user_id`: Associated user
- `session_id`: Associated session (optional)
- `event_type`: Event category
- `event_data`: Event-specific data
- `timestamp`: Event timestamp
- `severity`: Low/Medium/High/Critical

### SystemMetrics
- `timestamp`: Metrics timestamp
- `active_users`: Number of active users
- `total_sessions`: Total active sessions
- `server_load`: Server load percentage
- `memory_usage`: Memory usage percentage
- `cpu_usage`: CPU usage percentage
- `network_throughput`: Network throughput

## Usage Examples

### Creating a User
```rust
use crate::database::service::DataService;

let data_service = DataService::new();
let user_id = data_service.create_user(
    "user123".to_string(),
    "JohnDoe".to_string(),
    None
).await?;
```

### Starting a Game Session
```rust
let session_id = data_service.start_game_session(
    "user123",
    Some("game456".to_string())
).await?;
```

### Logging Events
```rust
let event_data = bson::doc! {
    "score": 100,
    "level": 5,
    "achievement": "first_win"
};

data_service.log_game_event(
    "user123",
    "game_achievement",
    event_data,
    EventSeverity::Low
).await?;
```

## Troubleshooting

### Connection Issues
1. Ensure MongoDB service is running
2. Check if the port 27017 is available
3. Verify the connection string in `.env`
4. Check firewall settings

### Permission Issues
1. Ensure the application has read/write permissions
2. Check MongoDB user permissions if using authentication
3. Verify database name is correct

### Performance Issues
1. Monitor MongoDB logs for slow queries
2. Consider adding indexes for frequently queried fields
3. Check connection pool settings 