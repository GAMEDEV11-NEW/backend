const { io } = require("socket.io-client");

// Connect to the Rust Socket.IO server
const socket = io("http://localhost:3002", {
    transports: ['websocket', 'polling'],
    path: "/socket.io/", // default Socket.IO path
    reconnection: true,
    reconnectionAttempts: 5,
    reconnectionDelay: 1000
});

// Connection event
socket.on("connect", () => {
    console.log("Connected to Socket.IO server");
    console.log("Socket ID:", socket.id);
    
    // Send initial message
    socket.emit("message", { text: "Hello from Node.js client!" });
    
    // Set up periodic messages
    setInterval(() => {
        const message = {
            text: `Test message at ${new Date().toISOString()}`,
            timestamp: Date.now()
        };
        console.log('Sending:', message);
        socket.emit("message", message);
    }, 5000); // Send a message every 5 seconds
});

// Listen for messages
socket.on("message", (data) => {
    console.log('Received:', data);
});

// Handle connection errors
socket.on("connect_error", (error) => {
    console.error("Connection error:", error.message);
});

// Handle disconnection
socket.on("disconnect", (reason) => {
    console.log("Disconnected:", reason);
});

// Handle reconnection attempts
socket.on("reconnect_attempt", (attemptNumber) => {
    console.log(`Attempting to reconnect... (attempt ${attemptNumber})`);
});

// Handle successful reconnection
socket.on("reconnect", (attemptNumber) => {
    console.log(`Reconnected after ${attemptNumber} attempts`);
});

// Handle process termination
process.on('SIGINT', () => {
    console.log('\nClosing Socket.IO connection...');
    socket.close();
    process.exit();
}); 