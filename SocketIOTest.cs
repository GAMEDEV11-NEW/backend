using UnityEngine;
using UnityEngine.UI;
using TMPro;

public class SocketIOTest : MonoBehaviour
{
    private SocketIOManager socketManager;
    public TextMeshProUGUI statusText; // Reference to UI text showing connection status
    public TextMeshProUGUI messageLog;  // Reference to UI text showing message log
    public TMP_InputField messageInput; // Reference to input field for sending messages
    public Button sendButton;           // Reference to send button

    private void Start()
    {
        // Get reference to SocketIOManager
        socketManager = FindObjectOfType<SocketIOManager>();
        
        if (socketManager == null)
        {
            Debug.LogError("SocketIOManager not found in scene!");
            return;
        }

        // Add button listener
        if (sendButton != null)
        {
            sendButton.onClick.AddListener(SendTestMessage);
        }

        // Start periodic status updates
        InvokeRepeating("UpdateStatus", 0f, 1f);
    }

    private void UpdateStatus()
    {
        if (statusText != null)
        {
            statusText.text = $"Server URL: {socketManager.serverUrl}\n" +
                            $"Connection Status: {(socketManager.IsConnected ? "Connected" : "Disconnected")}";
        }
    }

    public void SendTestMessage()
    {
        if (messageInput != null && !string.IsNullOrEmpty(messageInput.text))
        {
            // Send the message
            socketManager.SendMessage(messageInput.text);
            
            // Log the sent message
            AddToMessageLog($"Sent: {messageInput.text}");
            
            // Clear input field
            messageInput.text = "";
        }
    }

    // Call this method to add messages to the log
    public void AddToMessageLog(string message)
    {
        if (messageLog != null)
        {
            messageLog.text = $"{message}\n{messageLog.text}";
            // Keep only last 10 messages
            var lines = messageLog.text.Split('\n');
            if (lines.Length > 10)
            {
                messageLog.text = string.Join("\n", lines[..10]);
            }
        }
    }

    private void OnDestroy()
    {
        if (sendButton != null)
        {
            sendButton.onClick.RemoveListener(SendTestMessage);
        }
    }
} 