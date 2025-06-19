const { io } = require("socket.io-client");
const chalk = require('chalk');

function log(type, message, data = null) {
    const timestamp = new Date().toISOString();
    const prefix = `[DEVICE-STATUS] `;
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

// Device information
const deviceId = `TEST-DEVICE-${Math.random().toString(36).substr(2, 9)}`;
let statusCounter = 0;
let statusInterval;

// Connect to the Socket.IO server
const socket = io("http://localhost:3002", {
    transports: ['websocket'],
    reconnection: true,
    reconnectionAttempts: 5,
    reconnectionDelay: 1000
});

function sendStatusUpdate() {
    statusCounter++;
    const status = {
        deviceId: deviceId,
        status: "online",
        uptime: statusCounter * 5,
        metrics: {
            cpu: Math.floor(Math.random() * 100),
            memory: Math.floor(Math.random() * 100),
            temperature: Math.floor(20 + Math.random() * 30)
        },
        timestamp: new Date().toISOString()
    };
    
    log('debug', `Sending status update #${statusCounter}...`, status);
    socket.emit("device:status", status);
}

// Connection event
socket.on("connect", () => {
    log('success', `Connected to server with ID: ${socket.id}`);
});

// Connection established event
socket.on("connection:established", (data) => {
    log('info', 'Connection established', data);
    
    // Start sending status updates
    log('info', 'Starting status updates (every 5 seconds)');
    statusInterval = setInterval(sendStatusUpdate, 5000);
    sendStatusUpdate(); // Send first update immediately
});

// Status acknowledgments
socket.on("device:status:ack", (response) => {
    log('success', `Status update #${statusCounter} acknowledged`, response);
});

// Status errors
socket.on("device:status:error", (error) => {
    log('error', 'Status update failed', error);
});

// Error handling
socket.on("connect_error", (error) => {
    log('error', `Connection error: ${error.message}`);
});

socket.on("disconnect", (reason) => {
    log('info', `Disconnected: ${reason}`);
    if (statusInterval) {
        clearInterval(statusInterval);
    }
});

// Handle process termination
process.on('SIGINT', () => {
    log('info', 'Stopping status updates...');
    if (statusInterval) {
        clearInterval(statusInterval);
    }
    socket.close();
    process.exit();
}); 