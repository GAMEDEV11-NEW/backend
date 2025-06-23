const { io } = require('socket.io-client');

console.log('🔧 Testing WebSocket Panic Recovery and Socket Disconnection...\n');

// Test 1: Normal connection
async function testNormalConnection() {
    console.log('🔌 Test 1: Normal Connection');
    
    return new Promise((resolve, reject) => {
        const socket = io('http://localhost:3002', {
            transports: ['websocket'],
            timeout: 10000,
            forceNew: true
        });

        const timeout = setTimeout(() => {
            socket.disconnect();
            reject(new Error('Connection timeout'));
        }, 10000);

        socket.on('connect', () => {
            console.log('✅ Normal connection established');
            clearTimeout(timeout);
            socket.disconnect();
            resolve();
        });

        socket.on('connect_error', (error) => {
            console.log('❌ Connection failed:', error.message);
            clearTimeout(timeout);
            reject(error);
        });
    });
}

// Test 2: Simulate problematic behavior
async function testProblematicSocket() {
    console.log('\n⚠️ Test 2: Simulating Problematic Socket Behavior');
    
    return new Promise((resolve, reject) => {
        const socket = io('http://localhost:3002', {
            transports: ['websocket'],
            timeout: 10000,
            forceNew: true
        });

        const timeout = setTimeout(() => {
            socket.disconnect();
            reject(new Error('Problematic socket test timeout'));
        }, 15000);

        socket.on('connect', () => {
            console.log('✅ Connected, simulating problematic behavior...');
            
            // Send malformed data that might cause issues
            socket.emit('device:info', {
                device_id: 'a'.repeat(50000), // Very long string
                platform: null,
                version: undefined,
                // Send multiple rapid requests
                rapid_requests: true
            });
            
            // Send more problematic data
            setTimeout(() => {
                socket.emit('login', {
                    mobile_no: 'invalid',
                    device_id: 'a'.repeat(10000),
                    fcm_token: null
                });
            }, 1000);
            
            // Send even more problematic data
            setTimeout(() => {
                socket.emit('device:info', {
                    // Empty object
                });
            }, 2000);
        });

        socket.on('connection_error', (data) => {
            console.log('✅ Server handled problematic behavior correctly:', data.error_code);
            clearTimeout(timeout);
            socket.disconnect();
            resolve();
        });

        socket.on('disconnect', (reason) => {
            console.log('🔌 Socket disconnected:', reason);
            clearTimeout(timeout);
            resolve();
        });

        socket.on('connect_error', (error) => {
            console.log('❌ Connection error:', error.message);
            clearTimeout(timeout);
            reject(error);
        });
    });
}

// Test 3: Multiple connections to test panic recovery
async function testMultipleConnections() {
    console.log('\n🔗 Test 3: Multiple Connections (Panic Recovery Test)');
    
    const connections = [];
    const maxConnections = 5;
    
    try {
        console.log(`Creating ${maxConnections} connections...`);
        
        for (let i = 0; i < maxConnections; i++) {
            const socket = io('http://localhost:3002', {
                transports: ['websocket'],
                timeout: 5000,
                forceNew: true
            });

            connections.push(socket);
            
            socket.on('connect', () => {
                console.log(`✅ Connection ${i + 1} established`);
                
                // Send some data to each connection
                socket.emit('device:info', {
                    device_id: `test-device-${i}`,
                    platform: 'test',
                    version: '1.0.0'
                });
            });

            socket.on('connect_error', (error) => {
                console.log(`❌ Connection ${i + 1} failed:`, error.message);
            });

            socket.on('disconnect', (reason) => {
                console.log(`🔌 Connection ${i + 1} disconnected:`, reason);
            });
        }

        // Wait for connections to establish
        await new Promise(resolve => setTimeout(resolve, 3000));
        
        console.log('✅ Multiple connections test completed');
        
    } catch (error) {
        console.log('❌ Multiple connections test failed:', error.message);
    } finally {
        // Clean up all connections
        connections.forEach(socket => socket.disconnect());
    }
}

// Test 4: Server health check
async function testServerHealth() {
    console.log('\n🏥 Test 4: Server Health Check');
    
    try {
        const response = await fetch('http://localhost:3002/health');
        if (response.ok) {
            console.log('✅ Server health check passed');
        } else {
            console.log('❌ Server health check failed');
        }
    } catch (error) {
        console.log('❌ Server health check error:', error.message);
    }
}

// Run all tests
async function runAllTests() {
    try {
        await testNormalConnection();
        await testProblematicSocket();
        await testMultipleConnections();
        await testServerHealth();
        
        console.log('\n🎉 All tests completed!');
        console.log('\n📋 Summary:');
        console.log('- Normal connections should work fine');
        console.log('- Problematic sockets should be handled gracefully');
        console.log('- Server should remain stable during issues');
        console.log('- Panic recovery should disconnect problematic sockets');
        
    } catch (error) {
        console.error('❌ Test suite failed:', error.message);
    }
}

// Check if fetch is available (Node.js 18+)
if (typeof fetch === 'undefined') {
    // Remove health check for older Node.js versions
    const originalTestServerHealth = testServerHealth;
    testServerHealth = async function() {
        console.log('\n🏥 Test 4: Server Health Check (skipped - fetch not available)');
    };
}

runAllTests(); 