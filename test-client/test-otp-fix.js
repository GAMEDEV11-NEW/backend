const io = require('socket.io-client');

class OtpVerificationTest {
    constructor() {
        this.socket = null;
        this.mobileNo = '8888995555';
        this.deviceId = 'test-device-123';
        this.fcmToken = 'test-fcm-token-456';
        this.sessionToken = null;
        this.otp = null;
        this.testResults = [];
    }

    async connect() {
        return new Promise((resolve, reject) => {
            console.log('🔌 Connecting to server...');
            this.socket = io('http://localhost:3000');
            
            this.socket.on('connect', () => {
                console.log('✅ Connected to server');
                resolve();
            });
            
            this.socket.on('connect_error', (error) => {
                console.log('❌ Connection failed:', error);
                reject(error);
            });
            
            setTimeout(() => reject(new Error('Connection timeout')), 5000);
        });
    }

    async login() {
        return new Promise((resolve, reject) => {
            console.log('🔐 Sending login request...');
            
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
                this.otp = data.otp.toString();
                console.log(`📱 Received OTP: ${this.otp}`);
                resolve(data);
            });

            this.socket.once('connection_error', (error) => {
                console.log('❌ Login failed:', error);
                reject(error);
            });

            setTimeout(() => reject(new Error('Login timeout')), 10000);
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

            this.socket.once('otp:verification_failed', (error) => {
                console.log('❌ OTP verification failed:', error);
                resolve({ success: false, error });
            });

            setTimeout(() => reject(new Error('OTP verification timeout')), 10000);
        });
    }

    async runTests() {
        console.log('🧪 Starting OTP Verification Fix Tests...\n');

        try {
            // Test 1: Connect to server
            await this.connect();
            this.testResults.push({ test: 'Connection', status: 'PASS' });

            // Test 2: Login to get OTP
            await this.login();
            this.testResults.push({ test: 'Login', status: 'PASS' });

            // Test 3: Verify with correct OTP
            console.log('\n--- Test 3: Correct OTP Verification ---');
            const correctResult = await this.verifyOtp(this.otp);
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

            // Test 5: Test rate limiting
            console.log('\n--- Test 5: Rate Limiting Test ---');
            let attempts = 0;
            const maxAttempts = 6;
            let rateLimitHit = false;

            for (let i = 0; i < maxAttempts; i++) {
                const result = await this.verifyOtp('999999');
                attempts++;
                
                if (result.error && result.error.error_code === 'RATE_LIMIT_EXCEEDED') {
                    rateLimitHit = true;
                    console.log(`🚫 Rate limit hit after ${attempts} attempts`);
                    break;
                }
            }

            if (rateLimitHit) {
                this.testResults.push({ test: 'Rate Limiting', status: 'PASS' });
            } else {
                this.testResults.push({ test: 'Rate Limiting', status: 'FAIL', message: 'Rate limit not enforced' });
            }

            // Test 6: Test with invalid session token
            console.log('\n--- Test 6: Invalid Session Token ---');
            const invalidSessionData = {
                mobile_no: this.mobileNo,
                otp: this.otp,
                session_token: 'invalid-session-token',
                timestamp: new Date().toISOString()
            };

            this.socket.emit('verify:otp', invalidSessionData);
            
            const invalidSessionResult = await new Promise((resolve) => {
                this.socket.once('otp:verification_failed', (error) => {
                    resolve({ success: false, error });
                });
                setTimeout(() => resolve({ success: true, error: null }), 5000);
            });

            if (!invalidSessionResult.success && 
                (invalidSessionResult.error.error_code === 'SESSION_NOT_FOUND' || 
                 invalidSessionResult.error.error_code === 'INVALID_OTP')) {
                this.testResults.push({ test: 'Invalid Session', status: 'PASS' });
            } else {
                this.testResults.push({ test: 'Invalid Session', status: 'FAIL', error: invalidSessionResult.error });
            }

        } catch (error) {
            console.error('❌ Test failed:', error);
            this.testResults.push({ test: 'Overall Test', status: 'FAIL', error: error.message });
        } finally {
            this.disconnect();
            this.printResults();
        }
    }

    disconnect() {
        if (this.socket) {
            this.socket.disconnect();
            console.log('🔌 Disconnected from server');
        }
    }

    printResults() {
        console.log('\n📊 Test Results:');
        console.log('================');
        
        let passed = 0;
        let failed = 0;
        
        this.testResults.forEach((result, index) => {
            const status = result.status === 'PASS' ? '✅' : '❌';
            console.log(`${status} Test ${index + 1}: ${result.test} - ${result.status}`);
            
            if (result.status === 'PASS') {
                passed++;
            } else {
                failed++;
                if (result.error) {
                    console.log(`   Error: ${JSON.stringify(result.error, null, 2)}`);
                }
                if (result.message) {
                    console.log(`   Message: ${result.message}`);
                }
            }
        });
        
        console.log(`\n📈 Summary: ${passed} passed, ${failed} failed`);
        
        if (failed === 0) {
            console.log('🎉 All tests passed! OTP verification fix is working correctly.');
        } else {
            console.log('⚠️ Some tests failed. Please check the implementation.');
        }
    }
}

// Run the tests
const test = new OtpVerificationTest();
test.runTests().catch(console.error); 