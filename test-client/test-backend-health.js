// test-backend-health.js
const io = require('socket.io-client');
const axios = require('axios');

const SERVER_URL = 'http://localhost:3002';

async function checkHttpHealth() {
    try {
        const res = await axios.get(`${SERVER_URL}/health`);
        if (res.data === 'OK') {
            console.log('✅ HTTP /health endpoint: OK');
            return true;
        } else {
            console.error('❌ HTTP /health endpoint: Unexpected response:', res.data);
            return false;
        }
    } catch (err) {
        console.error('❌ HTTP /health endpoint: Error:', err.message);
        return false;
    }
}

function checkWebSocketConnection() {
    return new Promise((resolve) => {
        const socket = io(SERVER_URL, {
            transports: ['websocket'],
            timeout: 5000,
            forceNew: true,
        });

        let checks = {
            connect: false,
            heartbeat: false,
            healthCheck: false,
            unknownEvent: false,
        };

        socket.on('connect', () => {
            checks.connect = true;
            console.log('✅ WebSocket: Connected');
            // Send health check event
            socket.emit('health_check');
            // Send unknown event
            socket.emit('unknown_event', { foo: 'bar' });
        });

        socket.on('heartbeat', (data) => {
            checks.heartbeat = true;
            console.log('✅ WebSocket: Heartbeat received', data);
        });

        socket.on('health_check:ack', (data) => {
            checks.healthCheck = true;
            console.log('✅ WebSocket: Health check ack received', data);
        });

        socket.on('unknown_event_error', (data) => {
            checks.unknownEvent = true;
            console.log('✅ WebSocket: Unknown event handled gracefully', data.message);
            socket.disconnect();
            resolve(checks);
        });

        socket.on('connect_error', (err) => {
            console.error('❌ WebSocket: Connection error:', err.message);
            resolve(checks);
        });

        setTimeout(() => {
            if (!checks.unknownEvent) {
                socket.disconnect();
                resolve(checks);
            }
        }, 7000);
    });
}

(async () => {
    console.log('--- Backend Health Check ---');
    const httpOk = await checkHttpHealth();
    if (!httpOk) {
        console.error('❌ Backend HTTP health check failed.');
        process.exit(1);
    }
    const wsChecks = await checkWebSocketConnection();
    if (wsChecks.connect && wsChecks.heartbeat && wsChecks.healthCheck && wsChecks.unknownEvent) {
        console.log('🎉 All backend health checks passed!');
        process.exit(0);
    } else {
        console.error('❌ Some backend health checks failed:', wsChecks);
        process.exit(2);
    }
})(); 