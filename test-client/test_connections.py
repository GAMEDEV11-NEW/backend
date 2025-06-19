import socket
import time
import asyncio
from socketio import AsyncClient
import aiohttp
import sys
from colorama import init, Fore, Style
import websockets

# Initialize colorama for colored output
init()

def print_success(message):
    print(f"{Fore.GREEN}✅ {message}{Style.RESET_ALL}")

def print_error(message):
    print(f"{Fore.RED}❌ {message}{Style.RESET_ALL}")

def print_info(message):
    print(f"{Fore.BLUE}ℹ️ {message}{Style.RESET_ALL}")

async def test_tcp_connection():
    """Test direct TCP connection (should fail)"""
    print_info("\nTesting direct TCP connection...")
    try:
        # Create a TCP socket
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)  # Set timeout to 5 seconds
        
        # Try to connect
        result = sock.connect_ex(('localhost', 3002))
        
        if result == 0:
            print_error("TCP connection succeeded (unexpected - should be blocked)")
            # Try to send some data
            try:
                sock.send(b"Hello Server")
                data = sock.recv(1024)
                print_error(f"Received response: {data.decode()}")
            except Exception as e:
                print_info(f"Could not communicate over TCP: {e}")
        else:
            print_success("TCP connection blocked as expected")
            
    except Exception as e:
        print_success(f"TCP connection blocked as expected: {e}")
    finally:
        sock.close()

async def test_websocket_connection():
    """Test direct WebSocket connection (should fail)"""
    print_info("\nTesting direct WebSocket connection...")
    try:
        uri = "ws://localhost:3002/websocket"
        async with websockets.connect(uri, timeout=5) as websocket:
            await websocket.send("Hello Server")
            response = await websocket.recv()
            print_error(f"WebSocket connection succeeded unexpectedly: {response}")
    except websockets.exceptions.InvalidStatusCode as e:
        if e.status_code == 403:
            print_success("WebSocket connection blocked as expected (403 Forbidden)")
        else:
            print_error(f"WebSocket connection failed with unexpected status: {e.status_code}")
    except Exception as e:
        print_success(f"WebSocket connection blocked as expected: {e}")

async def test_socketio_connection():
    """Test Socket.IO connection (should succeed)"""
    print_info("\nTesting Socket.IO connection...")
    try:
        # Create Socket.IO client
        sio = AsyncClient()
        
        @sio.event
        async def connect():
            print_success("Socket.IO connection established successfully")
            # Test device connection event
            await sio.emit('device:connect', {
                'deviceId': 'test-python-client',
                'type': 'test-device',
                'status': 'online'
            })

        @sio.event
        async def disconnect():
            print_info("Socket.IO disconnected")

        @sio.event
        async def device_ack(data):
            print_success(f"Received device acknowledgment: {data}")

        # Connect to the server
        await sio.connect('http://localhost:3002', wait_timeout=5)
        
        # Wait for a moment to receive responses
        await asyncio.sleep(2)
        
        # Disconnect
        await sio.disconnect()
        
    except Exception as e:
        print_error(f"Socket.IO connection failed: {e}")

async def test_http_connection():
    """Test HTTP connection (should fail)"""
    print_info("\nTesting HTTP connection...")
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get('http://localhost:3002') as response:
                if response.status == 403:
                    print_success("HTTP connection blocked as expected (403 Forbidden)")
                else:
                    print_error(f"HTTP connection succeeded with status {response.status} (unexpected)")
    except aiohttp.ClientError as e:
        print_success(f"HTTP connection blocked as expected: {e}")
    except Exception as e:
        print_error(f"Unexpected error: {e}")

async def main():
    print_info("Starting connection tests...")
    print_info("This script will test TCP, HTTP, WebSocket, and Socket.IO connections")
    print_info("Only Socket.IO connections should succeed\n")

    # Run all tests
    await test_tcp_connection()
    await test_http_connection()
    await test_websocket_connection()
    await test_socketio_connection()

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print_info("\nTests interrupted by user")
        sys.exit(0)
    except Exception as e:
        print_error(f"\nUnexpected error: {e}")
        sys.exit(1) 