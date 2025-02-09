// This example demonstrates common uses of the context package in Go.
// The context package is used for carrying deadlines, cancellation signals,
// and request-scoped values across API boundaries and between processes.

// Key concepts demonstrated:
//
// Always use defer cancel() to prevent context leaks
// Context values are passed through the entire chain
// Multiple goroutines can share the same context
// Different types of context termination (timeout, deadline, cancellation)
// Proper error handling patterns
//
// Some important points about context usage:
//
// Context should be the first parameter of a function
// Never store contexts in structs
// Key type should be unexported to avoid collisions
// Always propagate context when making downstream calls
// Use context values sparingly, mainly for request-scoped data
package stdlib

import (
	"context"
	"fmt"
	"time"
)

// SimulateDBQuery simulates a database operation that takes time
func SimulateDBQuery(ctx context.Context) (string, error) {
	// Create a channel to simulate work
	select {
	case <-time.After(2 * time.Second):
		return "Query Result", nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

// ProcessWithValue demonstrates context with values
func ProcessWithValue(ctx context.Context) {
	// Get value from context
	if userID, ok := ctx.Value("userID").(string); ok {
		fmt.Printf("Processing request for user: %s\n", userID)
	}

	// Get multiple values
	if authToken, ok := ctx.Value("authToken").(string); ok {
		fmt.Printf("Using auth token: %s\n", authToken)
	}
}

// Worker simulates a long-running task
func Worker(ctx context.Context, id int) {
	for {
		select {
		case <-ctx.Done():
			fmt.Printf("Worker %d: Stopped due to context cancellation\n", id)
			return
		default:
			fmt.Printf("Worker %d: Working...\n", id)
			time.Sleep(time.Second)
		}
	}
}

func Ctx() {
	fmt.Println("Context Package Examples")
	fmt.Println("------------------------")

	// Example 1: Context with timeout
	fmt.Println("\n1. Context with Timeout:")
	timeoutCtx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel() // Always defer cancel to avoid context leak

	result, err := SimulateDBQuery(timeoutCtx)
	if err != nil {
		fmt.Printf("Query failed: %v\n", err)
	} else {
		fmt.Printf("Query succeeded: %s\n", result)
	}

	// Example 2: Context with deadline
	fmt.Println("\n2. Context with Deadline:")
	deadline := time.Now().Add(1 * time.Second)
	deadlineCtx, cancelDeadline := context.WithDeadline(context.Background(), deadline)
	defer cancelDeadline()

	select {
	case <-deadlineCtx.Done():
		fmt.Printf("Context deadline exceeded: %v\n", deadlineCtx.Err())
	case <-time.After(2 * time.Second):
		fmt.Println("This won't be reached due to deadline")
	}

	// Example 3: Context with cancellation
	fmt.Println("\n3. Context with Cancellation:")
	cancelCtx, cancelFunc := context.WithCancel(context.Background())

	// Start worker
	go Worker(cancelCtx, 1)

	// Let it work for 2 seconds
	time.Sleep(2 * time.Second)

	// Cancel the context
	cancelFunc()
	time.Sleep(time.Second) // Give worker time to stop

	// Example 4: Context with values
	fmt.Println("\n4. Context with Values:")
	// Create a context with values
	baseCtx := context.Background()
	userCtx := context.WithValue(baseCtx, "userID", "user123")
	authCtx := context.WithValue(userCtx, "authToken", "token456")

	ProcessWithValue(authCtx)

	// Example 5: Context chain with multiple operations
	fmt.Println("\n5. Context Chain:")
	// Create a timeout context
	chainCtx, chainCancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer chainCancel()

	// Add values to the context chain
	chainCtx = context.WithValue(chainCtx, "requestID", "req789")
	chainCtx = context.WithValue(chainCtx, "traceID", "trace101")

	// Start multiple workers with the same context
	for i := 1; i <= 3; i++ {
		go Worker(chainCtx, i)
	}

	// Let workers run for 2 seconds
	time.Sleep(2 * time.Second)
	chainCancel()           // Cancel all workers
	time.Sleep(time.Second) // Give workers time to stop

	// Example 6: Handling context errors
	fmt.Println("\n6. Context Error Handling:")
	errorCtx, errorCancel := context.WithTimeout(context.Background(), time.Millisecond)
	defer errorCancel()

	// Wait for more than the timeout
	time.Sleep(10 * time.Millisecond)

	// Check context errors
	switch errorCtx.Err() {
	case context.DeadlineExceeded:
		fmt.Println("Context deadline exceeded")
	case context.Canceled:
		fmt.Println("Context was canceled")
	case nil:
		fmt.Println("No error")
	default:
		fmt.Printf("Other error: %v\n", errorCtx.Err())
	}
}
