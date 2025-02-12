# Package stdlib: JSON Handling in Go

## Overview

The `encoding/json` package in Go provides functions to encode (marshal) and decode (unmarshal) JSON data. This document outlines various ways to work with JSON using Go's standard library.

---

## 1. Defining Structs with JSON Tags

JSON struct tags help control how fields are serialized and deserialized.

### Example: Basic Struct

```go
// User struct with JSON tags
type User struct {
    ID       int                    `json:"id"`
    Name     string                 `json:"name"`
    Email    string                 `json:"email,omitempty"` // Omits field if empty
    Password string                 `json:"-"`               // Ignores field in JSON
    Roles    []string               `json:"roles,omitempty"`
    Active   bool                   `json:"active"`
    Address  *Address               `json:"address,omitempty"`  // Pointer for nested struct
    Metadata map[string]interface{} `json:"metadata,omitempty"` // For dynamic JSON
}

// Address struct for nested JSON
type Address struct {
    Street  string `json:"street"`
    City    string `json:"city"`
    Country string `json:"country"`
}
```

---

## 2. Encoding Structs to JSON

### Example: Encoding and Pretty Printing JSON

```go
func demonstrateBasicEncoding() {
    user := User{
        ID:     1,
        Name:   "John Doe",
        Email:  "john@example.com",
        Active: true,
        Roles:  []string{"admin", "user"},
        Address: &Address{
            Street:  "123 Main St",
            City:    "New York",
            Country: "USA",
        },
    }

    jsonData, err := json.Marshal(user)
    if err != nil {
        log.Printf("Error marshaling: %v", err)
        return
    }
    fmt.Printf("Encoded JSON: %s\n", jsonData)

    prettyJSON, _ := json.MarshalIndent(user, "", "    ")
    fmt.Printf("Pretty JSON:\n%s\n", prettyJSON)
}
```

---

## 3. Decoding JSON into Structs

### Example: Decoding JSON

```go
func demonstrateBasicDecoding() {
    jsonStr := `{
        "id": 1,
        "name": "John Doe",
        "email": "john@example.com",
        "active": true,
        "roles": ["admin", "user"],
        "address": {
            "street": "123 Main St",
            "city": "New York",
            "country": "USA"
        }
    }`

    var user User
    err := json.Unmarshal([]byte(jsonStr), &user)
    if err != nil {
        log.Printf("Error unmarshaling: %v", err)
        return
    }
    fmt.Printf("Decoded struct: %+v\n", user)
}
```

---

## 4. Working with Dynamic JSON

### Example: Using `map[string]interface{}`

```go
func demonstrateDynamicJSON() {
    jsonStr := `{
        "name": "Dynamic Data",
        "values": {
            "number": 42,
            "boolean": true,
            "nested": {
                "array": [1, 2, 3]
            }
        }
    }`

    var data map[string]interface{}
    if err := json.Unmarshal([]byte(jsonStr), &data); err != nil {
        log.Printf("Error parsing dynamic JSON: %v", err)
        return
    }
    fmt.Printf("Parsed data: %+v\n", data)
}
```

---

## 5. Custom JSON Marshalling and Unmarshalling

### Example: Custom Type for Duration

```go
type Duration struct {
    Hours   int
    Minutes int
}

// MarshalJSON implements custom JSON encoding
func (d Duration) MarshalJSON() ([]byte, error) {
    return json.Marshal(fmt.Sprintf("%dh%dm", d.Hours, d.Minutes))
}

// UnmarshalJSON implements custom JSON decoding
func (d *Duration) UnmarshalJSON(data []byte) error {
    var str string
    if err := json.Unmarshal(data, &str); err != nil {
        return err
    }
    _, err := fmt.Sscanf(str, "%dh%dm", &d.Hours, &d.Minutes)
    return err
}
```

---

## 6. Handling JSON Arrays

### Example: Decoding JSON Arrays

```go
func demonstrateJSONArrays() {
    jsonArray := `[
        {"name": "John", "age": 30},
        {"name": "Jane", "age": 25},
        {"name": "Bob", "age": 35}
    ]`

    var users []struct {
        Name string `json:"name"`
        Age  int    `json:"age"`
    }

    err := json.Unmarshal([]byte(jsonArray), &users)
    if err != nil {
        log.Printf("Error unmarshaling array: %v", err)
        return
    }

    fmt.Printf("Decoded users array: %+v\n", users)
}
```

---

## 7. Error Handling in JSON Processing

### Example: Handling Errors

```go
func demonstrateErrorHandling() {
    invalidJSON := `{"name": "John", "age": invalid}`

    var data struct {
        Name string `json:"name"`
        Age  int    `json:"age"`
    }

    err := json.Unmarshal([]byte(invalidJSON), &data)
    if err != nil {
        fmt.Printf("Error handling example - Invalid JSON: %v\n", err)
    }
}
```

---

## Conclusion

The `encoding/json` package in Go offers a robust way to handle JSON encoding and decoding. This document covered fundamental operations, dynamic JSON handling, custom marshaling, JSON arrays, and error handling.

For more details, refer to the [official Go documentation](https://pkg.go.dev/encoding/json).
