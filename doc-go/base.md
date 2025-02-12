# Go Programming Language Reference Documentation

## Table of Contents

- [Basic Types and Zero Values](#basic-types-and-zero-values)
- [Variables and Declarations](#variables-and-declarations)
- [Constants](#constants)
- [Control Structures](#control-structures)
- [Functions](#functions)
- [Structures and Methods](#structures-and-methods)
- [Interfaces](#interfaces)
- [Error Handling](#error-handling)
- [Concurrency](#concurrency)
- [Collections](#collections)
- [Packages and Modules](#packages-and-modules)
- [Common Operations](#common-operations)
- [Best Practices](#best-practices)

## Basic Types and Zero Values

### Built-in Types

```go
bool        // false
string      // ""
int         // 0
int8        // 0
int16       // 0
int32       // 0
int64       // 0
uint        // 0
uint8       // 0
uint16      // 0
uint32      // 0
uint64      // 0
float32     // 0.0
float64     // 0.0
complex64   // 0+0i
complex128  // 0+0i
```

### Reference Types (Zero Value: nil)

```go
pointer     // nil
slice       // nil
map         // nil
channel     // nil
interface   // nil
function    // nil
```

## Variables and Declarations

### Multiple Declaration Styles

```go
// Single variable
var name string = "John"
var age = 25        // Type inference
name := "John"      // Short declaration (inside functions only)

// Multiple variables
var x, y int = 1, 2
var (
    name    string = "John"
    age     int    = 25
    isValid bool   = true
)

// Short declaration multiple variables
x, y := 0, 1
```

### Type Conversion

```go
// Basic type conversion
var i int = 42
var f float64 = float64(i)
var s string = strconv.Itoa(i)    // Integer to string
var i2 int = strconv.Atoi("42")   // String to integer

// Array to slice
arr := [3]int{1, 2, 3}
slice := arr[:]

// String conversion
str := string([]byte{'h', 'e', 'l', 'l', 'o'})
bytes := []byte("hello")
```

## Control Structures

### Conditional Statements

```go
// If statement with initialization
if value := getValue(); value > 10 {
    fmt.Println("Value is greater than 10")
} else if value < 0 {
    fmt.Println("Value is negative")
} else {
    fmt.Println("Value is between 0 and 10")
}

// Switch statement
switch os := runtime.GOOS; os {
case "darwin":
    fmt.Println("OS X")
case "linux":
    fmt.Println("Linux")
default:
    fmt.Printf("%s\n", os)
}

// Switch without condition (clean if-else)
switch {
case value > 100:
    fmt.Println("Large value")
case value < 0:
    fmt.Println("Negative value")
default:
    fmt.Println("Normal value")
}
```

### Loops

```go
// Standard for loop
for i := 0; i < 10; i++ {
    fmt.Println(i)
}

// While-style loop
n := 0
for n < 5 {
    n++
}

// Infinite loop
for {
    // Break to exit
    break
    // Continue to next iteration
    continue
}

// Range loops
for index, value := range slice {
    fmt.Printf("Index: %d, Value: %v\n", index, value)
}

for key, value := range map[string]int{
    "one": 1,
    "two": 2,
} {
    fmt.Printf("Key: %s, Value: %d\n", key, value)
}
```

## Functions

### Function Declarations

```go
// Basic function
func add(x, y int) int {
    return x + y
}

// Multiple return values
func divide(x, y float64) (float64, error) {
    if y == 0 {
        return 0, errors.New("division by zero")
    }
    return x / y, nil
}

// Named return values
func split(sum int) (x, y int) {
    x = sum * 4 / 9
    y = sum - x
    return  // Naked return
}

// Variadic function
func sum(nums ...int) int {
    total := 0
    for _, num := range nums {
        total += num
    }
    return total
}

// Function with function parameter
func compute(fn func(float64, float64) float64) float64 {
    return fn(3, 4)
}
```

### Closures and Anonymous Functions

```go
// Anonymous function
func() {
    fmt.Println("Immediately invoked")
}()

// Closure
func adder() func(int) int {
    sum := 0
    return func(x int) int {
        sum += x
        return sum
    }
}
```

### Defer Statement

```go
func readFile(filename string) error {
    file, err := os.Open(filename)
    if err != nil {
        return err
    }
    defer file.Close()  // Will be called when function returns

    // Process file...
    return nil
}

// Multiple defers (LIFO order)
func printNumbers() {
    for i := 0; i < 3; i++ {
        defer fmt.Printf("%d ", i)
    }
}
```

## Structures and Methods

### Structure Declaration and Initialization

```go
type Person struct {
    Name    string
    Age     int
    Address *Address
}

type Address struct {
    Street  string
    City    string
    Country string
}

// Creating instances
person1 := Person{Name: "John", Age: 30}
person2 := &Person{  // Pointer to struct
    Name: "Jane",
    Age:  25,
}
```

### Methods

```go
// Value receiver
func (p Person) GetInfo() string {
    return fmt.Sprintf("%s is %d years old", p.Name, p.Age)
}

// Pointer receiver
func (p *Person) SetAge(age int) {
    p.Age = age
}

// Method with embedded struct
type Employee struct {
    Person
    Salary float64
}

func (e Employee) GetSalaryInfo() string {
    return fmt.Sprintf("%s earns %.2f", e.Name, e.Salary)
}
```

## Interfaces

### Interface Declaration and Implementation

```go
type Writer interface {
    Write([]byte) (int, error)
}

type Reader interface {
    Read(p []byte) (n int, err error)
}

// Combining interfaces
type ReadWriter interface {
    Reader
    Writer
}

// Empty interface
type Any interface{}

// Type assertion
value, ok := someInterface.(string)
if ok {
    fmt.Printf("Value is string: %s\n", value)
}

// Type switch
switch v := someInterface.(type) {
case int:
    fmt.Printf("Integer: %d\n", v)
case string:
    fmt.Printf("String: %s\n", v)
default:
    fmt.Printf("Unknown type\n")
}
```

## Error Handling

### Error Creation and Handling

```go
// Creating errors
var ErrNotFound = errors.New("not found")

// Custom error type
type ValidationError struct {
    Field string
    Message string
}

func (e *ValidationError) Error() string {
    return fmt.Sprintf("%s: %s", e.Field, e.Message)
}

// Error handling
func divide(a, b float64) (float64, error) {
    if b == 0 {
        return 0, fmt.Errorf("division by zero")
    }
    return a / b, nil
}

// Using errors.Is and errors.As
if errors.Is(err, ErrNotFound) {
    // Handle not found error
}

var validationErr *ValidationError
if errors.As(err, &validationErr) {
    // Handle validation error
}
```

## Concurrency

### Goroutines and Channels

```go
// Starting a goroutine
go func() {
    // Do something concurrently
}()

// Channel operations
ch := make(chan int)        // Unbuffered channel
buffCh := make(chan int, 5) // Buffered channel

// Send and receive
ch <- 42      // Send
value := <-ch // Receive

// Select statement
select {
case v1 := <-ch1:
    fmt.Println("Received from ch1:", v1)
case v2 := <-ch2:
    fmt.Println("Received from ch2:", v2)
case ch3 <- 42:
    fmt.Println("Sent to ch3")
default:
    fmt.Println("No channel ready")
}
```

### Synchronization

```go
// WaitGroup
var wg sync.WaitGroup
wg.Add(1)
go func() {
    defer wg.Done()
    // Do work
}()
wg.Wait()

// Mutex
var mu sync.Mutex
mu.Lock()
// Critical section
mu.Unlock()

// RWMutex
var rwmu sync.RWMutex
rwmu.RLock()  // Read lock
// Read operations
rwmu.RUnlock()
```

## Collections

### Slices

```go
// Creating slices
slice1 := make([]int, 5)    // Length 5, capacity 5
slice2 := make([]int, 0, 5) // Length 0, capacity 5
slice3 := []int{1, 2, 3}    // With values

// Slice operations
slice = append(slice, 4)     // Append
slice = slice[1:4]          // Slicing
copy(dst, src)              // Copy

// Common patterns
// Remove from middle
slice = append(slice[:i], slice[i+1:]...)
// Clear slice
slice = slice[:0]
```

### Maps

```go
// Creating maps
m1 := make(map[string]int)
m2 := map[string]int{
    "one": 1,
    "two": 2,
}

// Map operations
m1["three"] = 3        // Insert
value, exists := m1["three"] // Check existence
delete(m1, "three")    // Delete

// Iterate over map
for key, value := range m1 {
    fmt.Printf("%s: %d\n", key, value)
}
```

## Best Practices

### Error Handling

```go
// Prefer early returns
func process(data []byte) error {
    if len(data) == 0 {
        return errors.New("empty data")
    }

    if err := validateData(data); err != nil {
        return fmt.Errorf("invalid data: %w", err)
    }

    return nil
}

// Use type assertions safely
value, ok := interface{}.(Type)
if !ok {
    return errors.New("invalid type")
}
```

### Concurrency Patterns

```go
// Worker pool pattern
func worker(id int, jobs <-chan int, results chan<- int) {
    for j := range jobs {
        results <- j * 2
    }
}

// Pipeline pattern
func generator(nums ...int) <-chan int {
    out := make(chan int)
    go func() {
        for _, n := range nums {
            out <- n
        }
        close(out)
    }()
    return out
}
```

### Performance Tips

```go
// Preallocate slices when length is known
slice := make([]int, 0, expectedSize)

// Use strings.Builder for string concatenation
var builder strings.Builder
builder.WriteString("Hello")
builder.WriteString(" ")
builder.WriteString("World")
result := builder.String()

// Use sync.Pool for frequently allocated objects
var pool = sync.Pool{
    New: func() interface{} {
        return make([]byte, 1024)
    },
}
```

## Common Operations

### String Operations

```go
// String manipulation
s := strings.Split("a,b,c", ",")    // Split
s = strings.Join([]string{"a", "b"}, ",") // Join
s = strings.Trim(" hello ", " ")    // Trim
s = strings.ToLower("Hello")        // Convert case
contains := strings.Contains("hello", "ll") // Check substring
```

### File Operations

```go
// Reading file
data, err := ioutil.ReadFile("file.txt")
if err != nil {
    log.Fatal(err)
}

// Writing file
err = ioutil.WriteFile("file.txt", []byte("Hello"), 0644)
if err != nil {
    log.Fatal(err)
}

// Reading file line by line
file, err := os.Open("file.txt")
if err != nil {
    log.Fatal(err)
}
defer file.Close()

scanner := bufio.NewScanner(file)
for scanner.Scan() {
    fmt.Println(scanner.Text())
}
```

## Packages and Modules

### Module Initialization

```bash
# Initialize a new module
go mod init example.com/myproject

# Add dependencies
go get github.com/some/dependency

# Update dependencies
go get -u
```

### Package Structure

```go
package main

import (
    "fmt"

    "example.com/myproject/pkg/models"
    "example.com/myproject/pkg/handlers"
)

func main() {
    // Use imported packages
}
```

_Note: This documentation covers the fundamentals of Go programming. For more detailed information, refer to the official Go documentation at https://golang.org/doc/._
