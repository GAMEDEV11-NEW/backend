# 🎮 Game Admin Backend - Socket.IO Documentation

## 📡 Connection Information

### **Server URL**
```
http://localhost:3002
```

### **Socket.IO Namespace**
```
/ (default namespace)
```

### **Connection Example**
```javascript
const io = require('socket.io-client');
const socket = io('http://localhost:3002');
```

---

## 🔗 Connection Flow

### **1. Initial Connection**
```javascript
socket.on('connect', () => {
    console.log('Connected to server');
    console.log('Socket ID:', socket.id);
});
```

### **2. Connect Response (Automatic)**
**Event:** `connect_response`  
**Message:** JSON object with token, message, timestamp, status, and event type  
**Triggered:** Automatically immediately after successful connection

```javascript
socket.on('connect_response', (response) => {
    console.log('🎉 Server welcome:', response.message);
    console.log('Token:', response.token);
    console.log('Socket ID:', response.socket_id);
    console.log('Timestamp:', response.timestamp);
    console.log('Status:', response.status);
    console.log('Event:', response.event);
});
```

### **3. Connection Status**
```javascript
socket.on('connect', () => {
    console.log('✅ Connected');
});

socket.on('disconnect', (reason) => {
    console.log('❌ Disconnected:', reason);
});

socket.on('connect_error', (error) => {
    console.log('❌ Connection Error:', error.message);
});
```

### **4. Connection Error**
**Event:** `connection_error`  
**Trigger:** When device info validation fails  
**Data:** JSON object with detailed error information

```javascript
socket.on('connection_error', (response) => {
    console.log('Validation error:', response.message);
    console.log('Error Code:', response.error_code);
    console.log('Error Type:', response.error_type);
    console.log('Field:', response.field);
    console.log('Details:', response.details);
    console.log('Status:', response.status);
    console.log('Timestamp:', response.timestamp);
    console.log('Socket ID:', response.socket_id);
    console.log('Event:', response.event);
    
    // Example response:
    // {
    //   "status": "error",
    //   "error_code": "EMPTY_FIELD",
    //   "error_type": "VALUE_ERROR",
    //   "field": "device_id",
    //   "message": "device_id cannot be empty",
    //   "details": {
    //     "min_length": 1,
    //     "received_length": 0
    //   },
    //   "timestamp": "2024-01-15T10:30:00Z",
    //   "socket_id": "abc123",
    //   "event": "connection_error"
    // }
});
```

---

## 📨 Client → Server Events

### **1. Device Information**
**Event:** `device:info`  
**Purpose:** Send device information to server  
**Acknowledgment:** `device:info:ack` (if valid) or `connection_error` (if invalid)

**Required Fields:**
- `device_id` (string, non-empty) - **Required**
- `device_type` (string, non-empty) - **Required**
- `timestamp` (string, ISO format) - **Required**

**Optional Fields:**
- `manufacturer` (string, non-empty if provided) - **Optional**
- `model` (string, non-empty if provided) - **Optional**
- `firmware_version` (string, non-empty if provided) - **Optional**
- `capabilities` (array of strings, non-empty if provided) - **Optional**

```javascript
// Send device info (valid with all fields)
socket.emit('device:info', {
    device_id: "test-device-001",
    device_type: "game-console",
    manufacturer: "TestCorp",
    model: "GameStation Pro",
    firmware_version: "2.1.0",
    capabilities: ["multiplayer", "streaming", "voice-chat"],
    timestamp: new Date().toISOString()
});

// Send minimal device info (only required fields)
socket.emit('device:info', {
    device_id: "test-device-002",
    device_type: "game-console",
    timestamp: new Date().toISOString()
    // Optional fields can be omitted
});

// Send device info with some optional fields
socket.emit('device:info', {
    device_id: "test-device-003",
    device_type: "game-console",
    manufacturer: "TestCorp", // Optional field
    model: "GameStation Pro", // Optional field
    timestamp: new Date().toISOString()
    // firmware_version and capabilities omitted
});

// Listen for acknowledgment (success)
socket.on('device:info:ack', (response) => {
    console.log('Device info acknowledged:', response.message);
    console.log('Status:', response.status);
    console.log('Timestamp:', response.timestamp);
    // Output: "Device info received"
});

// Listen for validation error
socket.on('connection_error', (response) => {
    console.log('Validation failed:', response.message);
    // Output: Specific error message like "device_id cannot be empty"
});
```

### **2. Device Status**
**Event:** `device:status`  
**Purpose:** Send device status updates  
**Acknowledgment:** `device:status:ack`

```javascript
// Send device status
socket.emit('device:status', {
    device_id: "test-device-001",
    status: "online",
    battery_level: 85,
    network_quality: "excellent",
    active_games: 2,
    timestamp: new Date().toISOString()
});

// Listen for acknowledgment
socket.on('device:status:ack', (response) => {
    console.log('Device status acknowledged:', response.message);
    console.log('Status:', response.status);
    console.log('Timestamp:', response.timestamp);
    // Output: "Device status received"
});
```

### **3. Game Actions**
**Event:** `game:action`  
**Purpose:** Send game-related actions  
**Acknowledgment:** None (currently)

```javascript
// Send game action
socket.emit('game:action', {
    game_id: "game-001",
    action: "start_match",
    players: ["player1", "player2", "player3"],
    settings: {
        mode: "competitive",
        map: "desert_arena",
        time_limit: 300
    },
    timestamp: new Date().toISOString()
});
```

### **4. Admin Commands**
**Event:** `admin:command`  
**Purpose:** Send administrative commands  
**Acknowledgment:** None (currently)

```javascript
// Send admin command
socket.emit('admin:command', {
    command: "restart_server",
    params: { 
        force: true, 
        reason: "maintenance" 
    },
    admin_id: "admin-001",
    timestamp: new Date().toISOString()
});
```

