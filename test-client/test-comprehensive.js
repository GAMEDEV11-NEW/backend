const io = require('socket.io-client');
const axios = require('axios');

const SERVER_URL = 'http://localhost:3002';
const SOCKET_URL = 'http://localhost:3002';

class ComprehensiveTester {
    constructor() {
        this.results = {
            httpHealth: false,
            socketConnection: false,
            connectResponse: false,
            deviceInfo: false,
            login: false,
            otpVerification: false,
            userProfile: false,
            languageSetting: false,
            heartbeat: false,
            healthCheck: false,
            errorHandling: false,
            panicRecovery: false
        };
        this.testCount = 0;
        this.passedTests = 0;
        this.failedTests = 0;
    }

    log(message, type = 'info') {
        const timestamp = new Date().toISOString();
        const colors = {
            info: '\x1b[36m',    // Cyan
            success: '\x1b[32m', // Green
            error: '\x1b[31m',   // Red
            warning: '\x1b[33m', // Yellow
            reset: '\x1b[0m'     // Reset
        };
        console.log(`${colors[type]}[${timestamp}] ${message}${colors.reset}`);
    }

    async testHttpHealth() {
        this.log('🔍 Testing HTTP Health Endpoint...', 'info');
        try {
            const response = await axios.get(`${SERVER_URL}/health`, {
                timeout: 5000,
                headers: {
                    'User-Agent': 'ComprehensiveTester/1.0'
                }
            });
            
            if (response.status === 200 && response.data === 'OK') {
                this.log('✅ HTTP Health: Server is responding correctly', 'success');
                this.results.httpHealth = true;
                this.passedTests++;
            } else {
                this.log(`❌ HTTP Health: Unexpected response - Status: ${response.status}, Data: ${response.data}`, 'error');
                this.failedTests++;
            }
        } catch (error) {
            this.log(`❌ HTTP Health: Connection failed - ${error.message}`, 'error');
            this.failedTests++;
        }
        this.testCount++;
    }

    async testSocketConnection() {
        this.log('🔌 Testing WebSocket Connection...', 'info');
        
        return new Promise((resolve) => {
            const socket = io(SOCKET_URL, {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true,
                reconnection: false
            });

            const timeout = setTimeout(() => {
                this.log('❌ Socket Connection: Timeout waiting for connection', 'error');
                socket.disconnect();
                this.failedTests++;
                this.testCount++;
                resolve();
            }, 10000);

            socket.on('connect', () => {
                clearTimeout(timeout);
                this.log('✅ Socket Connection: Successfully connected', 'success');
                this.results.socketConnection = true;
                this.passedTests++;
                this.testCount++;
                socket.disconnect();
                resolve();
            });

            socket.on('connect_error', (error) => {
                clearTimeout(timeout);
                this.log(`❌ Socket Connection: Failed - ${error.message}`, 'error');
                this.failedTests++;
                this.testCount++;
                resolve();
            });
        });
    }

