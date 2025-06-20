const { io } = require("socket.io-client");

// Test configuration
const SERVER_URL = "http://localhost:3002";
const TEST_TIMEOUT = 10000;

console.log("🚀 Starting Connection and Login Test...");
console.log(`📡 Connecting to: ${SERVER_URL}`);
console.log("─".repeat(60));

function testConnection() {
    return new Promise((resolve, reject) => {
        const socket = io(SERVER_URL, {
            transports: ["websocket"],
            timeout: TEST_TIMEOUT,
            forceNew: true
        });
        
        let testCompleted = false;
        let testTimeout;
        
        // Set up timeout
        testTimeout = setTimeout(() => {
            if (!testCompleted) {
                testCompleted = true;
                socket.disconnect();
                reject(new Error("Test timeout - no response received"));
            }
        }, TEST_TIMEOUT);
        
        // Handle connection
        socket.on("connect", () => {
            console.log(`✅ Connected to server (socket ID: ${socket.id})`);
            
            // Wait a bit for connection response
            setTimeout(() => {
                if (!testCompleted) {
                    console.log("⏳ Connection established, waiting for response...");
                }
            }, 1000);
        });
        
        // Handle connect response
        socket.on("connect_response", (data) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                console.log("📥 Received connect_response:", JSON.stringify(data, null, 2));
                socket.disconnect();
                resolve({ success: true, type: "connect_response", data });
            }
        });
        
        // Handle welcome message
        socket.on("welcome", (data) => {
            console.log("📥 Received welcome message:", JSON.stringify(data, null, 2));
        });
        
        // Handle heartbeat
        socket.on("heartbeat", (data) => {
            console.log("💓 Received heartbeat:", JSON.stringify(data, null, 2));
        });
        
        // Handle error messages
        socket.on("error", (data) => {
            console.log("❌ Received error:", JSON.stringify(data, null, 2));
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
        
        // Handle disconnect
        socket.on("disconnect", (reason) => {
            console.log(`🔌 Disconnected: ${reason}`);
        });
    });
}

function testLogin() {
    return new Promise((resolve, reject) => {
        const socket = io(SERVER_URL, {
            transports: ["websocket"],
            timeout: TEST_TIMEOUT,
            forceNew: true
        });
        
        let testCompleted = false;
        let testTimeout;
        
        // Set up timeout
        testTimeout = setTimeout(() => {
            if (!testCompleted) {
                testCompleted = true;
                socket.disconnect();
                reject(new Error("Login test timeout"));
            }
        }, TEST_TIMEOUT);
        
        // Handle connection
        socket.on("connect", () => {
            console.log(`🔌 Connected for login test (socket ID: ${socket.id})`);
            
            // Send login request
            const loginData = {
                mobile_no: "9876543210",
                device_id: "test_device_001",
                fcm_token: "fcm_token_example_" + "x".repeat(100),
                email: "test@example.com",
                timestamp: new Date().toISOString()
            };
            
            console.log("📤 Sending login request:", JSON.stringify(loginData, null, 2));
            socket.emit("login", loginData);
        });
        
        // Handle successful login
        socket.on("login:success", (data) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                console.log("✅ Login successful:", JSON.stringify(data, null, 2));
                socket.disconnect();
                resolve({ success: true, type: "login_success", data });
            }
        });
        
        // Handle login error
        socket.on("connection_error", (data) => {
            if (!testCompleted) {
                testCompleted = true;
                clearTimeout(testTimeout);
                console.log("❌ Login failed:", JSON.stringify(data, null, 2));
                socket.disconnect();
                resolve({ success: false, type: "login_error", data });
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

async function runTests() {
    try {
        console.log("🔍 Testing connection...");
        const connectionResult = await testConnection();
        console.log("✅ Connection test completed");
        console.log("─".repeat(60));
        
        console.log("🔍 Testing login...");
        const loginResult = await testLogin();
        console.log("✅ Login test completed");
        console.log("─".repeat(60));
        
        console.log("📊 Test Summary:");
        console.log(`Connection: ${connectionResult.success ? "✅ PASSED" : "❌ FAILED"}`);
        console.log(`Login: ${loginResult.success ? "✅ PASSED" : "❌ FAILED"}`);
        
    } catch (error) {
        console.error("💥 Test failed:", error.message);
    }
}

// Run tests if this file is executed directly
if (require.main === module) {
    runTests().catch(console.error);
}

module.exports = { testConnection, testLogin, runTests }; 