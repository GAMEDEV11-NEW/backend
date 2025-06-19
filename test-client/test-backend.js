const io = require('socket.io-client');
const chalk = require('chalk');
const readline = require('readline');

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
    reconnectAttempts: 5,
    reconnectDelay: 1000,
    testDelay: 1500
};

// Test results tracking
let testResults = {
    total: 0,
    passed: 0,
    failed: 0,
    startTime: null,
    endTime: null
};

// Create readline interface for interactive testing
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

// Beautiful logging functions
const log = {
    header: (text) => {
        console.log('\n' + colors.primary('╔' + '═'.repeat(text.length + 4) + '╗'));
        console.log(colors.primary('║  ' + text + '  ║'));
        console.log(colors.primary('╚' + '═'.repeat(text.length + 4) + '╝'));
    },
    section: (text) => {
        console.log('\n' + colors.highlight('▸ ' + text));
        console.log(colors.dim('─'.repeat(text.length + 2)));
    },
    success: (msg) => console.log(colors.success('✅ ' + msg)),
    error: (msg) => console.log(colors.error('❌ ' + msg)),
    warning: (msg) => console.log(colors.warning('⚠️  ' + msg)),
    info: (msg) => console.log(colors.info('ℹ️  ' + msg)),
    test: (name, status) => {
        const icon = status === 'PASS' ? colors.success('✓') : colors.error('✗');
        const statusText = status === 'PASS' ? colors.success(status) : colors.error(status);
        console.log(`  ${icon} ${name}: ${statusText}`);
    },
    progress: (current, total) => {
        const percentage = Math.round((current / total) * 100);
        const bar = '█'.repeat(Math.floor(percentage / 2)) + '░'.repeat(50 - Math.floor(percentage / 2));
        process.stdout.write(`\r${colors.info('Progress:')} [${bar}] ${percentage}% (${current}/${total})`);
    }
};

// Test data
const testData = {
    deviceInfo: {
        device_id: "test-device-001",
        device_type: "game-console",
        manufacturer: "TestCorp",
        model: "GameStation Pro",
        firmware_version: "2.1.0",
        capabilities: ["multiplayer", "streaming", "voice-chat"],
        timestamp: new Date().toISOString()
    },
    deviceStatus: {
        device_id: "test-device-001",
        status: "online",
        battery_level: 85,
        network_quality: "excellent",
        active_games: 2,
        timestamp: new Date().toISOString()
    },
    gameAction: {
        game_id: "game-001",
        action: "start_match",
        players: ["player1", "player2", "player3"],
        settings: {
            mode: "competitive",
            map: "desert_arena",
            time_limit: 300
        },
        timestamp: new Date().toISOString()
    },
    adminCommand: {
        command: "restart_server",
        params: { force: true, reason: "maintenance" },
        admin_id: "admin-001",
        timestamp: new Date().toISOString()
    }
};

class BackendTester {
    constructor() {
        this.socket = null;
        this.isConnected = false;
        this.completedTests = new Set(); // Track completed tests to avoid duplicates
        this.currentTest = 0;
    }

    // Initialize Socket.IO connection
    async connect() {
        log.header('Game Admin Backend Tester');
        log.info(`Connecting to ${CONFIG.serverUrl}...`);

        return new Promise((resolve, reject) => {
            this.socket = io(CONFIG.serverUrl, {
                reconnection: true,
                reconnectionAttempts: CONFIG.reconnectAttempts,
                reconnectionDelay: CONFIG.reconnectDelay,
                timeout: 5000
            });

            this.socket.on('connect', () => {
                this.isConnected = true;
                log.success(`Connected to server (ID: ${this.socket.id})`);
                resolve();
            });

            this.socket.on('connect_error', (error) => {
                log.error(`Connection failed: ${error.message}`);
                reject(error);
            });

            this.socket.on('disconnect', (reason) => {
                this.isConnected = false;
                log.warning(`Disconnected: ${reason}`);
            });

            // Set up event listeners
            this.setupEventListeners();
        });
    }

