const io = require('socket.io-client');
const chalk = require('chalk');

// Beautiful console styling
const colors = {
    primary: chalk.cyan.bold,
    success: chalk.green.bold,
    error: chalk.red.bold,
    warning: chalk.yellow.bold,
    info: chalk.blue,
    highlight: chalk.magenta
};

const log = {
    success: (msg) => console.log(colors.success('✅ ' + msg)),
    error: (msg) => console.log(colors.error('❌ ' + msg)),
    warning: (msg) => console.log(colors.warning('⚠️ ' + msg)),
    info: (msg) => console.log(colors.info('ℹ️ ' + msg)),
    highlight: (msg) => console.log(colors.highlight('🎯 ' + msg))
};

console.log(colors.primary('🔍 Testing Device Info Validation'));
console.log(colors.primary('='.repeat(40)));

// Test data
const validDeviceInfo = {
    device_id: "test-device-001",
    device_type: "game-console",
    manufacturer: "TestCorp",
    model: "GameStation Pro",
    firmware_version: "2.1.0",
    capabilities: ["multiplayer", "streaming", "voice-chat"],
    timestamp: new Date().toISOString()
};

const minimalValidDeviceInfo = {
    device_id: "test-device-002",
    device_type: "game-console",
    timestamp: new Date().toISOString()
    // No optional fields - should still be valid
};

const invalidDeviceInfo = {
    device_id: "", // Empty device_id (required field)
    device_type: "game-console",
    manufacturer: "TestCorp",
    model: "GameStation Pro",
    firmware_version: "2.1.0",
    capabilities: ["multiplayer"],
    timestamp: "invalid-timestamp" // Invalid timestamp (required field)
};

const missingRequiredFieldsDeviceInfo = {
    device_id: "test-device-003",
    // Missing device_type (required)
    // Missing timestamp (required)
    manufacturer: "TestCorp" // Optional field
};

const emptyOptionalFieldsDeviceInfo = {
    device_id: "test-device-004",
    device_type: "game-console",
    manufacturer: "", // Empty optional field - should fail
    timestamp: new Date().toISOString()
};

class ValidationTester {
    constructor() {
        this.socket = null;
        this.testResults = [];
    }

    async connect() {
        return new Promise((resolve, reject) => {
            this.socket = io('http://localhost:3002');

            this.socket.on('connect', () => {
                log.success('Connected to server');
                resolve();
            });

            this.socket.on('connect_error', (error) => {
                log.error(`Connection failed: ${error.message}`);
                reject(error);
            });

            this.setupEventListeners();
        });
    }

    setupEventListeners() {
        // Connect response
        this.socket.on('connect_response', (response) => {
            log.info(`Connected with token: ${response.token}`);
        });

        // Connection error
        this.socket.on('connection_error', (response) => {
            log.error(`Validation failed: ${response.message}`);
            log.info(`Error Code: ${response.error_code}`);
            log.info(`Error Type: ${response.error_type}`);
            log.info(`Field: ${response.field}`);
            log.info(`Details:`, response.details);
            
            this.testResults.push({
                test: 'Invalid Data Test',
                status: 'FAIL',
                message: response.message,
                error_code: response.error_code,
                field: response.field,
                details: response.details
            });
        });

        // Device info acknowledgment
        this.socket.on('device:info:ack', (response) => {
            log.success(`Validation passed: ${response.message}`);
            this.testResults.push({
                test: 'Valid Data Test',
                status: 'PASS',
                message: response.message
            });
        });
    }

    async testValidData() {
        log.highlight('\n🧪 Testing Valid Device Info...');
        this.socket.emit('device:info', validDeviceInfo);
        
        return new Promise((resolve) => {
            setTimeout(() => {
                if (!this.testResults.find(r => r.test === 'Valid Data Test')) {
                    this.testResults.push({
                        test: 'Valid Data Test',
                        status: 'FAIL',
                        message: 'No response received'
                    });
                }
                resolve();
            }, 2000);
        });
    }

