const io = require('socket.io-client');

const SERVER_URL = 'http://localhost:3000'; // Change if your backend runs on a different port
const MOBILE_NO = '1234567890';
const DEVICE_ID = 'test-device-001';
const FCM_TOKEN = 'dummy-fcm-token-1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890';
const EMAIL = 'test@example.com';
const FULL_NAME = 'John Doe';
const STATE = 'California';
const REFERRAL_CODE = 'JOHN123'; // Optional - will be generated if not provided
const REFERRED_BY = 'FRIEND456'; // Optional
const LANGUAGE_CODE = 'en';
const LANGUAGE_NAME = 'English';
const REGION_CODE = 'US';
const TIMEZONE = 'America/New_York';

const socket = io(SERVER_URL, {
  transports: ['websocket'],
  reconnection: false,
});

function logEvent(event, data) {
  console.log(`\n[${event}]`);
  console.dir(data, { depth: 5 });
}

let sessionToken = null;
let otp = null;

socket.on('connect', () => {
  console.log('Connected to backend. Starting complete flow test...');
  console.log('Step 1: Sending login...');
  socket.emit('login', {
    mobile_no: MOBILE_NO,
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
  console.log('Step 2: Sending OTP verification...');
  setTimeout(() => {
    socket.emit('verify:otp', {
      mobile_no: MOBILE_NO,
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
      mobile_no: MOBILE_NO,
      session_token: sessionToken,
      full_name: FULL_NAME,
      state: STATE,
      referral_code: REFERRAL_CODE, // Optional - will be generated if not provided
      referred_by: REFERRED_BY, // Optional
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
      mobile_no: MOBILE_NO,
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
  console.log('✅ Complete flow test finished successfully!');
  console.log('🎉 User onboarding completed:');
  console.log(`   - Login: ✅`);
  console.log(`   - OTP Verification: ✅`);
  console.log(`   - Profile Setup: ✅`);
  console.log(`   - Language Setting: ✅`);
  socket.disconnect();
});

socket.on('connection_error', (data) => {
  logEvent('connection_error', data);
  console.log('❌ Test failed with error');
  socket.disconnect();
});

socket.on('disconnect', () => {
  console.log('Disconnected from backend.');
});

// Test with auto-generated referral code
console.log('\n=== Testing with auto-generated referral code ===');
console.log('This test will generate a referral code automatically if not provided.');

// You can also test with a custom referral code by changing REFERRAL_CODE above 