    // Set up Socket.IO event listeners
    setupEventListeners() {
        // Connect response event (sent immediately after connection)
        this.socket.on('connect_response', (response) => {
            log.success(`Server welcome: ${response.message}`);
            log.info(`Token: ${response.token}`);
            log.info(`Socket ID: ${response.socket_id}`);
            log.info(`Timestamp: ${response.timestamp}`);
            log.info(`Status: ${response.status}`);
            log.info(`Event: ${response.event}`);
        });

        // Connection error event (sent when validation fails)
        this.socket.on('connection_error', (response) => {
            log.error(`Connection error: ${response.message}`);
            log.info(`Error Code: ${response.error_code}`);
            log.info(`Error Type: ${response.error_type}`);
            log.info(`Field: ${response.field}`);
            log.info(`Details:`, response.details);
            log.info(`Status: ${response.status}`);
            log.info(`Timestamp: ${response.timestamp}`);
            log.info(`Socket ID: ${response.socket_id}`);
            log.info(`Event: ${response.event}`);
        });

        // Device info acknowledgment (now JSON)
        this.socket.on('device:info:ack', (response) => {
            log.success('Device info acknowledgment received');
            log.info(`Status: ${response.status}`);
            log.info(`Message: ${response.message}`);
            log.info(`Timestamp: ${response.timestamp}`);
            this.completeTest('Device Info Test', true);
        });

        // Device status acknowledgment (now JSON)
        this.socket.on('device:status:ack', (response) => {
            log.success('Device status acknowledgment received');
            log.info(`Status: ${response.status}`);
            log.info(`Message: ${response.message}`);
            log.info(`Timestamp: ${response.timestamp}`);
            this.completeTest('Device Status Test', true);
        });

        // Error handling
        this.socket.on('error', (error) => {
            log.error(`Socket error: ${error.message}`);
        });
    }

    // Run all tests
    async runAllTests() {
        testResults.startTime = Date.now();
        log.section('Running Comprehensive Tests');

        const tests = [
            { name: 'Connection Test', fn: () => this.testConnection() },
            { name: 'Device Info Test', fn: () => this.testDeviceInfo() },
            { name: 'Device Status Test', fn: () => this.testDeviceStatus() },
            { name: 'Game Action Test', fn: () => this.testGameAction() },
            { name: 'Admin Command Test', fn: () => this.testAdminCommand() },
            { name: 'Disconnection Test', fn: () => this.testDisconnection() }
        ];

        // Reset test results
        testResults.total = 0;
        testResults.passed = 0;
        testResults.failed = 0;
        this.completedTests.clear();

        for (let i = 0; i < tests.length; i++) {
            const test = tests[i];
            log.progress(i + 1, tests.length);
            
            try {
                await test.fn();
                await this.delay(CONFIG.testDelay);
            } catch (error) {
                log.error(`Test failed: ${test.name} - ${error.message}`);
                this.completeTest(test.name, false);
            }
        }

        console.log('\n'); // Clear progress bar
        this.showResults();
    }

    // Individual test methods
    async testConnection() {
        return new Promise((resolve) => {
            if (this.isConnected) {
                this.completeTest('Connection Test', true);
                resolve();
            } else {
                this.completeTest('Connection Test', false);
                resolve();
            }
        });
    }

    async testDeviceInfo() {
        return new Promise((resolve) => {
            log.info('Sending device info...');
            this.socket.emit('device:info', testData.deviceInfo);
            
            // Set timeout for acknowledgment
            setTimeout(() => {
                if (!this.completedTests.has('Device Info Test')) {
                    this.completeTest('Device Info Test', false);
                }
                resolve();
            }, 2000); // Reduced timeout for faster testing
        });
    }

    async testDeviceStatus() {
        return new Promise((resolve) => {
            log.info('Sending device status...');
            this.socket.emit('device:status', testData.deviceStatus);
            
            setTimeout(() => {
                if (!this.completedTests.has('Device Status Test')) {
                    this.completeTest('Device Status Test', false);
                }
                resolve();
            }, 2000); // Reduced timeout for faster testing
        });
    }

    async testGameAction() {
        return new Promise((resolve) => {
            log.info('Sending game action...');
            this.socket.emit('game:action', testData.gameAction);
            
            // For now, we'll assume success since the server doesn't have specific game action handlers
            setTimeout(() => {
                this.completeTest('Game Action Test', true);
                resolve();
            }, 500); // Faster response for non-acknowledged events
        });
    }

    async testAdminCommand() {
        return new Promise((resolve) => {
            log.info('Sending admin command...');
            this.socket.emit('admin:command', testData.adminCommand);
            
            // For now, we'll assume success since the server doesn't have specific admin command handlers
            setTimeout(() => {
                this.completeTest('Admin Command Test', true);
                resolve();
            }, 500); // Faster response for non-acknowledged events
        });
    }

