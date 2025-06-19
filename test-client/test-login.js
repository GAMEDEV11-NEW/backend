const { io } = require("socket.io-client");

// Test configuration
const SERVER_URL = "http://localhost:3002";
const TEST_TIMEOUT = 5000;

// Test cases for login functionality
const loginTestCases = [
    // Valid login case
    {
        name: "Valid login with all required fields",
        data: {
            mobile_no: "9876543210",
            device_id: "device_001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "success"
    },
    // Invalid: missing mobile_no
    {
        name: "Missing mobile_no",
        data: {
            device_id: "device_001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: missing device_id
    {
        name: "Missing device_id",
        data: {
            mobile_no: "9876543210",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: missing fcm_token
    {
        name: "Missing fcm_token",
        data: {
            mobile_no: "9876543210",
            device_id: "device_001",
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: empty mobile_no
    {
        name: "Empty mobile_no",
        data: {
            mobile_no: "",
            device_id: "device_001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: mobile_no not digits
    {
        name: "Non-digit mobile_no",
        data: {
            mobile_no: "98A6543210",
            device_id: "device_001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: mobile_no too short
    {
        name: "Short mobile_no",
        data: {
            mobile_no: "12345",
            device_id: "device_001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: device_id with special chars
    {
        name: "Invalid device_id format",
        data: {
            mobile_no: "9876543210",
            device_id: "device@001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: fcm_token too short
    {
        name: "Short fcm_token",
        data: {
            mobile_no: "9876543210",
            device_id: "device_001",
            fcm_token: "shorttoken",
            timestamp: "2024-01-15T10:30:00Z"
        },
        expectedStatus: "error"
    },
    // Invalid: timestamp wrong format
    {
        name: "Invalid timestamp format",
        data: {
            mobile_no: "9876543210",
            device_id: "device_001",
            fcm_token: "fcm_token_example_" + "x".repeat(100),
            timestamp: "2024-01-15 10:30:00"
        },
        expectedStatus: "error"
    }
];

// Test runner
async function runLoginTests() {
    console.log("🚀 Starting Login Tests...\n");
    
    let passedTests = 0;
    let failedTests = 0;
    
    for (const testCase of loginTestCases) {
        console.log(`📋 Test: ${testCase.name}`);
        console.log(`📤 Sending data:`, JSON.stringify(testCase.data, null, 2));
        
        try {
            const result = await testLogin(testCase.data, testCase.expectedStatus);
            
            if (result.success) {
                console.log(`✅ PASSED - Expected: ${testCase.expectedStatus}, Got: ${result.status}`);
                passedTests++;
            } else {
                console.log(`❌ FAILED - Expected: ${testCase.expectedStatus}, Got: ${result.status}`);
                console.log(`   Error: ${result.error}`);
                failedTests++;
            }
        } catch (error) {
            console.log(`💥 ERROR - ${error.message}`);
            failedTests++;
        }
        
        console.log("─".repeat(50));
    }
    
    console.log(`\n📊 Test Results:`);
    console.log(`✅ Passed: ${passedTests}`);
    console.log(`❌ Failed: ${failedTests}`);
    console.log(`📈 Success Rate: ${((passedTests / (passedTests + failedTests)) * 100).toFixed(1)}%`);
}

// Individual test function
function testLogin(loginData, expectedStatus) {
    return new Promise((resolve, reject) => {
        const socket = io(SERVER_URL, {
            transports: ["websocket"],
            timeout: TEST_TIMEOUT
        });
        
        let testCompleted = false;
        let testTimeout;
        
        // Set up timeout
        testTimeout = setTimeout(() => {
            if (!testCompleted) {
                testCompleted = true;
                socket.disconnect();
                reject(new Error("Test timeout"));
            }
        }, TEST_TIMEOUT);
        
        // Handle connection
        socket.on("connect", () => {
            console.log(`   🔌 Connected to server (socket ID: ${socket.id})`);
            
            // Send login request
            socket.emit("login", loginData);
        });
        
        // Handle successful login
        socket.on("login:success", (data) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                socket.disconnect();
                
                console.log(`   📥 Received login:success:`, JSON.stringify(data, null, 2));
                
                resolve({
                    success: expectedStatus === "success",
                    status: "success",
                    data: data
                });
            }
        });
        
        // Handle login error
        socket.on("connection_error", (data) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                socket.disconnect();
                
                console.log(`   📥 Received connection_error:`, JSON.stringify(data, null, 2));
                
                resolve({
                    success: expectedStatus === "error",
                    status: "error",
                    data: data
                });
            }
        });
        
        // Handle connection errors
        socket.on("connect_error", (error) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                socket.disconnect();
                reject(new Error(`Connection error: ${error.message}`));
            }
        });
        
        socket.on("error", (error) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                socket.disconnect();
                reject(new Error(`Socket error: ${error}`));
            }
        });
    });
}

// Run tests if this file is executed directly
if (require.main === module) {
    runLoginTests().catch(console.error);
}

module.exports = { runLoginTests, testLogin }; 