using UnityEngine;
using UnityEngine.UI;
using TMPro;
using System.Collections.Generic;
using System;

public class SocketIODebugger : MonoBehaviour
{
    [Header("UI References")]
    public GameObject debugPanel;           // Reference to the debug panel object
    public TextMeshProUGUI connectionStatus; // Shows connection status
    public TextMeshProUGUI eventLog;        // Shows event log
    public TMP_InputField serverUrlInput;   // Input field for server URL
    public Button connectButton;            // Button to connect/disconnect
    public Button clearLogButton;           // Button to clear log
    public Button testMessageButton;        // Button to send test message

    [Header("Debug Settings")]
    public bool showDebugOnStart = true;
    public int maxLogEntries = 50;

    private SocketIOManager socketManager;
    private List<string> logEntries = new List<string>();
    private bool isPanelVisible;

    void Start()
    {
        // Get reference to SocketIOManager
        socketManager = FindObjectOfType<SocketIOManager>();
        if (socketManager == null)
        {
            LogError("SocketIOManager not found in scene!");
            return;
        }

        // Setup UI elements
        SetupUI();

        // Show/hide debug panel based on setting
        debugPanel.SetActive(showDebugOnStart);
        isPanelVisible = showDebugOnStart;

        // Start updating status
        InvokeRepeating("UpdateStatus", 0f, 0.5f);
    }

    void SetupUI()
    {
        if (serverUrlInput != null)
        {
            serverUrlInput.text = socketManager.serverUrl;
            serverUrlInput.onEndEdit.AddListener(OnServerUrlChanged);
        }

        if (connectButton != null)
        {
            connectButton.onClick.AddListener(ToggleConnection);
        }

        if (clearLogButton != null)
        {
            clearLogButton.onClick.AddListener(ClearLog);
        }

        if (testMessageButton != null)
        {
            testMessageButton.onClick.AddListener(SendTestMessage);
        }
    }

    void Update()
    {
        // Toggle debug panel with F1 key
        if (Input.GetKeyDown(KeyCode.F1))
        {
            isPanelVisible = !isPanelVisible;
            debugPanel.SetActive(isPanelVisible);
        }
    }

    void UpdateStatus()
    {
        if (connectionStatus != null)
        {
            string status = $"Server: {socketManager.serverUrl}\n" +
                          $"Status: {(socketManager.IsConnected ? "Connected" : "Disconnected")}\n" +
                          $"Socket ID: {(socketManager.IsConnected ? socketManager.SocketId : "None")}\n" +
                          $"Last Ping: {socketManager.LastPingTime:0.00}ms";
            
            connectionStatus.text = status;
        }
    }

    public void LogMessage(string message)
    {
        string timeStamp = DateTime.Now.ToString("HH:mm:ss.fff");
        string logEntry = $"[{timeStamp}] {message}";
        
        logEntries.Add(logEntry);
        
        // Keep log size under control
        while (logEntries.Count > maxLogEntries)
        {
            logEntries.RemoveAt(0);
        }

        // Update UI
        if (eventLog != null)
        {
            eventLog.text = string.Join("\n", logEntries);
        }

        // Also log to Unity console
        Debug.Log($"[SocketIO] {message}");
    }

    public void LogError(string message)
    {
        LogMessage($"<color=red>ERROR: {message}</color>");
        Debug.LogError($"[SocketIO] {message}");
    }

    public void LogWarning(string message)
    {
        LogMessage($"<color=yellow>WARNING: {message}</color>");
        Debug.LogWarning($"[SocketIO] {message}");
    }

    void OnServerUrlChanged(string newUrl)
    {
        if (socketManager != null)
        {
            socketManager.UpdateServerUrl(newUrl);
            LogMessage($"Server URL updated to: {newUrl}");
        }
    }

    void ToggleConnection()
    {
        if (socketManager.IsConnected)
        {
            socketManager.Disconnect();
            LogMessage("Manual disconnect requested");
        }
        else
        {
            socketManager.Connect();
            LogMessage("Manual connect requested");
        }
    }

    void SendTestMessage()
    {
        if (socketManager.IsConnected)
        {
            string testMessage = $"Test message {DateTime.Now:HH:mm:ss}";
            socketManager.SendMessage(testMessage);
            LogMessage($"Sent test message: {testMessage}");
        }
        else
        {
            LogWarning("Cannot send test message - not connected");
        }
    }

    void ClearLog()
    {
        logEntries.Clear();
        if (eventLog != null)
        {
            eventLog.text = "";
        }
    }

    void OnDestroy()
    {
        // Clean up UI listeners
        if (serverUrlInput != null)
            serverUrlInput.onEndEdit.RemoveListener(OnServerUrlChanged);
        
        if (connectButton != null)
            connectButton.onClick.RemoveListener(ToggleConnection);
        
        if (clearLogButton != null)
            clearLogButton.onClick.RemoveListener(ClearLog);
        
        if (testMessageButton != null)
            testMessageButton.onClick.RemoveListener(SendTestMessage);
    }
} 