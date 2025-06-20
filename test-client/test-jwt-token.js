const io = require('socket.io-client');

// Test JWT token functionality
async function testJwtToken() {
    console.log('🔐 Testing JWT Token Functionality...\n');

    const socket = io('http://localhost:3002', {
        transports: ['websocket'],
        timeout: 10000
    });

    socket.on('connect', () => {
        console.log('✅ Connected to server');
        console.log('📱 Socket ID:', socket.id);
        
        // Test device info
        const deviceInfo = {
            device_id: 'test-device-jwt-001',
            device_type: 'mobile',
            manufacturer: 'Test Manufacturer',
            model: 'Test Model',
            firmware_version: '1.0.0',
            capabilities: ['push_notifications', 'location']
        };
        
        socket.emit('device:info', deviceInfo);
    });

    socket.on('device:info:ack', (data) => {
        console.log('📱 Device info acknowledged:', data);
        
        // Test login
        const loginData = {
            mobile_no: '8888855555',
            device_id: 'test-device-jwt-001',
            fcm_token: 'test-fcm-token-jwt-123',
            email: 'test@example.com'
        };
        
        socket.emit('login', loginData);
    });

    socket.on('login:success', (data) => {
        console.log('🔐 Login successful:', {
            mobile_no: data.mobile_no,
            device_id: data.device_id,
            session_token: data.session_token,
            otp: data.otp,
            is_new_user: data.is_new_user
        });
        
        // Test OTP verification
        const otpData = {
            mobile_no: '8888855555',
            device_id: 'test-device-jwt-001',
            fcm_token: 'test-fcm-token-jwt-123',
            session_token: data.session_token,
            otp: data.otp.toString()
        };
        
        socket.emit('verify:otp', otpData);
    });

    socket.on('otp:verified', (data) => {
        console.log('✅ OTP verification successful!');
        console.log('🔐 JWT Token Details:');
        console.log('   - User ID (UUID v7):', data.user_id);
        console.log('   - User Number:', data.user_number);
        console.log('   - Mobile Number:', data.mobile_no);
        console.log('   - Device ID:', data.device_id);
        console.log('   - User Status:', data.user_status);
        console.log('   - JWT Token Type:', data.token_type);
        console.log('   - Token Expires In:', data.expires_in, 'seconds');
        console.log('   - JWT Token (first 50 chars):', data.jwt_token.substring(0, 50) + '...');
        
        // Test JWT token validation by sending it back
        testJwtValidation(socket, data.jwt_token, data.mobile_no, data.device_id);
    });

    socket.on('otp:verification_failed', (data) => {
        console.log('❌ OTP verification failed:', data);
        socket.disconnect();
    });

    socket.on('connection_error', (data) => {
        console.log('❌ Connection error:', data);
        socket.disconnect();
    });

    socket.on('system_error', (data) => {
        console.log('💥 System error:', data);
        socket.disconnect();
    });

    socket.on('disconnect', () => {
        console.log('🔌 Disconnected from server');
    });

    socket.on('error', (error) => {
        console.log('⚠️ Socket error:', error);
    });
}

// Test JWT token validation
function testJwtValidation(socket, jwtToken, mobileNo, deviceId) {
    console.log('\n🔍 Testing JWT Token Validation...');
    
    // Simulate a request that requires JWT validation
    const validationData = {
        jwt_token: jwtToken,
        mobile_no: mobileNo,
        device_id: deviceId,
        action: 'test_validation'
    };
    
    // For this test, we'll just log the token details
    console.log('📋 JWT Token Validation Test:');
    console.log('   - Token Length:', jwtToken.length);
    console.log('   - Token Format:', jwtToken.split('.').length === 3 ? 'Valid JWT format' : 'Invalid format');
    console.log('   - Token Header:', jwtToken.split('.')[0]);
    console.log('   - Token Payload:', jwtToken.split('.')[1]);
    console.log('   - Token Signature:', jwtToken.split('.')[2].substring(0, 20) + '...');
    
    // Test token refresh (if implemented)
    setTimeout(() => {
        console.log('\n🔄 Testing token refresh...');
        const refreshData = {
            jwt_token: jwtToken,
            mobile_no: mobileNo,
            device_id: deviceId
        };
        
        // In a real implementation, you would send this to a refresh endpoint
        console.log('📤 Refresh request data:', refreshData);
        
        // Disconnect after testing
        setTimeout(() => {
            console.log('\n✅ JWT Token testing completed');
            socket.disconnect();
        }, 2000);
    }, 2000);
}

// Test JWT token with different scenarios
async function testJwtScenarios() {
    console.log('\n🧪 Testing JWT Token Scenarios...\n');
    
    // Test 1: New user registration
    console.log('📝 Scenario 1: New User Registration');
    await testJwtToken();
    
    // Wait before next test
    await new Promise(resolve => setTimeout(resolve, 5000));
    
    // Test 2: Existing user login
    console.log('\n📝 Scenario 2: Existing User Login');
    await testJwtToken();
}

// Run the tests
if (require.main === module) {
    testJwtScenarios().catch(console.error);
}

module.exports = { testJwtToken, testJwtScenarios }; 