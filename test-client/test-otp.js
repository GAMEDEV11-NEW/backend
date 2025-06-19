const io = require('socket.io-client');

class OtpTestClient {
    constructor() {
        this.socket = null;
        this.mobileNo = '9876543210';
        this.deviceId = 'test-device-001';
        this.fcmToken = 'fcm_token_example_' + 'x'.repeat(100);
        this.sessionToken = null;
        this.otp = null;
        this.testResults = [];
    }

    connect() {
        return new Promise((resolve, reject) => {
            console.log('🔌 Connecting to Socket.IO server...');
            
            this.socket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 10000
            });

            this.socket.on('connect', () => {
                console.log('✅ Connected to server');
                resolve();
            });

            this.socket.on('connect_response', (data) => {
                console.log('📨 Received connect response:', data);
            });

            this.socket.on('disconnect', () => {
                console.log('🔌 Disconnected from server');
            });

            this.socket.on('error', (error) => {
                console.error('❌ Socket error:', error);
                reject(error);
            });

            // Set timeout for connection
            setTimeout(() => {
                reject(new Error('Connection timeout'));
            }, 10000);
        });
    }

    async login() {
        return new Promise((resolve, reject) => {
            console.log('🔐 Attempting login...');
            
            const loginData = {
                mobile_no: this.mobileNo,
                device_id: this.deviceId,
                fcm_token: this.fcmToken,
                timestamp: new Date().toISOString()
            };

            this.socket.emit('login', loginData);

            this.socket.once('login:success', (data) => {
                console.log('✅ Login successful:', data);
                this.sessionToken = data.session_token;
                this.otp = data.otp;
                resolve(data);
            });

            this.socket.once('connection_error', (error) => {
                console.error('❌ Login failed:', error);
                reject(new Error(error.message));
            });

            // Set timeout
            setTimeout(() => {
                reject(new Error('Login timeout'));
            }, 10000);
        });
    }

    async verifyOtp(otpToVerify) {
        return new Promise((resolve, reject) => {
            console.log(`🔢 Verifying OTP: ${otpToVerify}`);
            
            const otpData = {
                mobile_no: this.mobileNo,
                otp: otpToVerify,
                session_token: this.sessionToken,
                timestamp: new Date().toISOString()
            };

            this.socket.emit('verify:otp', otpData);

            this.socket.once('otp:verified', (data) => {
                console.log('✅ OTP verification successful:', data);
                resolve({ success: true, data });
            });

            this.socket.once('connection_error', (error) => {
                console.log('❌ OTP verification failed:', error);
                resolve({ success: false, error });
            });

            // Set timeout
            setTimeout(() => {
                reject(new Error('OTP verification timeout'));
            }, 10000);
        });
    }

    disconnect() {
        if (this.socket) {
            this.socket.disconnect();
            console.log('🔌 Disconnected from server');
        }
    }

    async runTests() {
        console.log('🧪 Starting OTP Verification Tests...\n');

        try {
            // Test 1: Connect to server
            await this.connect();
            this.testResults.push({ test: 'Connection', status: 'PASS' });

            // Test 2: Login to get OTP
            await this.login();
            this.testResults.push({ test: 'Login', status: 'PASS' });

            // Test 3: Verify with correct OTP
            console.log('\n--- Test 3: Correct OTP Verification ---');
            const correctResult = await this.verifyOtp(this.otp.toString());
            if (correctResult.success) {
                this.testResults.push({ test: 'Correct OTP', status: 'PASS' });
            } else {
                this.testResults.push({ test: 'Correct OTP', status: 'FAIL', error: correctResult.error });
            }

            // Test 4: Verify with incorrect OTP
            console.log('\n--- Test 4: Incorrect OTP Verification ---');
            const incorrectResult = await this.verifyOtp('123456');
            if (!incorrectResult.success && incorrectResult.error.error_code === 'INVALID_OTP') {
                this.testResults.push({ test: 'Incorrect OTP', status: 'PASS' });
            } else {
                this.testResults.push({ test: 'Incorrect OTP', status: 'FAIL', error: incorrectResult.error });
            }

            // Test 5: Multiple incorrect attempts
            console.log('\n--- Test 5: Multiple Incorrect Attempts ---');
            let attempts = 0;
            const maxAttempts = 5;
            
            for (let i = 0; i < maxAttempts; i++) {
                attempts++;
                console.log(`Attempt ${attempts}/${maxAttempts}: Trying incorrect OTP...`);
                
                const result = await this.verifyOtp('000000');
                
                if (result.success) {
                    this.testResults.push({ test: 'Multiple Incorrect Attempts', status: 'FAIL', error: 'Unexpected success' });
                    break;
                } else if (result.error.error_code === 'MAX_ATTEMPTS_EXCEEDED' && attempts >= maxAttempts) {
                    this.testResults.push({ test: 'Multiple Incorrect Attempts', status: 'PASS' });
                    break;
                } else if (result.error.error_code === 'INVALID_OTP' && attempts < maxAttempts) {
                    console.log(`Attempt ${attempts} failed as expected`);
                    continue;
                } else {
                    this.testResults.push({ test: 'Multiple Incorrect Attempts', status: 'FAIL', error: result.error });
                    break;
                }
            }

            // Test 6: Invalid OTP format
            console.log('\n--- Test 6: Invalid OTP Format ---');
            const invalidFormatResult = await this.verifyOtp('12345'); // 5 digits instead of 6
            if (!invalidFormatResult.success && invalidFormatResult.error.error_code === 'INVALID_LENGTH') {
                this.testResults.push({ test: 'Invalid OTP Format', status: 'PASS' });
            } else {
                this.testResults.push({ test: 'Invalid OTP Format', status: 'FAIL', error: invalidFormatResult.error });
            }

            // Test 7: Missing required fields
            console.log('\n--- Test 7: Missing Required Fields ---');
            const missingFieldsResult = await this.verifyOtpWithMissingFields();
            if (!missingFieldsResult.success && missingFieldsResult.error.error_code === 'MISSING_FIELD') {
                this.testResults.push({ test: 'Missing Required Fields', status: 'PASS' });
            } else {
                this.testResults.push({ test: 'Missing Required Fields', status: 'FAIL', error: missingFieldsResult.error });
            }

        } catch (error) {
            console.error('❌ Test execution failed:', error.message);
            this.testResults.push({ test: 'Test Execution', status: 'FAIL', error: error.message });
        } finally {
            this.disconnect();
            this.printResults();
        }
    }

    async verifyOtpWithMissingFields() {
        return new Promise((resolve, reject) => {
            console.log('🔢 Verifying OTP with missing fields...');
            
            const otpData = {
                mobile_no: this.mobileNo,
                // Missing otp and session_token
                timestamp: new Date().toISOString()
            };

            this.socket.emit('verify:otp', otpData);

            this.socket.once('otp:verified', (data) => {
                resolve({ success: true, data });
            });

            this.socket.once('connection_error', (error) => {
                resolve({ success: false, error });
            });

            setTimeout(() => {
                reject(new Error('Missing fields test timeout'));
            }, 10000);
        });
    }

    printResults() {
        console.log('\n📊 Test Results Summary:');
        console.log('========================');
        
        let passed = 0;
        let failed = 0;
        
        this.testResults.forEach(result => {
            const status = result.status === 'PASS' ? '✅' : '❌';
            console.log(`${status} ${result.test}: ${result.status}`);
            
            if (result.error) {
                console.log(`   Error: ${result.error.message || result.error}`);
            }
            
            if (result.status === 'PASS') {
                passed++;
            } else {
                failed++;
            }
        });
        
        console.log('\n📈 Summary:');
        console.log(`Total Tests: ${this.testResults.length}`);
        console.log(`Passed: ${passed}`);
        console.log(`Failed: ${failed}`);
        console.log(`Success Rate: ${((passed / this.testResults.length) * 100).toFixed(1)}%`);
        
        if (failed === 0) {
            console.log('\n🎉 All tests passed! OTP verification system is working correctly.');
        } else {
            console.log('\n⚠️ Some tests failed. Please check the implementation.');
        }
    }
}

// Run the tests
async function runOtpTests() {
    const client = new OtpTestClient();
    await client.runTests();
}

// Export for use in other test files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { OtpTestClient, runOtpTests };
}

// Run tests if this file is executed directly
if (require.main === module) {
    runOtpTests().catch(console.error);
} 