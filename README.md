# Game Admin Backend (Rust)

A robust Socket.IO server built with Rust for managing game administration, device connections, and real-time game actions.

## Project Structure

```
src/
├── api/                    # API related code
│   ├── middleware/        # Custom middleware components
│   │   ├── connection_guard.rs    # WebSocket connection validation
│   │   ├── socket_validation.rs   # Socket.IO validation
│   │   └── mod.rs
│   └── mod.rs
├── handlers/              # Event handlers
│   ├── socket_handlers.rs # Socket.IO event handler implementations
│   └── mod.rs
├── managers/             # Business logic managers
│   ├── socket_manager.rs # Socket.IO connection management
│   └── mod.rs
└── main.rs              # Application entry point

test-client/            # Test client implementations
├── test-all.js         # Complete test suite
├── test-device.js      # Device-specific tests
└── test_connections.py # Connection testing
```

## Features

- 🔐 Secure WebSocket/Socket.IO connections
- 🚦 Multiple namespaces for different client types:
  - `/` - Default namespace (supports all events)
  - `/admin` - Administrative operations
  - `/device` - Device management
  - `/game` - Game actions
- 📝 Comprehensive logging
- ⚡ Asynchronous operation
- 🛡️ CORS support
- 🔌 Connection validation
- 📊 Structured event handling

## Event Types

### Default Namespace (/)
- `device:connect` - Device connection events
- `game:action` - Game-related actions
- `admin:command` - Administrative commands
- `disconnect` - Connection termination

### Admin Namespace (/admin)
- `command` - Administrative operations
- `disconnect` - Connection termination

### Device Namespace (/device)
- `connect` - Device registration
- `disconnect` - Device disconnection

### Game Namespace (/game)
- `action` - Game state updates
- `disconnect` - Connection termination

## Getting Started

1. Install Rust and Cargo
2. Clone the repository
3. Install dependencies:
   ```bash
   cargo build
   ```
4. Run the server:
   ```bash
   cargo run
   ```

## Development

The project uses:
- Axum for the web framework
- Socket.IO for real-time communication
- Tower for middleware
- Tokio for async runtime

## Testing

Run the test suite:
```bash
# Run Rust tests
cargo test

# Run client tests
cd test-client
npm install
npm test
```

## Environment Variables

Create a `.env` file in the root directory:
```env
PORT=3002
HOST=0.0.0.0
```

## Modules Overview

- `api/`: Contains all API-related code including middleware and route handlers
- `config/`: Configuration management and environment settings
- `connection/`: WebSocket and Socket.IO connection handling
- `events/`: Socket.IO event handlers organized by domain
- `models/`: Data structures and database models
- `services/`: Business logic and service layer
- `utils/`: Helper functions and utility code

## Prerequisites

- Rust (latest stable version)
- Node.js (for test client)
- Cargo (Rust package manager)
- npm (Node.js package manager)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd <project-directory>
```

2. Build the Rust server:
```bash
cargo build
```

3. Set up the test client:
```bash
cd test-client
npm install
```

## Running the Server

1. Start the server:
```bash
cargo run
```
The server will start on `http://localhost:3002`

2. Run the test client:
```bash
cd test-client
npm test
```

## WebSocket Events

### Server Events

- `connection:ready`: Sent when server is ready to receive device information
- `device:info:ack`: Acknowledgment for received device information

### Client Events

- `device:info`: Send device information to server

## Event Data Structures

### Device Info Request
```json
{
    "device_id": "string",
    "device_type": "string",
    "manufacturer": "string",
    "model": "string",
    "firmware_version": "string",
    "capabilities": ["string"],
    "timestamp": "string (ISO format)"
}
```

### Device Response
```json
{
    "status": "string",
    "message": "string",
    "device_id": "string",
    "timestamp": "string (ISO format)",
    "data": "object (optional)"
}
```

## Error Handling

The server implements comprehensive error handling:
- Connection errors
- Message parsing errors
- Device communication errors

All errors are logged with appropriate context and timestamps.

## Development

The project uses:
- `socketioxide` for WebSocket handling
- `tokio` for async runtime
- `axum` for web server functionality
- `tracing` for logging
- `serde` for serialization/deserialization

## Logging

The server implements detailed logging with:
- Emoji indicators for different event types
- Timestamp for each log entry
- Detailed error information
- Connection status updates

## License

[Your License Here]

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request 