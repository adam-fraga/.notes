# Go WebSocket Implementation Guide

## Overview

WebSocket is a protocol that enables two-way communication between a client and server over a single, long-lived connection. This guide demonstrates how to implement WebSocket functionality in Go using the Gorilla WebSocket package.

## Installation

First, install the Gorilla WebSocket package:

```bash
go get github.com/gorilla/websocket
```

## Basic Server Implementation

### Server Code

```go
package websockets

import (
    "fmt"
    "github.com/gorilla/websocket"
    "log"
    "net/http"
)

// upgrader converts HTTP server connections to WebSocket connections.
var upgrader = websocket.Upgrader{
    // Enable CORS
    CheckOrigin: func(r *http.Request) bool {
        return true // WARNING: Don't use in production without proper checks
    },
    // Optional: Specify buffer sizes
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
}

// handleWebSocket manages a single WebSocket connection
func handleWebSocket(w http.ResponseWriter, r *http.Request) {
    // Upgrade HTTP connection to WebSocket
    conn, err := upgrader.Upgrade(w, r, nil)
    if err != nil {
        log.Println("Upgrade error:", err)
        return
    }
    defer conn.Close()

    // Main message loop
    for {
        // Read incoming message
        messageType, msg, err := conn.ReadMessage()
        if err != nil {
            log.Println("Read error:", err)
            break
        }
        fmt.Println("Received:", string(msg))

        // Echo message back to client
        if err := conn.WriteMessage(messageType, msg); err != nil {
            log.Println("Write error:", err)
            break
        }
    }
}

// WS initializes and starts the WebSocket server
func WS() {
    http.HandleFunc("/ws", handleWebSocket)
    log.Println("WebSocket server started on ws://localhost:8080/ws")
    log.Fatal(http.ListenAndServe(":8080", nil))
}
```

## Client-Side Implementation

### JavaScript Example

```javascript
// Create WebSocket connection
const socket = new WebSocket("ws://localhost:8080/ws");

// Connection opened
socket.addEventListener("open", (event) => {
  console.log("Connected to WebSocket server");
  socket.send("Hello Server!");
});

// Listen for messages
socket.addEventListener("message", (event) => {
  console.log("Message from server:", event.data);
});

// Handle errors
socket.addEventListener("error", (event) => {
  console.error("WebSocket error:", event);
});

// Connection closed
socket.addEventListener("close", (event) => {
  console.log("Disconnected from server");
});
```

## Advanced Features

### Message Types

The Gorilla WebSocket package supports different types of messages:

```go
// Text message
conn.WriteMessage(websocket.TextMessage, []byte("Hello"))

// Binary message
conn.WriteMessage(websocket.BinaryMessage, []byte{1, 2, 3})

// Ping message
conn.WriteMessage(websocket.PingMessage, []byte{})

// Close message
conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""))
```

### Configuration Options

```go
// Extended upgrader configuration
var upgrader = websocket.Upgrader{
    CheckOrigin: func(r *http.Request) bool {
        // Implement proper origin checking
        return true
    },
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
    HandshakeTimeout: 10 * time.Second,
    EnableCompression: true,
}
```

### Error Handling and Connection Management

```go
// Example of enhanced error handling
func handleWebSocket(w http.ResponseWriter, r *http.Request) {
    conn, err := upgrader.Upgrade(w, r, nil)
    if err != nil {
        log.Printf("Upgrade error: %v", err)
        return
    }
    defer conn.Close()

    // Set read deadline
    conn.SetReadDeadline(time.Now().Add(60 * time.Second))

    // Set ping handler
    conn.SetPingHandler(func(string) error {
        conn.SetReadDeadline(time.Now().Add(60 * time.Second))
        return nil
    })

    // Handle messages
    for {
        messageType, message, err := conn.ReadMessage()
        if err != nil {
            if websocket.IsUnexpectedCloseError(err,
                websocket.CloseGoingAway,
                websocket.CloseAbnormalClosure) {
                log.Printf("Error: %v", err)
            }
            break
        }
        // Process message...
    }
}
```

## Security Considerations

1. **Origin Checking**: In production, implement proper origin checking:

```go
var upgrader = websocket.Upgrader{
    CheckOrigin: func(r *http.Request) bool {
        origin := r.Header.Get("Origin")
        return origin == "https://trusted-site.com"
    },
}
```

2. **Connection Timeouts**: Implement timeouts to prevent resource exhaustion:

```go
conn.SetReadDeadline(time.Now().Add(60 * time.Second))
conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
```

3. **Message Size Limits**: Set maximum message size:

```go
conn.SetReadLimit(512) // Limit messages to 512 bytes
```

## Best Practices

1. **Always Close Connections**: Use defer to ensure connections are closed:

```go
defer conn.Close()
```

2. **Handle Connection Errors Gracefully**:

```go
if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
    log.Printf("Error: %v", err)
}
```

3. **Implement Heartbeat**:

```go
// On server
conn.SetPingHandler(func(string) error {
    conn.SetReadDeadline(time.Now().Add(60 * time.Second))
    return nil
})

// On client (JavaScript)
setInterval(() => {
    socket.send("ping");
}, 30000);
```

## Common Use Cases

1. **Broadcasting Messages**:

```go
// Maintain a list of connections
var clients = make(map[*websocket.Conn]bool)
var broadcast = make(chan []byte)

// Broadcast messages to all clients
func broadcaster() {
    for {
        message := <-broadcast
        for client := range clients {
            err := client.WriteMessage(websocket.TextMessage, message)
            if err != nil {
                log.Printf("Broadcast error: %v", err)
                client.Close()
                delete(clients, client)
            }
        }
    }
}
```

2. **Handling JSON Messages**:

```go
type Message struct {
    Type    string `json:"type"`
    Content string `json:"content"`
}

// Read JSON
var msg Message
err := conn.ReadJSON(&msg)

// Write JSON
err := conn.WriteJSON(Message{
    Type:    "response",
    Content: "Message received",
})
```

This documentation provides a foundation for implementing WebSocket functionality in Go. Remember to adapt the security measures and error handling according to your specific needs.
