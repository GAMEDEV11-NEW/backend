const io = require('socket.io-client');
const chalk = require('chalk');

// Beautiful console styling
const colors = {
    primary: chalk.cyan.bold,
    success: chalk.green.bold,
    error: chalk.red.bold,
    warning: chalk.yellow.bold,
    info: chalk.blue,
    highlight: chalk.magenta,
    dim: chalk.gray
};

// Test configuration
const CONFIG = {
    serverUrl: 'http://localhost:3002',
    heartbeatInterval: 25000, // 25 seconds
    reconnectAttempts: 5,
    reconnectDelay: 1000,
    timeout: 60000 // 60 seconds
};

// Beautiful logging functions
const log = {
    header: (text) => {
        console.log('\n' + colors.primary('╔' + '═'.repeat(text.length + 4) + '╗'));
        console.log(colors.primary('║  ' + text + '  ║'));
        console.log(colors.primary('╚' + '═'.repeat(text.length + 4) + '╝'));
    },
    success: (msg) => console.log(colors.success('✅ ' + msg)),
    error: (msg) => console.log(colors.error('❌ ' + msg)),
    warning: (msg) => console.log(colors.warning('⚠️  ' + msg)),
    info: (msg) => console.log(colors.info('ℹ️  ' + msg)),
    heartbeat: (msg) => console.log(colors.highlight('💓 ' + msg)),
    connection: (msg) => console.log(colors.dim('🔌 ' + msg))
};

class HeartbeatTester {
    constructor() {
        this.socket = null;
        this.isConnected = false;
        this.heartbeatInterval = null;
        this.connectionStartTime = null;
        this.lastHeartbeat = null;
        this.heartbeatCount = 0;
        this.disconnectCount = 0;
    }

    // Initialize Socket.IO connection with proper configuration
    async connect() {
        log.header('Heartbeat Connection Tester');
        log.info(`Connecting to ${CONFIG.serverUrl}...`);

        return new Promise((resolve, reject) => {
            this.socket = io(CONFIG.serverUrl, {
                reconnection: true,
                reconnectionAttempts: CONFIG.reconnectAttempts,
                reconnectionDelay: CONFIG.reconnectDelay,
                timeout: CONFIG.timeout,
                transports: ['websocket', 'polling'], // Prefer WebSocket, fallback to polling
                upgrade: true,
                rememberUpgrade: true,
                forceNew: false
            });

            this.socket.on('connect', () => {
                this.isConnected = true;
                this.connectionStartTime = Date.now();
                log.success(`Connected to server (ID: ${this.socket.id})`);
                log.info(`Connection time: ${new Date().toISOString()}`);
                
                // Start heartbeat monitoring
                this.startHeartbeat();
                resolve();
            });

            this.socket.on('connect_error', (error) => {
                log.error(`Connection failed: ${error.message}`);
                reject(error);
            });

            this.socket.on('disconnect', (reason) => {
                this.isConnected = false;
                this.disconnectCount++;
                log.warning(`Disconnected: ${reason}`);
                log.info(`Disconnect count: ${this.disconnectCount}`);
                log.info(`Connection duration: ${this.getConnectionDuration()} seconds`);
                
                // Stop heartbeat when disconnected
                this.stopHeartbeat();
                
                if (reason === 'io server disconnect') {
                    log.info('Server initiated disconnect, attempting to reconnect...');
                }
            });

            // Set up event listeners
            this.setupEventListeners();
        });
    }

    // Set up Socket.IO event listeners
    setupEventListeners() {
        // Connect response event
        this.socket.on('connect_response', (response) => {
            log.success(`Server welcome: ${response.message}`);
            log.info(`Token: ${response.token}`);
            log.info(`Socket ID: ${response.socket_id}`);
            log.info(`Server info:`, response.server_info);
        });

        // Heartbeat events
        this.socket.on('heartbeat', (data) => {
            this.lastHeartbeat = Date.now();
            this.heartbeatCount++;
            log.heartbeat(`Received heartbeat from server (count: ${this.heartbeatCount})`);
            log.info(`Heartbeat data:`, data);
            
            // Respond to heartbeat
            this.sendHeartbeatResponse();
        });

        this.socket.on('pong', (data) => {
            log.heartbeat(`Received pong from server`);
            log.info(`Pong data:`, data);
        });

        this.socket.on('keepalive:ack', (data) => {
            log.heartbeat(`Received keepalive ack from server`);
            log.info(`Keepalive data:`, data);
        });

        // Error handling
        this.socket.on('error', (error) => {
            log.error(`Socket error: ${error.message}`);
        });

        // Connection error event
        this.socket.on('connection_error', (response) => {
            log.error(`Connection error: ${response.message}`);
            log.info(`Error Code: ${response.error_code}`);
            log.info(`Error Type: ${response.error_type}`);
        });
    }

