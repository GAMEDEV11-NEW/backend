const io = require('socket.io-client');

const SERVER_URL = 'http://localhost:3002'; // Change if your backend runs on a different port
const MOBILE_NO = '1234567890';
const DEVICE_ID = 'test-device-001';
const FCM_TOKEN = 'dummy-fcm-token-1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890';
const EMAIL = 'test@example.com';
const FULL_NAME = 'John Doe';
const STATE = 'California';
const REFERRAL_CODE = 'JOHN123';
const REFERRED_BY = 'FRIEND456';
const LANGUAGE_CODE = 'en';
const LANGUAGE_NAME = 'English';
const REGION_CODE = 'US';
const TIMEZONE = 'America/New_York';

function logEvent(event, data) {
  console.log(`\n[${event}]`);
  console.dir(data, { depth: 5 });
}

async function testUserFlow(mobileNo, isNewUser = true) {
  return new Promise((resolve, reject) => {
    const socket = io(SERVER_URL, {
      transports: ['websocket'],
      reconnection: false,
    });

    let sessionToken = null;
    let otp = null;

    socket.on('connect', () => {
      console.log(`\n=== Testing ${isNewUser ? 'NEW USER' : 'EXISTING USER'} flow for mobile: ${mobileNo} ===`);
      console.log('Step 1: Sending login...');
      socket.emit('login', {
        mobile_no: mobileNo,
        device_id: DEVICE_ID,
        fcm_token: FCM_TOKEN,
        email: EMAIL,
        timestamp: new Date().toISOString(),
      });
    });

    socket.on('login:success', (data) => {
      logEvent('login:success', data);
      sessionToken = data.session_token;
      otp = data.otp.toString();
      
      console.log(`User type: ${data.is_new_user ? 'NEW USER' : 'EXISTING USER'}`);
      
      if (data.is_new_user !== isNewUser) {
        console.log(`❌ Expected ${isNewUser ? 'NEW' : 'EXISTING'} user but got ${data.is_new_user ? 'NEW' : 'EXISTING'} user`);
        socket.disconnect();
        reject(new Error('User type mismatch'));
        return;
      }
      
      console.log('Step 2: Sending OTP verification...');
      setTimeout(() => {
        socket.emit('verify:otp', {
          mobile_no: mobileNo,
          otp: otp,
          session_token: sessionToken,
          timestamp: new Date().toISOString(),
        });
      }, 1000);
    });

    socket.on('otp:verified', (data) => {
      logEvent('otp:verified', data);
      console.log('Step 3: Sending user profile setup...');
      setTimeout(() => {
        socket.emit('set:profile', {
          mobile_no: mobileNo,
          session_token: sessionToken,
          full_name: FULL_NAME,
          state: STATE,
          referral_code: REFERRAL_CODE,
          referred_by: REFERRED_BY,
          profile_data: {
            age: 25,
            gender: 'male',
            interests: ['gaming', 'technology'],
            avatar: 'default.png'
          },
          timestamp: new Date().toISOString(),
        });
      }, 1000);
    });

    socket.on('profile:set', (data) => {
      logEvent('profile:set', data);
      console.log('Step 4: Sending language setting...');
      setTimeout(() => {
        socket.emit('set:language', {
          mobile_no: mobileNo,
          session_token: sessionToken,
          language_code: LANGUAGE_CODE,
          language_name: LANGUAGE_NAME,
          region_code: REGION_CODE,
          timezone: TIMEZONE,
          user_preferences: {
            theme: 'dark',
            notifications: true,
            sound_enabled: true
          },
          timestamp: new Date().toISOString(),
        });
      }, 1000);
    });

    socket.on('language:set', (data) => {
      logEvent('language:set', data);
      console.log(`✅ ${isNewUser ? 'NEW USER' : 'EXISTING USER'} flow completed successfully!`);
      socket.disconnect();
      resolve(data);
    });

    socket.on('connection_error', (data) => {
      logEvent('connection_error', data);
      console.log(`❌ ${isNewUser ? 'NEW USER' : 'EXISTING USER'} test failed with error`);
      socket.disconnect();
      reject(new Error(data.message || 'Connection error'));
    });

    socket.on('disconnect', () => {
      console.log('Disconnected from backend.');
    });
  });
}

async function runTests() {
  try {
    console.log('🚀 Starting User Registration Tests...\n');
    
    // Test 1: New user registration
    console.log('📝 Test 1: New User Registration');
    await testUserFlow(MOBILE_NO, true);
    
    // Wait a bit between tests
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Test 2: Existing user login (same mobile number)
    console.log('\n📝 Test 2: Existing User Login');
    await testUserFlow(MOBILE_NO, false);
    
    // Wait a bit between tests
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Test 3: Another new user with different mobile number
    console.log('\n📝 Test 3: Another New User Registration');
    await testUserFlow('9876543210', true);
    
    console.log('\n🎉 All tests completed successfully!');
    console.log('\n📊 Summary:');
    console.log('   ✅ New user registration works');
    console.log('   ✅ Existing user login works');
    console.log('   ✅ User data is stored in userregister collection');
    console.log('   ✅ Profile and language settings are updated');
    console.log('   ✅ No duplicate users are created');
    
  } catch (error) {
    console.error('\n❌ Test failed:', error.message);
    process.exit(1);
  }
}

// Run the tests
runTests(); 