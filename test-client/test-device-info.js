const { io } = require("socket.io-client");
const chalk = require('chalk');

function log(type, message, data = null) {
    const timestamp = new Date().toISOString();
    const prefix = `[DEVICE-INFO] `;
    switch (type) {
        case 'info':
            console.log(chalk.blue(`${prefix}[${timestamp}] ℹ️ ${message}`));
            break;
        case 'success':
            console.log(chalk.green(`${prefix}[${timestamp}] ✅ ${message}`));
            break;
        case 'error':
            console.log(chalk.red(`${prefix}[${timestamp}] ❌ ${message}`));
            break;
        case 'debug':
            console.log(chalk.gray(`${prefix}[${timestamp}] 🔍 ${message}`));
            break;
    }
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

// Connect to the Socket.IO server
const socket = io("http://localhost:3002", {
    transports: ['websocket'],
    reconnection: true,
    reconnectionAttempts: 5,
    reconnectionDelay: 1000
});

// Connection event
socket.on("connect", () => {
    log('success', `Connected to server with ID: ${socket.id}`);
});

// Connection established event
socket.on("connection:established", (data) => {
    log('info', 'Connection established', data);
    
    // Send device information
    log('debug', 'Sending device information...', deviceInfo);
    socket.emit("device:info", deviceInfo);
});

// Device info acknowledgment
socket.on("device:info:ack", (response) => {
    log('success', 'Device registration successful', response);
    log('info', 'Device info test completed successfully');
    
    // Close connection after successful test
    setTimeout(() => {
        socket.close();
        process.exit(0);
    }, 1000);
});

// Device info errors
socket.on("device:info:error", (error) => {
    log('error', 'Device registration failed', error);
    socket.close();
    process.exit(1);
});

// Error handling
socket.on("connect_error", (error) => {
    log('error', `Connection error: ${error.message}`);
});

socket.on("disconnect", (reason) => {
    log('info', `Disconnected: ${reason}`);
});

// Handle process termination
process.on('SIGINT', () => {
    log('info', 'Closing connection...');
    socket.close();
    process.exit();
}); 