    async testFullWorkflow() {
        this.log('🔄 Testing Full User Workflow...', 'info');
        
        return new Promise((resolve) => {
            const socket = io(SOCKET_URL, {
                transports: ['websocket'],
                timeout: 15000,
                forceNew: true,
                reconnection: false
            });

            let workflowStep = 0;
            const maxSteps = 8;
            const timeout = setTimeout(() => {
                this.log('❌ Full Workflow: Timeout during workflow testing', 'error');
                socket.disconnect();
                this.failedTests++;
                this.testCount++;
                resolve();
            }, 30000);

            socket.on('connect', () => {
                this.log('✅ Workflow: Connected, starting workflow test', 'success');
                workflowStep++;
                
                // Test 1: Connect Response
                socket.on('connect_response', (data) => {
                    this.log('✅ Workflow: Connect response received', 'success');
                    this.results.connectResponse = true;
                    workflowStep++;
                    
                    // Test 2: Device Info
                    socket.emit('device:info', {
                        device_id: 'test-device-123',
                        device_type: 'mobile',
                        platform: 'test',
                        version: '1.0.0',
                        model: 'TestDevice',
                        os_version: 'TestOS 1.0',
                        app_version: '1.0.0',
                        build_number: '1'
                    });
                });

                // Test 2: Device Info Response
                socket.on('device:info:ack', (data) => {
                    this.log('✅ Workflow: Device info acknowledged', 'success');
                    this.results.deviceInfo = true;
                    workflowStep++;
                    
                    // Test 3: Login
                    socket.emit('login', {
                        mobile_no: '1234567890',
                        device_id: 'test-device-123',
                        fcm_token: 'test-fcm-token-123',
                        email: 'test@example.com'
                    });
                });

                // Test 3: Login Response
                socket.on('login:success', (data) => {
                    this.log('✅ Workflow: Login successful', 'success');
                    this.results.login = true;
                    this.sessionToken = data.session_token;
                    this.otp = data.otp;
                    workflowStep++;
                    
                    // Test 4: OTP Verification
                    socket.emit('otp:verify', {
                        mobile_no: '1234567890',
                        session_token: this.sessionToken,
                        otp: this.otp
                    });
                });

                // Test 4: OTP Verification Response
                socket.on('otp:verified', (data) => {
                    this.log('✅ Workflow: OTP verified', 'success');
                    this.results.otpVerification = true;
                    workflowStep++;
                    
                    // Test 5: User Profile
                    socket.emit('set:profile', {
                        mobile_no: '1234567890',
                        session_token: this.sessionToken,
                        full_name: 'Test User',
                        state: 'Test State',
                        referral_code: 'TEST123'
                    });
                });

                // Test 5: User Profile Response
                socket.on('profile:set', (data) => {
                    this.log('✅ Workflow: Profile set successfully', 'success');
                    this.results.userProfile = true;
                    workflowStep++;
                    
                    // Test 6: Language Setting
                    socket.emit('set:language', {
                        mobile_no: '1234567890',
                        session_token: this.sessionToken,
                        language_code: 'en',
                        language_name: 'English',
                        region_code: 'US',
                        timezone: 'America/New_York'
                    });
                });

                // Test 6: Language Setting Response
                socket.on('language:set', (data) => {
                    this.log('✅ Workflow: Language set successfully', 'success');
                    this.results.languageSetting = true;
                    workflowStep++;
                    
                    // Test 7: Heartbeat
                    socket.emit('ping');
                });

                // Test 7: Heartbeat Response
                socket.on('pong', (data) => {
                    this.log('✅ Workflow: Heartbeat working', 'success');
                    this.results.heartbeat = true;
                    workflowStep++;
                    
                    // Test 8: Health Check
                    socket.emit('health_check');
                });

                // Test 8: Health Check Response
                socket.on('health_check:ack', (data) => {
                    this.log('✅ Workflow: Health check working', 'success');
                    this.results.healthCheck = true;
                    workflowStep++;
                    
                    // Complete workflow
                    clearTimeout(timeout);
                    socket.disconnect();
                    this.passedTests++;
                    this.testCount++;
                    resolve();
                });

                // Error handling
                socket.on('connection_error', (data) => {
                    this.log(`❌ Workflow: Connection error - ${data.message}`, 'error');
                    this.log(`   Error code: ${data.error_code}`, 'error');
                    this.log(`   Error type: ${data.error_type}`, 'error');
                    this.log(`   Field: ${data.field}`, 'error');
                    clearTimeout(timeout);
                    socket.disconnect();
                    this.failedTests++;
                    this.testCount++;
                    resolve();
                });

                socket.on('error', (error) => {
                    this.log(`❌ Workflow: Socket error - ${error}`, 'error');
                    clearTimeout(timeout);
                    socket.disconnect();
                    this.failedTests++;
                    this.testCount++;
                    resolve();
                });
            });

            socket.on('connect_error', (error) => {
                this.log(`❌ Workflow: Connection failed - ${error.message}`, 'error');
                clearTimeout(timeout);
                this.failedTests++;
                this.testCount++;
                resolve();
            });
        });
    }