    async testInvalidData() {
        log.highlight('\n🧪 Testing Invalid Device Info...');
        this.socket.emit('device:info', invalidDeviceInfo);
        
        return new Promise((resolve) => {
            setTimeout(() => {
                if (!this.testResults.find(r => r.test === 'Invalid Data Test')) {
                    this.testResults.push({
                        test: 'Invalid Data Test',
                        status: 'FAIL',
                        message: 'No error response received'
                    });
                }
                resolve();
            }, 2000);
        });
    }

    async testMissingFields() {
        log.highlight('\n🧪 Testing Missing Fields...');
        this.socket.emit('device:info', missingRequiredFieldsDeviceInfo);
        
        return new Promise((resolve) => {
            setTimeout(() => {
                if (!this.testResults.find(r => r.test === 'Missing Fields Test')) {
                    this.testResults.push({
                        test: 'Missing Fields Test',
                        status: 'FAIL',
                        message: 'No error response received'
                    });
                }
                resolve();
            }, 2000);
        });
    }

    async testMinimalValidData() {
        log.highlight('\n🧪 Testing Minimal Valid Device Info (Only Required Fields)...');
        this.socket.emit('device:info', minimalValidDeviceInfo);
        
        return new Promise((resolve) => {
            setTimeout(() => {
                if (!this.testResults.find(r => r.test === 'Minimal Valid Data Test')) {
                    this.testResults.push({
                        test: 'Minimal Valid Data Test',
                        status: 'FAIL',
                        message: 'No response received'
                    });
                }
                resolve();
            }, 2000);
        });
    }

    async testEmptyOptionalFields() {
        log.highlight('\n🧪 Testing Empty Optional Fields...');
        this.socket.emit('device:info', emptyOptionalFieldsDeviceInfo);
        
        return new Promise((resolve) => {
            setTimeout(() => {
                if (!this.testResults.find(r => r.test === 'Empty Optional Fields Test')) {
                    this.testResults.push({
                        test: 'Empty Optional Fields Test',
                        status: 'FAIL',
                        message: 'No error response received'
                    });
                }
                resolve();
            }, 2000);
        });
    }

    showResults() {
        console.log('\n' + colors.primary('📊 Validation Test Results:'));
        console.log(colors.primary('─'.repeat(30)));
        
        this.testResults.forEach(result => {
            const icon = result.status === 'PASS' ? colors.success('✓') : colors.error('✗');
            const statusColor = result.status === 'PASS' ? colors.success : colors.error;
            console.log(`${icon} ${result.test}: ${statusColor(result.status)}`);
            console.log(`   Message: ${result.message}`);
            
            if (result.error_code) {
                console.log(`   Error Code: ${result.error_code}`);
                console.log(`   Field: ${result.field}`);
                console.log(`   Details:`, result.details);
            }
        });

        const passed = this.testResults.filter(r => r.status === 'PASS').length;
        const total = this.testResults.length;
        const successRate = ((passed / total) * 100).toFixed(1);
        
        console.log(`\n${colors.highlight(`Success Rate: ${successRate}% (${passed}/${total})`)}`);
    }

    cleanup() {
        if (this.socket) {
            this.socket.disconnect();
        }
    }
}

async function main() {
    const tester = new ValidationTester();
    
    try {
        await tester.connect();
        
        // Run validation tests
        await tester.testValidData();
        await tester.testInvalidData();
        await tester.testMissingFields();
        await tester.testMinimalValidData();
        await tester.testEmptyOptionalFields();
        
        // Show results
        tester.showResults();
        
        // Cleanup
        tester.cleanup();
        process.exit(0);
    } catch (error) {
        log.error(`Test failed: ${error.message}`);
        process.exit(1);
    }
}

// Handle process termination
process.on('SIGINT', () => {
    log.info('\nReceived SIGINT. Cleaning up...');
    process.exit(0);
});

main().catch(console.error); 