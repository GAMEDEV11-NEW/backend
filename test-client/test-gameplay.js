const { io } = require('socket.io-client');

// Connect to the server
const socket = io('http://localhost:3002', {
    transports: ['websocket'],
    timeout: 20000,
    forceNew: true
});

console.log('🎮 Testing Gameplay Events...\n');

// Connection event
socket.on('connect', () => {
    console.log('✅ Connected to server');
    console.log('Socket ID:', socket.id);
    
    // Test gameplay event
    testGameplayEvent();
});

// Disconnect event
socket.on('disconnect', (reason) => {
    console.log('❌ Disconnected:', reason);
});

// Error event
socket.on('connect_error', (error) => {
    console.log('❌ Connection error:', error.message);
});

// Test gameplay event
function testGameplayEvent() {
    console.log('\n🎯 Testing get_game_data event...');
    
    const testData = {
        user_id: "test_user_123",
        timestamp: new Date().toISOString()
    };
    
    socket.emit('get_game_data', testData);
}

// Listen for game data response
socket.on('game_data_response', (response) => {
    console.log('\n✅ Received game data response:');
    console.log('Status:', response.status);
    console.log('Event:', response.event);
    console.log('Socket ID:', response.socket_id);
    console.log('Timestamp:', response.timestamp);
    
    console.log('\n🎮 Game Data:');
    response.data.forEach((game, index) => {
        console.log(`\n${index + 1}. ${game.game_name}`);
        console.log(`   ID: ${game.game_id}`);
        console.log(`   Type: ${game.game_type}`);
        console.log(`   Description: ${game.description}`);
        console.log(`   Active: ${game.is_active}`);
        console.log(`   Players: ${game.player_count}`);
        console.log(`   Difficulty: ${game.difficulty}`);
    });
    
    // Disconnect after receiving response
    setTimeout(() => {
        console.log('\n👋 Disconnecting...');
        socket.disconnect();
        process.exit(0);
    }, 1000);
});

// Handle any errors
socket.on('error', (error) => {
    console.log('❌ Socket error:', error);
});

// Handle timeout
setTimeout(() => {
    console.log('⏰ Test timeout - disconnecting...');
    socket.disconnect();
    process.exit(1);
}, 10000); 