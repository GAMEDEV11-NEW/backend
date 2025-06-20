const io = require('socket.io-client');

const SERVER_URL = 'http://localhost:3000'; // Change if your backend runs on a different port
const MOBILE_NO = '1234567890';
const DEVICE_ID = 'test-device-001';
const FCM_TOKEN = 'dummy-fcm-token-1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890';
const EMAIL = 'test@example.com';
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
    console.log('Connected to backend. Sending login...');
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
    // Simulate user entering OTP
    setTimeout(() => {
        console.log('Sending OTP verification...');
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
    // Now test set:language event
    setTimeout(() => {
        console.log('Sending set:language event...');
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
            },
            timestamp: new Date().toISOString(),
        });
    }, 1000);
});

socket.on('language:set', (data) => {
    logEvent('language:set', data);
    console.log('Language setting test completed successfully!');
    socket.disconnect();
});

socket.on('connection_error', (data) => {
    logEvent('connection_error', data);
    socket.disconnect();
});

socket.on('disconnect', () => {
    console.log('Disconnected from backend.');
}); 