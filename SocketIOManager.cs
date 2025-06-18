using UnityEngine;
using SocketIOClient;
using System;
using System.Threading.Tasks;
using System.Collections;
using System.Text.Json;
using Newtonsoft.Json.Linq;

public class SocketIOManager : MonoBehaviour
{
    private SocketIO client;
    private bool isConnected = false;
    public string serverUrl = "http://192.168.31.171:3002"; // Can be set in Unity Inspector

    // Debug properties
    public bool IsConnected => isConnected;
    public string SocketId => client?.Id;
    public float LastPingTime { get; private set; }
    private SocketIODebugger debugger;

    private void Start()
    {
        debugger = FindObjectOfType<SocketIODebugger>();
        InitializeSocketIO();
    }

    private async void InitializeSocketIO()
    {
        try
        {
            LogDebug("Initializing Socket.IO...");
            client = new SocketIO(serverUrl, new SocketIOOptions
            {
                Path = "/socket.io/",
                Reconnection = true,
                ReconnectionAttempts = 5,
                ReconnectionDelay = 1000,
                Transport = TransportProtocol.WebSocket
            });

            // Setup event handlers
            SetupEventHandlers();

            await ConnectAsync();
        }
        catch (Exception e)
        {
            LogError($"Initialization error: {e.Message}");
        }
    }

    private void SetupEventHandlers()
    {
        client.OnConnected += (sender, e) =>
        {
            MainThreadDispatcher.Instance.Enqueue(() =>
            {
                isConnected = true;
                LogDebug($"Connected to server. Socket ID: {client.Id}");
            });
        };

        client.OnDisconnected += (sender, e) =>
        {
            MainThreadDispatcher.Instance.Enqueue(() =>
            {
                isConnected = false;
                LogDebug($"Disconnected: {e}");
            });
        };

        client.OnReconnectAttempt += (sender, attempt) =>
        {
            LogDebug($"Reconnection attempt {attempt}...");
        };

        client.OnReconnected += (sender, attempt) =>
        {
            LogDebug($"Reconnected after {attempt} attempts");
        };

        client.OnPing += () =>
        {
            LastPingTime = Time.realtimeSinceStartup;
        };

        client.On("message", response =>
        {
            MainThreadDispatcher.Instance.Enqueue(() =>
            {
                try
                {
                    var data = response.GetValue<JObject>();
                    LogDebug($"Received: {data}");
                    
                    if (data["type"] != null)
                    {
                        HandleMessageByType(data);
                    }
                }
                catch (Exception e)
                {
                    LogError($"Error parsing message: {e.Message}");
                }
            });
        });

        client.OnError += (sender, e) =>
        {
            LogError($"Socket.IO Error: {e}");
        };
    }

    private void HandleMessageByType(JObject data)
    {
        string messageType = data["type"].ToString();
        LogDebug($"Handling message of type: {messageType}");
        
        switch (messageType)
        {
            case "gameState":
                // Handle game state updates
                break;
            case "playerAction":
                // Handle player actions
                break;
            default:
                LogDebug($"Unknown message type: {messageType}");
                break;
        }
    }

    public async void Connect()
    {
        if (client == null)
        {
            InitializeSocketIO();
            return;
        }

        await ConnectAsync();
    }

    private async Task ConnectAsync()
    {
        try
        {
            LogDebug("Connecting to server...");
            await client.ConnectAsync();
        }
        catch (Exception e)
        {
            LogError($"Connection error: {e.Message}");
        }
    }

    public async void Disconnect()
    {
        if (client != null && isConnected)
        {
            try
            {
                LogDebug("Disconnecting from server...");
                await client.DisconnectAsync();
            }
            catch (Exception e)
            {
                LogError($"Disconnect error: {e.Message}");
            }
        }
    }

    public async void SendMessage(string message)
    {
        if (!isConnected)
        {
            LogWarning("Cannot send message - not connected");
            return;
        }

        try
        {
            var messageData = new
            {
                text = message,
                timestamp = DateTimeOffset.Now.ToUnixTimeMilliseconds()
            };
            await client.EmitAsync("message", messageData);
            LogDebug($"Sent message: {message}");
        }
        catch (Exception e)
        {
            LogError($"Error sending message: {e.Message}");
        }
    }

    public async void SendGameAction(string actionType, object actionData)
    {
        if (!isConnected)
        {
            LogWarning("Cannot send game action - not connected");
            return;
        }

        try
        {
            var gameAction = new
            {
                type = actionType,
                data = actionData,
                timestamp = DateTimeOffset.Now.ToUnixTimeMilliseconds()
            };
            await client.EmitAsync("gameAction", gameAction);
            LogDebug($"Sent game action: {actionType}");
        }
        catch (Exception e)
        {
            LogError($"Error sending game action: {e.Message}");
        }
    }

    public void UpdateServerUrl(string newUrl)
    {
        if (serverUrl != newUrl)
        {
            serverUrl = newUrl;
            if (isConnected)
            {
                Disconnect();
                Connect();
            }
        }
    }

    private void LogDebug(string message)
    {
        if (debugger != null)
            debugger.LogMessage(message);
        else
            Debug.Log($"[SocketIO] {message}");
    }

    private void LogError(string message)
    {
        if (debugger != null)
            debugger.LogError(message);
        else
            Debug.LogError($"[SocketIO] {message}");
    }

    private void LogWarning(string message)
    {
        if (debugger != null)
            debugger.LogWarning(message);
        else
            Debug.LogWarning($"[SocketIO] {message}");
    }

    private void OnDestroy()
    {
        if (client != null && isConnected)
        {
            Disconnect();
        }
    }
}

// Helper class to run code on Unity's main thread
public class MainThreadDispatcher : MonoBehaviour
{
    private static MainThreadDispatcher instance;
    private readonly Queue actionQueue = new Queue();

    public static MainThreadDispatcher Instance
    {
        get
        {
            if (instance == null)
            {
                var go = new GameObject("MainThreadDispatcher");
                instance = go.AddComponent<MainThreadDispatcher>();
                DontDestroyOnLoad(go);
            }
            return instance;
        }
    }

    public void Enqueue(Action action)
    {
        lock (actionQueue)
        {
            actionQueue.Enqueue(action);
        }
    }

    private void Update()
    {
        lock (actionQueue)
        {
            while (actionQueue.Count > 0)
            {
                var action = (Action)actionQueue.Dequeue();
                action.Invoke();
            }
        }
    }
} 