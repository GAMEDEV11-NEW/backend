const { runLoginTests } = require('./test-login.js');

console.log('🔐 Login Test Runner');
console.log('===================\n');

// Check if server is running
const io = require('socket.io-client');
const socket = io('http://localhost:3002', {
    timeout: 3000,
    transports: ['websocket']
});

socket.on('connect', () => {
    console.log('✅ Server is running and accessible');
    socket.disconnect();
    
    // Run login tests
    console.log('\n🚀 Starting comprehensive login tests...\n');
    runLoginTests()
        .then(() => {
            console.log('\n🎉 All login tests completed!');
            process.exit(0);
        })
        .catch((error) => {
            console.error('\n💥 Test execution failed:', error.message);
            process.exit(1);
        });
});

socket.on('connect_error', (error) => {
    console.error('❌ Cannot connect to server at http://localhost:3002');
    console.error('   Make sure the server is running with: cargo run');
    console.error(`   Error: ${error.message}`);
    process.exit(1);
});

// Timeout for server check
setTimeout(() => {
    console.error('❌ Server connection timeout');
    console.error('   Make sure the server is running with: cargo run');
    process.exit(1);
}, 5000); 