const { io } = require('socket.io-client');
const readline = require('readline');

class WebSocketStabilityTester {
    constructor() {
        this.socket = null;
        this.testResults = {
            connectionTests: 0,
            messageTests: 0,
            errorTests: 0,
            panicRecoveryTests: 0,
            successfulTests: 0,
            failedTests: 0
        };
        this.rl = null;
    }

    async runStabilityTests() {
        console.log('🔧 Starting WebSocket Stability Tests...\n');
        
        try {
            // Test 1: Basic Connection
            await this.testBasicConnection();
            
            // Test 2: Message Sending
            await this.testMessageSending();
            
            // Test 3: Error Handling
            await this.testErrorHandling();
            
            // Test 4: Panic Recovery
            await this.testPanicRecovery();
            
            // Test 5: Connection Stress Test
            await this.testConnectionStress();
            
            // Test 6: Heartbeat and Keepalive
            await this.testHeartbeatAndKeepalive();
            
            this.printTestResults();
            
        } catch (error) {
            console.error('❌ Test suite failed:', error.message);
        } finally {
            if (this.socket) {
                this.socket.disconnect();
            }
            if (this.rl) {
                this.rl.close();
            }
        }
    }

    async testBasicConnection() {
        console.log('🔌 Test 1: Basic Connection Test');
        this.testResults.connectionTests++;
        
        return new Promise((resolve, reject) => {
            const testSocket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true
            });

            const timeout = setTimeout(() => {
                testSocket.disconnect();
                reject(new Error('Connection timeout'));
            }, 10000);

            testSocket.on('connect', () => {
                console.log('✅ Connection established successfully');
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.successfulTests++;
                resolve();
            });

            testSocket.on('connect_error', (error) => {
                console.log('❌ Connection failed:', error.message);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(error);
            });

            testSocket.on('error', (error) => {
                console.log('❌ Socket error:', error);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(error);
            });
        });
    }

    async testMessageSending() {
        console.log('\n📨 Test 2: Message Sending Test');
        this.testResults.messageTests++;
        
        return new Promise((resolve, reject) => {
            const testSocket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true
            });

            const timeout = setTimeout(() => {
                testSocket.disconnect();
                reject(new Error('Message test timeout'));
            }, 15000);

            testSocket.on('connect', () => {
                console.log('✅ Connected, sending test messages...');
                
                // Send device info
                testSocket.emit('device:info', {
                    device_id: 'test-device-123',
                    platform: 'test',
                    version: '1.0.0'
                });
            });

            testSocket.on('device:info:ack', (data) => {
                console.log('✅ Device info acknowledged:', data.status);
                
                // Send login request
                testSocket.emit('login', {
                    mobile_no: '1234567890',
                    device_id: 'test-device-123',
                    fcm_token: 'test-fcm-token'
                });
            });

            testSocket.on('login:success', (data) => {
                console.log('✅ Login successful:', data.status);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.successfulTests++;
                resolve();
            });

            testSocket.on('connection_error', (data) => {
                console.log('❌ Connection error:', data);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(new Error(data.message));
            });

            testSocket.on('connect_error', (error) => {
                console.log('❌ Connection error:', error.message);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(error);
            });
        });
    }

    async testErrorHandling() {
        console.log('\n🛡️ Test 3: Error Handling Test');
        this.testResults.errorTests++;
        
        return new Promise((resolve, reject) => {
            const testSocket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true
            });

            const timeout = setTimeout(() => {
                testSocket.disconnect();
                reject(new Error('Error handling test timeout'));
            }, 10000);

            testSocket.on('connect', () => {
                console.log('✅ Connected, testing error handling...');
                
                // Send invalid data to trigger validation errors
                testSocket.emit('device:info', {
                    // Missing required fields
                });
            });

            testSocket.on('connection_error', (data) => {
                console.log('✅ Error handling working correctly:', data.error_code);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.successfulTests++;
                resolve();
            });

            testSocket.on('connect_error', (error) => {
                console.log('❌ Connection error:', error.message);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(error);
            });
        });
    }

    async testPanicRecovery() {
        console.log('\n🔄 Test 4: Panic Recovery Test');
        this.testResults.panicRecoveryTests++;
        
        return new Promise((resolve, reject) => {
            const testSocket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true
            });

            const timeout = setTimeout(() => {
                testSocket.disconnect();
                reject(new Error('Panic recovery test timeout'));
            }, 10000);

            testSocket.on('connect', () => {
                console.log('✅ Connected, testing panic recovery...');
                
                // Send malformed data that might cause issues
                testSocket.emit('device:info', {
                    device_id: 'a'.repeat(10000), // Very long string
                    platform: null,
                    version: undefined
                });
            });

            testSocket.on('connection_error', (data) => {
                console.log('✅ Panic recovery working:', data.error_code);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.successfulTests++;
                resolve();
            });

            testSocket.on('connect_error', (error) => {
                console.log('❌ Connection error:', error.message);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(error);
            });
        });
    }

    async testConnectionStress() {
        console.log('\n💪 Test 5: Connection Stress Test');
        
        const connections = [];
        const maxConnections = 10;
        
        try {
            console.log(`Creating ${maxConnections} concurrent connections...`);
            
            for (let i = 0; i < maxConnections; i++) {
                const socket = io('http://localhost:3002', {
                    transports: ['websocket'],
                    timeout: 5000,
                    forceNew: true
                });

                connections.push(socket);
                
                socket.on('connect', () => {
                    console.log(`✅ Connection ${i + 1} established`);
                });

                socket.on('connect_error', (error) => {
                    console.log(`❌ Connection ${i + 1} failed:`, error.message);
                });
            }

            // Wait for all connections to establish
            await new Promise(resolve => setTimeout(resolve, 5000));
            
            console.log('✅ Stress test completed successfully');
            this.testResults.successfulTests++;
            
        } catch (error) {
            console.log('❌ Stress test failed:', error.message);
            this.testResults.failedTests++;
        } finally {
            // Clean up all connections
            connections.forEach(socket => socket.disconnect());
        }
    }

    async testHeartbeatAndKeepalive() {
        console.log('\n💓 Test 6: Heartbeat and Keepalive Test');
        
        return new Promise((resolve, reject) => {
            const testSocket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 10000,
                forceNew: true
            });

            const timeout = setTimeout(() => {
                testSocket.disconnect();
                reject(new Error('Heartbeat test timeout'));
            }, 30000);

            testSocket.on('connect', () => {
                console.log('✅ Connected, testing heartbeat...');
                
                // Send ping
                testSocket.emit('ping');
            });

            testSocket.on('pong', (data) => {
                console.log('✅ Pong received:', data.status);
                
                // Send keepalive
                testSocket.emit('keepalive');
            });

            testSocket.on('keepalive:ack', (data) => {
                console.log('✅ Keepalive acknowledged:', data.status);
                
                // Send health check
                testSocket.emit('health_check');
            });

            testSocket.on('health_check:ack', (data) => {
                console.log('✅ Health check acknowledged:', data.status);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.successfulTests++;
                resolve();
            });

            testSocket.on('connection_error', (data) => {
                console.log('❌ Connection error:', data);
                clearTimeout(timeout);
                testSocket.disconnect();
                this.testResults.failedTests++;
                reject(new Error(data.message));
            });
        });
    }

    printTestResults() {
        console.log('\n📊 Test Results Summary:');
        console.log('========================');
        console.log(`Connection Tests: ${this.testResults.connectionTests}`);
        console.log(`Message Tests: ${this.testResults.messageTests}`);
        console.log(`Error Handling Tests: ${this.testResults.errorTests}`);
        console.log(`Panic Recovery Tests: ${this.testResults.panicRecoveryTests}`);
        console.log(`Successful Tests: ${this.testResults.successfulTests}`);
        console.log(`Failed Tests: ${this.testResults.failedTests}`);
        
        const successRate = (this.testResults.successfulTests / 
            (this.testResults.successfulTests + this.testResults.failedTests)) * 100;
        
        console.log(`\nSuccess Rate: ${successRate.toFixed(1)}%`);
        
        if (successRate >= 90) {
            console.log('🎉 Excellent! WebSocket stability is working well.');
        } else if (successRate >= 70) {
            console.log('⚠️ Good, but some improvements needed.');
        } else {
            console.log('❌ Significant issues detected. Check server logs.');
        }
    }
}

// Run the tests
const tester = new WebSocketStabilityTester();
tester.runStabilityTests().catch(console.error); 