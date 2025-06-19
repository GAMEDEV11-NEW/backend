const io = require('socket.io-client');

// Connect to the Socket.IO server
const socket = io('http://localhost:3002');

// Handle connection events
socket.on('connect', () => {
    console.log('🌟 Connected to server');
    
    // Send device connection info
    const deviceInfo = {
        device_type: "sensor",
        device_name: "Sensor-X1",
        version: "1.0.0"
    };
    
    console.log('📤 Sending device connection info:', deviceInfo);
    socket.emit('device:connect', deviceInfo);
});

socket.on('device:ack', (response) => {
    console.log('✅ Device connection acknowledged:', response);
    
    // Send a test game action
    const gameAction = {
        id: "action_" + Date.now(),
        action: "sensor_reading",
        parameters: {
            temperature: 25.5,
            humidity: 60
        },
        timestamp: Date.now()
    };
    
    console.log('🎮 Sending game action:', gameAction);
    socket.emit('game:action', gameAction);
});

socket.on('game:ack', (response) => {
    console.log('✅ Game action acknowledged:', response);
});

socket.on('disconnect', () => {
    console.log('👋 Disconnected from server');
});

// Handle errors
socket.on('connect_error', (error) => {
    console.error('❌ Connection error:', error);
});

socket.on('error', (error) => {
    console.error('❌ Socket error:', error);
}); 