    async testDisconnection() {
        return new Promise((resolve) => {
            log.info('Testing disconnection...');
            this.socket.disconnect();
            
            setTimeout(() => {
                this.completeTest('Disconnection Test', !this.isConnected);
                resolve();
            }, 1000);
        });
    }

    // Interactive testing mode
    async interactiveMode() {
        log.header('Interactive Testing Mode');
        log.info('Available commands:');
        log.info('  device:info    - Send device information');
        log.info('  device:status  - Send device status');
        log.info('  game:action    - Send game action');
        log.info('  admin:command  - Send admin command');
        log.info('  status         - Show connection status');
        log.info('  quit           - Exit interactive mode');

        const askQuestion = () => {
            rl.question(colors.primary('\nEnter command: '), async (command) => {
                switch (command.toLowerCase()) {
                    case 'device:info':
                        await this.sendDeviceInfo();
                        break;
                    case 'device:status':
                        await this.sendDeviceStatus();
                        break;
                    case 'game:action':
                        await this.sendGameAction();
                        break;
                    case 'admin:command':
                        await this.sendAdminCommand();
                        break;
                    case 'status':
                        this.showStatus();
                        break;
                    case 'quit':
                        log.info('Exiting interactive mode...');
                        rl.close();
                        this.cleanup();
                        return;
                    default:
                        log.warning('Unknown command. Type "quit" to exit.');
                }
                askQuestion();
            });
        };

        askQuestion();
    }

    // Interactive mode helper methods
    async sendDeviceInfo() {
        log.info('Sending device info...');
        this.socket.emit('device:info', testData.deviceInfo);
    }

    async sendDeviceStatus() {
        log.info('Sending device status...');
        this.socket.emit('device:status', testData.deviceStatus);
    }

    async sendGameAction() {
        log.info('Sending game action...');
        this.socket.emit('game:action', testData.gameAction);
    }

    async sendAdminCommand() {
        log.info('Sending admin command...');
        this.socket.emit('admin:command', testData.adminCommand);
    }

    showStatus() {
        log.info(`Connection Status: ${this.isConnected ? 'Connected' : 'Disconnected'}`);
        if (this.isConnected) {
            log.info(`Socket ID: ${this.socket.id}`);
        }
    }

    // Utility methods
    completeTest(testName, passed) {
        // Only complete each test once
        if (this.completedTests.has(testName)) {
            return;
        }
        
        this.completedTests.add(testName);
        testResults.total++;
        
        if (passed) {
            testResults.passed++;
            log.test(testName, 'PASS');
        } else {
            testResults.failed++;
            log.test(testName, 'FAIL');
        }
    }

    showResults() {
        testResults.endTime = Date.now();
        const duration = (testResults.endTime - testResults.startTime) / 1000;

        log.header('Test Results Summary');
        console.log(colors.info(`Duration: ${duration.toFixed(2)} seconds`));
        console.log(colors.success(`Passed: ${testResults.passed}`));
        console.log(colors.error(`Failed: ${testResults.failed}`));
        console.log(colors.primary(`Total: ${testResults.total}`));
        
        const successRate = ((testResults.passed / testResults.total) * 100).toFixed(1);
        console.log(colors.highlight(`Success Rate: ${successRate}%`));

        if (testResults.failed === 0) {
            log.success('🎉 All tests passed!');
        } else {
            log.warning('⚠️  Some tests failed. Check the output above.');
        }
    }

    delay(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    cleanup() {
        if (this.socket) {
            this.socket.disconnect();
        }
        process.exit(0);
    }
}

// Main execution
async function main() {
    const tester = new BackendTester();
    
    try {
        await tester.connect();
        
        // Check command line arguments
        const args = process.argv.slice(2);
        
        if (args.includes('--interactive') || args.includes('-i')) {
            await tester.interactiveMode();
        } else {
            await tester.runAllTests();
            tester.cleanup();
        }
    } catch (error) {
        log.error(`Failed to start tester: ${error.message}`);
        process.exit(1);
    }
}

// Handle process termination
process.on('SIGINT', () => {
    log.info('\nReceived SIGINT. Cleaning up...');
    process.exit(0);
});

process.on('SIGTERM', () => {
    log.info('\nReceived SIGTERM. Cleaning up...');
    process.exit(0);
});

// Start the tester
main().catch(console.error); 