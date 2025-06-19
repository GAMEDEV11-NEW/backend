const io = require('socket.io-client');
const chalk = require('chalk');
const { runOtpTests } = require('./test-otp');

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

    // Test device info
    log.info('Testing device info...');
    socket.emit('device:info', {
        device_id: 'test-device-001',
        device_type: 'game-console',
        timestamp: new Date().toISOString(),
        manufacturer: 'TestCorp',
        model: 'GameStation Pro',
        firmware_version: '1.2.3',
        capabilities: ['multiplayer', 'streaming', 'vr']
    });

    // Test login functionality
    setTimeout(() => {
        log.info('Testing login functionality...');
        
        // Test valid login
        socket.emit('login', {
            username: 'testuser',
            password: 'password123',
            device_id: 'test-device-001',
            timestamp: new Date().toISOString()
        });
    }, 1000);

    // Test invalid login
    setTimeout(() => {
        log.info('Testing invalid login...');
        socket.emit('login', {
            username: 'ab', // Too short
            password: '123' // Too short
        });
    }, 2000);

    // Test admin command
    setTimeout(() => {
        log.info('Testing admin command...');
        socket.emit('admin:command', {
            command: 'restart_server',
            params: { force: true }
        });
    }, 3000);
});

// Handle device info acknowledgment
socket.on('device:info:ack', (response) => {
    log.success('Device info acknowledgment received:');
    console.log(response);
});

// Handle login responses
socket.on('login:success', (response) => {
    log.success('Login successful:');
    console.log(response);
});

socket.on('login:error', (response) => {
    log.error('Login failed:');
    console.log(response);
});

// Handle connection errors
socket.on('connection_error', (response) => {
    log.error('Connection error:');
    console.log(response);
});

// Handle admin acknowledgment
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

async function runAllTests() {
    console.log('�� Starting All Tests Suite...\n');
    
    try {
        // Run OTP verification tests
        console.log('🔢 Running OTP Verification Tests...');
        await runOtpTests();
        
        console.log('\n✅ All test suites completed!');
        
    } catch (error) {
        console.error('❌ Test suite execution failed:', error);
        process.exit(1);
    }
}

// Run all tests
runAllTests(); 