    // Start heartbeat monitoring
    startHeartbeat() {
        log.info(`Starting heartbeat monitoring (interval: ${CONFIG.heartbeatInterval}ms)`);
        
        this.heartbeatInterval = setInterval(() => {
            if (this.isConnected) {
                this.sendHeartbeat();
            }
        }, CONFIG.heartbeatInterval);
    }

    // Stop heartbeat monitoring
    stopHeartbeat() {
        if (this.heartbeatInterval) {
            clearInterval(this.heartbeatInterval);
            this.heartbeatInterval = null;
            log.info('Stopped heartbeat monitoring');
        }
    }

    // Send heartbeat to server
    sendHeartbeat() {
        const heartbeat = {
            type: 'client_heartbeat',
            timestamp: new Date().toISOString(),
            socket_id: this.socket.id,
            client_info: {
                userAgent: navigator.userAgent,
                platform: navigator.platform,
                language: navigator.language
            }
        };

        this.socket.emit('ping', heartbeat);
        log.heartbeat(`Sent ping to server`);
    }

    // Send heartbeat response
    sendHeartbeatResponse() {
        const response = {
            type: 'client_pong',
            timestamp: new Date().toISOString(),
            socket_id: this.socket.id
        };

        this.socket.emit('pong', response);
        log.heartbeat(`Sent pong to server`);
    }

    // Send keepalive
    sendKeepalive() {
        const keepalive = {
            type: 'client_keepalive',
            timestamp: new Date().toISOString(),
            socket_id: this.socket.id
        };

        this.socket.emit('keepalive', keepalive);
        log.heartbeat(`Sent keepalive to server`);
    }

    // Get connection duration in seconds
    getConnectionDuration() {
        if (!this.connectionStartTime) return 0;
        return Math.floor((Date.now() - this.connectionStartTime) / 1000);
    }

    // Show connection status
    showStatus() {
        log.info(`Connection Status: ${this.isConnected ? 'Connected' : 'Disconnected'}`);
        log.info(`Socket ID: ${this.socket?.id || 'N/A'}`);
        log.info(`Connection Duration: ${this.getConnectionDuration()} seconds`);
        log.info(`Heartbeat Count: ${this.heartbeatCount}`);
        log.info(`Disconnect Count: ${this.disconnectCount}`);
        log.info(`Last Heartbeat: ${this.lastHeartbeat ? new Date(this.lastHeartbeat).toISOString() : 'Never'}`);
    }

    // Run long-term connection test
    async runLongTermTest(durationMinutes = 10) {
        log.header(`Long-term Connection Test (${durationMinutes} minutes)`);
        
        const startTime = Date.now();
        const endTime = startTime + (durationMinutes * 60 * 1000);
        
        // Send keepalive every 30 seconds
        const keepaliveInterval = setInterval(() => {
            if (this.isConnected) {
                this.sendKeepalive();
            }
        }, 30000);

        // Show status every minute
        const statusInterval = setInterval(() => {
            this.showStatus();
        }, 60000);

        // Wait for test duration
        while (Date.now() < endTime) {
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            // Check if still connected
            if (!this.isConnected) {
                log.warning('Connection lost during test, attempting to reconnect...');
                try {
                    await this.connect();
                } catch (error) {
                    log.error(`Failed to reconnect: ${error.message}`);
                }
            }
        }

        // Cleanup intervals
        clearInterval(keepaliveInterval);
        clearInterval(statusInterval);
        
        // Final status
        log.header('Test Complete');
        this.showStatus();
        
        const totalDuration = Math.floor((Date.now() - startTime) / 1000);
        log.info(`Total test duration: ${totalDuration} seconds`);
        log.info(`Average connection uptime: ${this.calculateUptime()}%`);
    }

    // Calculate connection uptime percentage
    calculateUptime() {
        if (this.disconnectCount === 0) return 100;
        // This is a simplified calculation
        return Math.max(0, 100 - (this.disconnectCount * 10));
    }

    // Cleanup
    cleanup() {
        this.stopHeartbeat();
        if (this.socket) {
            this.socket.disconnect();
        }
        log.info('Cleanup completed');
    }
}

// Main function
async function main() {
    const tester = new HeartbeatTester();
    
    try {
        // Connect to server
        await tester.connect();
        
        // Show initial status
        tester.showStatus();
        
        // Run long-term test (5 minutes by default)
        await tester.runLongTermTest(5);
        
    } catch (error) {
        log.error(`Test failed: ${error.message}`);
    } finally {
        tester.cleanup();
    }
}

// Handle process termination
process.on('SIGINT', () => {
    log.info('Received SIGINT. Shutting down...');
    process.exit(0);
});

process.on('SIGTERM', () => {
    log.info('Received SIGTERM. Shutting down...');
    process.exit(0);
});

// Run the test
if (require.main === module) {
    main().catch(console.error);
}

module.exports = HeartbeatTester; 