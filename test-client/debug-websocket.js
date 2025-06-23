const { io } = require('socket.io-client');
const readline = require('readline');

// Create readline interface for user input
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

// Create Socket.IO client with detailed logging
const socket = io('http://localhost:3002', {
    transports: ['websocket'],
    upgrade: false,
    forceNew: true,
    timeout: 20000,
    reconnection: false
});

let sessionToken = '';
let expectedOtp = '';
let profileTimeout;

// Add comprehensive event listeners
socket.on('connect', () => {
    console.log('✅ Connected to server');
    console.log('Socket ID:', socket.id);
    
    // Test device info
    setTimeout(() => {
        console.log('📱 Sending device info...');
        socket.emit('device:info', {
            device_id: 'test_device_123',
            device_type: 'mobile',
            timestamp: new Date().toISOString(),
            manufacturer: 'Test',
            model: 'Test Model',
            firmware_version: 'Test OS',
            capabilities: ['camera', 'gps']
        });
    }, 1000);
});

// Debug event listeners for unhandled events
socket.on('connect_response', (data) => {
    console.log('📡 Connect response:', data);
});

socket.on('heartbeat', (data) => {
    console.log('📡 Heartbeat:', data);
});

socket.on('welcome', (data) => {
    console.log('📡 Welcome:', data);
});

socket.on('device:info:ack', (data) => {
    console.log('📱 Device info acknowledged:', data);
    
    // Test login
    setTimeout(() => {
        console.log('🔐 Sending login...');
        socket.emit('login', {
            mobile_no: '6935824711',
            device_id: 'test_device_123',
            fcm_token: 'fcm_token_test_very_long_string_to_satisfy_validation_requirements_minimum_100_characters_long_for_testing_purposes_only_this_is_not_a_real_fcm_token_just_for_debugging_123456789',
            email: 'test@example.com'
        });
    }, 1000);
});

socket.on('login:success', (data) => {
    console.log('🔐 Login successful:', data);
    sessionToken = data.session_token;
    expectedOtp = data.otp.toString();
    
    console.log(`\n📱 Expected OTP: ${expectedOtp}`);
    console.log('🔢 Please enter the OTP:');
    
    // Ask for OTP input
    rl.question('Enter OTP: ', (userOtp) => {
        console.log('🔢 Sending OTP verification...');
        socket.emit('verify:otp', {
            mobile_no: '6935824711',
            session_token: sessionToken,
            otp: userOtp
        });
    });
});

socket.on('otp:verified', (data) => {
    console.log('✅ OTP verified:', data);
    console.log('User status:', data.user_status);
    console.log('Session token:', sessionToken);
    
    // If new user, set up profile
    if (data.user_status === 'new_user') {
        console.log('\n👤 Setting up user profile...');
        setTimeout(() => {
            const profileData = {
                mobile_no: '6935824711',
                session_token: sessionToken,
                full_name: 'John Doe',
                state: 'California',
                referred_by: 'FRIEND456',
                profile_data: {
                    avatar: 'https://example.com/avatar.jpg',
                    bio: 'Gaming enthusiast',
                    preferences: {
                        notifications: true,
                        privacy: 'public'
                    }
                }
            };
            console.log('📤 Sending set:profile with data:', JSON.stringify(profileData, null, 2));
            socket.emit('set:profile', profileData);
            console.log('👤 Profile set request sent');
            
            // Set timeout for profile:set response
            profileTimeout = setTimeout(() => {
                console.log('⏰ Timeout: No profile:set response received after 10 seconds');
                console.log('🔍 Debugging: Checking if session is still valid...');
                console.log('🔍 Session token being used:', sessionToken);
                console.log('🔍 Mobile number being used: 6935824711');
                
                // Try to send a ping to check connection
                socket.emit('ping');
                
                // Try sending a simple test event
                console.log('🧪 Testing with a simple event...');
                socket.emit('health_check');
            }, 10000);
        }, 1000);
    } else {
        console.log('👤 Existing user - skipping profile setup');
        // Continue to language settings
        setTimeout(() => {
            console.log('\n🌐 Setting language preferences...');
            socket.emit('set:language', {
                mobile_no: '6935824711',
                session_token: sessionToken,
                language_code: 'en',
                language_name: 'English',
                region_code: 'US',
                timezone: 'America/Los_Angeles',
                user_preferences: {
                    date_format: 'MM/DD/YYYY',
                    time_format: '12h',
                    currency: 'USD'
                }
            });
        }, 1000);
    }
});

// Add health check response handler
socket.on('health_check:ack', (data) => {
    console.log('🏥 Health check response:', data);
});

socket.on('profile:set', (data) => {
    console.log('✅ Profile set successfully:', data);
    
    // Clear timeout since we got the response
    if (profileTimeout) {
        clearTimeout(profileTimeout);
    }
    
    // Continue to language settings
    setTimeout(() => {
        console.log('\n🌐 Setting language preferences...');
        socket.emit('set:language', {
            mobile_no: '6935824711',
            session_token: sessionToken,
            language_code: 'en',
            language_name: 'English',
            region_code: 'US',
            timezone: 'America/Los_Angeles',
            user_preferences: {
                date_format: 'MM/DD/YYYY',
                time_format: '12h',
                currency: 'USD'
            }
        });
    }, 1000);
});

socket.on('language:set', (data) => {
    console.log('✅ Language set successfully:', data);
    console.log('🎉 Complete user setup finished!');
    
    // Disconnect after successful test
    setTimeout(() => {
        console.log('🔌 Disconnecting...');
        rl.close();
        socket.disconnect();
    }, 2000);
});

// Add error event listeners
socket.on('connection_error', (data) => {
    console.log('❌ Connection error:', data);
    
    // Close readline if there's an error
    if (rl && !rl.closed) {
        rl.close();
    }
});

socket.on('error', (error) => {
    console.log('💥 Socket error:', error);
    
    // Close readline if there's an error
    if (rl && !rl.closed) {
        rl.close();
    }
});

socket.on('disconnect', (reason) => {
    console.log('🔌 Disconnected:', reason);
    
    // Close readline on disconnect
    if (rl && !rl.closed) {
        rl.close();
    }
});

socket.on('connect_error', (error) => {
    console.log('🔌 Connection error:', error);
    
    // Close readline if there's an error
    if (rl && !rl.closed) {
        rl.close();
    }
});

// Add ping/pong handlers for debugging
socket.on('pong', (data) => {
    console.log('🏓 Pong received:', data);
});

console.log('🚀 Starting WebSocket debug test...');
console.log('Server URL: http://localhost:3002');
console.log('📱 Mobile number: 6935824711'); 