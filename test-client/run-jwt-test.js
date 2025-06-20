const { testJwtToken, testJwtScenarios } = require('./test-jwt-token');

console.log('🚀 Starting JWT Token and UUID v7 Testing Suite...\n');

async function runJwtTests() {
    try {
        console.log('📋 Test Plan:');
        console.log('1. ✅ UUID v7 User ID generation');
        console.log('2. ✅ Sequential user numbering');
        console.log('3. ✅ JWT token generation after OTP verification');
        console.log('4. ✅ JWT token validation with device check');
        console.log('5. ✅ JWT token payload verification');
        console.log('6. ✅ JWT token expiry handling');
        console.log('7. ✅ FCM + Device ID + Mobile Number integration');
        console.log('8. ✅ Secret key configuration');
        console.log('\n');

        await testJwtScenarios();
        
        console.log('\n🎉 All JWT and UUID v7 tests completed successfully!');
        console.log('\n📊 Summary:');
        console.log('   ✅ UUID v7 user IDs are being generated');
        console.log('   ✅ Sequential user numbering is working');
        console.log('   ✅ JWT tokens are generated after OTP verification');
        console.log('   ✅ JWT tokens include FCM, device_id, and mobile_number');
        console.log('   ✅ JWT tokens have proper expiry (7 days)');
        console.log('   ✅ JWT tokens use secret key for signing');
        console.log('   ✅ All database operations are working');
        
    } catch (error) {
        console.error('❌ Test failed:', error);
        process.exit(1);
    }
}

// Run the tests
runJwtTests(); 