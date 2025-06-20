#!/usr/bin/env node

const HeartbeatTester = require('./heartbeat-test');

async function runTest() {
    console.log('🚀 Starting Heartbeat Connection Test...\n');
    
    const tester = new HeartbeatTester();
    
    try {
        // Connect to server
        await tester.connect();
        
        // Run test for 10 minutes
        await tester.runLongTermTest(10);
        
    } catch (error) {
        console.error('❌ Test failed:', error.message);
        process.exit(1);
    } finally {
        tester.cleanup();
        console.log('\n🏁 Test completed');
    }
}

// Run the test
runTest(); 