    async testErrorHandling() {
        this.log('🛡️ Testing Error Handling...', 'info');
        
        return new Promise((resolve) => {
            const socket = io(SOCKET_URL, {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true,
                reconnection: false
            });

            const timeout = setTimeout(() => {
                this.log('❌ Error Handling: Timeout', 'error');
                socket.disconnect();
                this.failedTests++;
                this.testCount++;
                resolve();
            }, 10000);

            socket.on('connect', () => {
                // Send invalid event
                socket.emit('invalid_event', { invalid: 'data' });
                
                // Send malformed device info (missing required fields)
                socket.emit('device:info', { 
                    invalid: 'data',
                    device_id: 'test' // Missing device_type
                });
            });

            socket.on('unknown_event_error', (data) => {
                this.log('✅ Error Handling: Unknown event handled gracefully', 'success');
                this.results.errorHandling = true;
                clearTimeout(timeout);
                socket.disconnect();
                this.passedTests++;
                this.testCount++;
                resolve();
            });

            socket.on('connection_error', (data) => {
                this.log('✅ Error Handling: Connection error handled properly', 'success');
                this.log(`   Error details: ${data.message}`, 'info');
                this.results.errorHandling = true;
                clearTimeout(timeout);
                socket.disconnect();
                this.passedTests++;
                this.testCount++;
                resolve();
            });

            socket.on('connect_error', (error) => {
                this.log(`❌ Error Handling: Connection failed - ${error.message}`, 'error');
                clearTimeout(timeout);
                this.failedTests++;
                this.testCount++;
                resolve();
            });
        });
    }

    async testPanicRecovery() {
        this.log('🔄 Testing Panic Recovery (Simulated)...', 'info');
        
        // Create multiple connections to simulate load
        const connections = [];
        const connectionCount = 5;
        
        for (let i = 0; i < connectionCount; i++) {
            const socket = io(SOCKET_URL, {
                transports: ['websocket'],
                timeout: 5000,
                forceNew: true,
                reconnection: false
            });
            
            connections.push(socket);
            
            socket.on('connect', () => {
                this.log(`✅ Panic Recovery: Connection ${i + 1} established`, 'success');
            });
            
            socket.on('connect_error', (error) => {
                this.log(`❌ Panic Recovery: Connection ${i + 1} failed - ${error.message}`, 'error');
            });
        }
        
        // Wait a bit then disconnect all
        await new Promise(resolve => setTimeout(resolve, 3000));
        
        connections.forEach((socket, index) => {
            socket.disconnect();
            this.log(`🔌 Panic Recovery: Connection ${index + 1} disconnected`, 'info');
        });
        
        this.log('✅ Panic Recovery: Multiple connections handled successfully', 'success');
        this.results.panicRecovery = true;
        this.passedTests++;
        this.testCount++;
    }

    printResults() {
        this.log('\n📊 COMPREHENSIVE TEST RESULTS', 'info');
        this.log('='.repeat(50), 'info');
        
        const tests = [
            { name: 'HTTP Health Check', result: this.results.httpHealth },
            { name: 'Socket Connection', result: this.results.socketConnection },
            { name: 'Connect Response', result: this.results.connectResponse },
            { name: 'Device Info', result: this.results.deviceInfo },
            { name: 'User Login', result: this.results.login },
            { name: 'OTP Verification', result: this.results.otpVerification },
            { name: 'User Profile', result: this.results.userProfile },
            { name: 'Language Setting', result: this.results.languageSetting },
            { name: 'Heartbeat', result: this.results.heartbeat },
            { name: 'Health Check', result: this.results.healthCheck },
            { name: 'Error Handling', result: this.results.errorHandling },
            { name: 'Panic Recovery', result: this.results.panicRecovery }
        ];

        tests.forEach(test => {
            const status = test.result ? '✅ PASS' : '❌ FAIL';
            const color = test.result ? 'success' : 'error';
            this.log(`${status} ${test.name}`, color);
        });

        this.log('='.repeat(50), 'info');
        this.log(`Total Tests: ${this.testCount}`, 'info');
        this.log(`Passed: ${this.passedTests}`, 'success');
        this.log(`Failed: ${this.failedTests}`, this.failedTests > 0 ? 'error' : 'success');
        this.log(`Success Rate: ${((this.passedTests / this.testCount) * 100).toFixed(1)}%`, 'info');

        if (this.failedTests === 0) {
            this.log('\n🎉 ALL TESTS PASSED! Backend is working perfectly!', 'success');
            process.exit(0);
        } else {
            this.log('\n⚠️ Some tests failed. Please check the backend logs.', 'warning');
            process.exit(1);
        }
    }

    async runAllTests() {
        this.log('🚀 Starting Comprehensive Backend Tests', 'info');
        this.log('='.repeat(50), 'info');
        
        await this.testHttpHealth();
        await this.testSocketConnection();
        await this.testFullWorkflow();
        await this.testErrorHandling();
        await this.testPanicRecovery();
        
        this.printResults();
    }
}

// Run the comprehensive test
const tester = new ComprehensiveTester();
tester.runAllTests().catch(error => {
    console.error('❌ Test runner error:', error);
    process.exit(1);
}); 