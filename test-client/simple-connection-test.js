const io = require('socket.io-client');
const chalk = require('chalk');

// Beautiful console styling
const colors = {
    primary: chalk.cyan.bold,
    success: chalk.green.bold,
    error: chalk.red.bold,
    info: chalk.blue,
    highlight: chalk.magenta
};

const log = {
    success: (msg) => console.log(colors.success('✅ ' + msg)),
    error: (msg) => console.log(colors.error('❌ ' + msg)),
    info: (msg) => console.log(colors.info('ℹ️ ' + msg)),
    highlight: (msg) => console.log(colors.highlight('🎯 ' + msg))
};

console.log(colors.primary('🔗 Testing Socket.IO Connection with Welcome Data'));
console.log(colors.primary('='.repeat(50)));

// Connect to server
const socket = io('http://localhost:3002');

// Connection event
socket.on('connect', () => {
    log.success('Connected to server');
    log.info(`Socket ID: ${socket.id}`);
    log.highlight('Waiting for connect response data...');
});

// Connect response event (welcome data sent immediately after connect)
socket.on('connect_response', (response) => {
    console.log('\n' + colors.primary('📨 Connect Response Data Received:'));
    console.log(colors.primary('─'.repeat(35)));
    
    log.success(`Message: ${response.message}`);
    log.info(`Token: ${response.token}`);
    log.info(`Socket ID: ${response.socket_id}`);
    log.info(`Timestamp: ${response.timestamp}`);
    log.info(`Status: ${response.status}`);
    log.info(`Event: ${response.event}`);
    
    console.log('\n' + colors.highlight('🎉 Connect response test successful!'));
    console.log(colors.highlight('The server sent welcome data as connect response.'));
    
    // Disconnect after showing the data
    setTimeout(() => {
        log.info('Disconnecting...');
        socket.disconnect();
        process.exit(0);
    }, 2000);
});

// Error handling
socket.on('connect_error', (error) => {
    log.error(`Connection failed: ${error.message}`);
    process.exit(1);
});

socket.on('error', (error) => {
    log.error(`Socket error: ${error.message}`);
});

// Handle process termination
process.on('SIGINT', () => {
    log.info('Received SIGINT. Disconnecting...');
    socket.disconnect();
    process.exit(0);
}); 