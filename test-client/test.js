const { io } = require("socket.io-client");
const chalk = require('chalk');

// Helper function for logging
function log(type, message, data = null) {
    const timestamp = new Date().toISOString();
    switch (type) {
        case 'info':
            console.log(chalk.blue(`[${timestamp}] ℹ️ ${message}`));
            break;
        case 'success':
            console.log(chalk.green(`[${timestamp}] ✅ ${message}`));
            break;
        case 'error':
            console.log(chalk.red(`[${timestamp}] ❌ ${message}`));
            break;
        case 'warning':
            console.log(chalk.yellow(`[${timestamp}] ⚠️ ${message}`));
            break;
        case 'debug':
            console.log(chalk.gray(`[${timestamp}] 🔍 ${message}`));
            break;
    }
    console.log("awdawdawsefsef", type);
    if (data) {
        console.log(chalk.gray('Data:'), JSON.stringify(data, null, 2));
    }
}

// Mock device information
const deviceInfo = {
    deviceId: `TEST-DEVICE-${Math.random().toString(36).substr(2, 9)}`,
    deviceType: "test-client",
    manufacturer: "Test Corp",
    model: "Test Model X1",
    firmwareVersion: "1.0.0",
    capabilities: ["status", "telemetry", "diagnostics"],
    timestamp: new Date().toISOString()
};

// Connect to the Rust Socket.IO server
const socket = io("http://localhost:3002", {
    transports: ['websocket'],
    reconnection: true,
    reconnectionAttempts: 5,
    reconnectionDelay: 1000
});

// Function to start sending periodic status updates
function startStatusUpdates() {
    let statusCounter = 0;
    
    const interval = setInterval(() => {
        statusCounter++;
        const status = {
            deviceId: deviceInfo.deviceId,
            status: "online",
            uptime: statusCounter * 5,
            metrics: {
                cpu: Math.floor(Math.random() * 100),
                memory: Math.floor(Math.random() * 100),
                temperature: Math.floor(20 + Math.random() * 30)
            },
            timestamp: new Date().toISOString()
        };
        
        log('debug', 'Sending status update...', status);
        // socket.emit("device:status", status);
    }, 5000);

    // Store interval for cleanup
    socket.statusInterval = interval;
}

// Function to request device configuration
function requestDeviceConfig() {
    const configRequest = {
        deviceId: deviceInfo.deviceId,
        currentConfig: {
            updateInterval: 5000,
            reportLevel: "basic"
        },
        timestamp: new Date().toISOString()
    };
    
    log('debug', 'Requesting device configuration...', configRequest);
    // socket.emit("device:config", configRequest);
}

// Connection event
socket.on("connect", () => {
    log('success', `Connected to Socket.IO server with ID: ${socket.id}`);
});

// Connection established event
socket.on("connection:established", (data) => {
    log('info', 'Connection established', data);
    
    // Send device information after connection
    log('debug', 'Sending device information...', deviceInfo);
    socket.emit("device:info", deviceInfo);
});

// Device info acknowledgment
socket.on("device:info:ack", (response) => {
    log('success', 'Device info acknowledged', response);
    
    // // Start sending periodic status updates
    // startStatusUpdates();
    
    // // Request device configuration
    // requestDeviceConfig();
});

// // Device info errors
// socket.on("device:info:error", (error) => {
//     log('error', 'Device info error', error);
// });

// // Status acknowledgments
// socket.on("device:status:ack", (response) => {
//     log('success', 'Status update acknowledged', response);
// });

// // Status errors
// socket.on("device:status:error", (error) => {
//     log('error', 'Status update error', error);
// });

// Configuration responses
socket.on("device:config:response", (config) => {
    log('info', 'Received device configuration', config);
  
    // Simulate applying configuration
    log('debug', 'Applying configuration...');
    // setTimeout(() => {
    //     log('success', 'Configuration applied successfully');
        
    //     // Send confirmation
    //     socket.emit("device:config:applied", {
    //         deviceId: deviceInfo.deviceId,
    //         status: "success",
    //         timestamp: new Date().toISOString()
    //     });
    // }, 1000);
});

// Error handling
socket.on("connect_error", (error) => {
    log('error', `Connection error: ${error.message}`);
});

socket.on("disconnect", (reason) => {
    log('warning', `Disconnected: ${reason}`);
    
    // Clean up intervals
    if (socket.statusInterval) {
        clearInterval(socket.statusInterval);
    }
});

socket.on("reconnect_attempt", (attemptNumber) => {
    log('info', `Attempting to reconnect... (attempt ${attemptNumber})`);
});

socket.on("reconnect", (attemptNumber) => {
    log('success', `Reconnected after ${attemptNumber} attempts`);
});

// Handle process termination
process.on('SIGINT', () => {
    log('info', 'Closing Socket.IO connection...');
    
    // Clean up intervals
    if (socket.statusInterval) {
        clearInterval(socket.statusInterval);
    }
    
    socket.close();
    process.exit();
});