const io = require('socket.io-client');
const chalk = require('chalk');

// Connect to the Socket.IO server
const socket = io('http://localhost:3002');

// Log with colors and emojis
const log = {
    info: (msg) => console.log(chalk.blue('ℹ️ ' + msg)),
    success: (msg) => console.log(chalk.green('✅ ' + msg)),
    error: (msg) => console.log(chalk.red('❌ ' + msg)),
    warn: (msg) => console.log(chalk.yellow('⚠️ ' + msg))
};

// Handle connection events
socket.on('connect', () => {
    log.success('Connected to server');

    // Test device connection
    log.info('Testing device connection...');
    socket.emit('device:connect', {
        deviceId: 'test-device-001',
        type: 'game-console',
        status: 'online'
    });

    // Test game action
    setTimeout(() => {
        log.info('Testing game action...');
        socket.emit('game:action', {
            gameId: 'game-001',
            action: 'start_match',
            players: ['player1', 'player2']
        });
    }, 1000);

    // Test admin command
    setTimeout(() => {
        log.info('Testing admin command...');
        socket.emit('admin:command', {
            command: 'restart_server',
            params: { force: true }
        });
    }, 2000);
});

// Handle acknowledgments
socket.on('device:ack', (response) => {
    log.success('Device acknowledgment received:');
    console.log(response);
});

socket.on('game:ack', (response) => {
    log.success('Game acknowledgment received:');
    console.log(response);
});

socket.on('admin:ack', (response) => {
    log.success('Admin acknowledgment received:');
    console.log(response);
});

// Handle connection errors
socket.on('connect_error', (error) => {
    log.error(`Connection error: ${error.message}`);
});

socket.on('error', (error) => {
    log.error(`Socket error: ${error.message}`);
});

// Handle disconnection
socket.on('disconnect', () => {
    log.warn('Disconnected from server');
});

// Clean up on process exit
process.on('SIGINT', () => {
    log.info('Closing connection...');
    socket.close();
    process.exit();
}); 