---

## 📨 Server → Client Events

### **1. Welcome Message**
**Event:** `welcome`  
**Trigger:** Automatic after connection  
**Data:** String message

```javascript
socket.on('welcome', (message) => {
    console.log('Welcome message:', message);
    // Output: "Welcome to the Game Admin Server!"
});
```

### **2. Device Info Acknowledgment**
**Event:** `device:info:ack`  
**Trigger:** After receiving `device:info` event  
**Data:** JSON object with status, message, timestamp, and socket_id

```javascript
socket.on('device:info:ack', (response) => {
    console.log('Device info acknowledgment:', response.message);
    console.log('Status:', response.status);
    console.log('Timestamp:', response.timestamp);
    console.log('Socket ID:', response.socket_id);
    
    // Example response:
    // {
    //   "status": "success",
    //   "message": "Device info received",
    //   "timestamp": "2024-01-15T10:30:00Z",
    //   "socket_id": "abc123",
    //   "data_received": { ... }
    // }
});
```

### **3. Device Status Acknowledgment**
**Event:** `device:status:ack`  
**Trigger:** After receiving `device:status` event  
**Data:** JSON object with status, message, timestamp, and socket_id

```javascript
socket.on('device:status:ack', (response) => {
    console.log('Device status acknowledgment:', response.message);
    console.log('Status:', response.status);
    console.log('Timestamp:', response.timestamp);
    console.log('Socket ID:', response.socket_id);
    
    // Example response:
    // {
    //   "status": "success",
    //   "message": "Device status received",
    //   "timestamp": "2024-01-15T10:30:00Z",
    //   "socket_id": "abc123",
    //   "data_received": { ... }
    // }
});
```

---

## 🔄 Complete Connection Example

```javascript
const io = require('socket.io-client');

// Connect to server
const socket = io('http://localhost:3002');

// Connection events
socket.on('connect', () => {
    console.log('✅ Connected to server');
    console.log('Socket ID:', socket.id);
    
    // Send device info after connection
    socket.emit('device:info', {
        device_id: "my-device-001",
        device_type: "game-console",
        manufacturer: "MyCorp",
        model: "GameStation",
        firmware_version: "1.0.0",
        capabilities: ["multiplayer"],
        timestamp: new Date().toISOString()
    });
});

// Welcome message
socket.on('connection_ready', (response) => {
    console.log('🎉 Server welcome:', response.message);
    console.log('Token:', response.token);
    console.log('Socket ID:', response.socket_id);
    console.log('Timestamp:', response.timestamp);
    console.log('Status:', response.status);
});

// Device acknowledgments
socket.on('device:info:ack', (response) => {
    console.log('📱 Device info acknowledged:', response);
    
    // Send device status after info is acknowledged
    socket.emit('device:status', {
        device_id: "my-device-001",
        status: "online",
        battery_level: 90,
        network_quality: "good",
        active_games: 1,
        timestamp: new Date().toISOString()
    });
});

socket.on('device:status:ack', (response) => {
    console.log('📊 Device status acknowledged:', response.message);
    console.log('Status:', response.status);
    console.log('Timestamp:', response.timestamp);
    // Output: "Device status received"
});

// Error handling
socket.on('connect_error', (error) => {
    console.log('❌ Connection error:', error.message);
});

socket.on('disconnect', (reason) => {
    console.log('🔌 Disconnected:', reason);
});

// Cleanup on exit
process.on('SIGINT', () => {
    console.log('🛑 Closing connection...');
    socket.disconnect();
    process.exit();
});
```

---

## 📊 Event Summary Table

| Event | Direction | Purpose | Acknowledgment |
|-------|-----------|---------|----------------|
| `connect` | Client ← Server | Connection established | - |
| `connect_response` | Client ← Server | Welcome message with token | - |
| `device:info` | Client → Server | Send device information | `device:info:ack` or `connection_error` |
| `device:status` | Client → Server | Send device status | `device:status:ack` |
| `game:action` | Client → Server | Send game actions | None |
| `admin:command` | Client → Server | Send admin commands | None |
| `connection_error` | Client ← Server | Validation error response | - |
| `disconnect` | Client ← Server | Connection terminated | - |

---

## 🧪 Testing Commands

### **Run Automated Tests**
```bash
cd test-client
node test-backend.js
```

### **Run Interactive Mode**
```bash
cd test-client
node test-backend.js --interactive
```

### **Available Interactive Commands**
- `device:info` - Send device information
- `device:status` - Send device status
- `game:action` - Send game action
- `admin:command` - Send admin command
- `status` - Show connection status
- `quit` - Exit interactive mode

---

## 🔧 Server Configuration

### **Rust Backend Details**
- **Framework:** Axum + Socket.IO
- **Port:** 3002
- **Host:** 0.0.0.0
- **CORS:** Enabled for all origins
- **Middleware:** Socket.IO validation

### **Connection Options**
```javascript
const socket = io('http://localhost:3002', {
    reconnection: true,
    reconnectionAttempts: 5,
    reconnectionDelay: 1000,
    timeout: 5000
});
```

---

## 📝 Notes

1. **All timestamps** should be in ISO format: `new Date().toISOString()`
2. **Device IDs** should be unique identifiers
3. **Event acknowledgments** are sent automatically by the server
4. **Connection validation** is handled by middleware
5. **CORS** is enabled for web clients
6. **Reconnection** is automatic with exponential backoff

---

## 🚀 Next Steps

1. **Add more event handlers** in the Rust backend
2. **Implement data validation** for incoming events
3. **Add state management** for connected devices
4. **Create web dashboard** for monitoring
5. **Add authentication** for admin commands

---

*Last updated: $(date)*
*Backend Version: 0.1.0* 