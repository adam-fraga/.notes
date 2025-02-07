package main

import (
	"context"
	"fmt"
	"time"
)

/*
Context Package Summary:
------------------------
The `context` package in Go is used for managing deadlines, cancellations,
and carrying request-scoped values across API boundaries. It is commonly
used in goroutines to handle timeouts and prevent resource leaks.

Features:
1. context.Background() - Creates a base context, typically used as a root.
2. context.WithTimeout() - Creates a context that automatically cancels after a timeout.
3. context.WithDeadline() - Creates a context that cancels at a specific time.
4. context.WithCancel() - Creates a context that can be manually canceled.
5. context.WithValue() - Attaches key-value pairs to a context for passing metadata.
*/

// doWork simulates a task that checks for context cancellation
// and stops execution when the context is canceled or times out.
func doWork(ctx context.Context) {
	for {
		select {
		case <-ctx.Done(): // Listens for the context's cancellation signal
			fmt.Println("Work canceled or timed out!")
			return
		default:
			fmt.Println("Working...")
			time.Sleep(500 * time.Millisecond) // Simulates ongoing work
		}
	}
}

func Ctx() {
	// Create a base context
	baseCtx := context.Background()

	// Create a context with a timeout of 2 seconds
	timeoutCtx, cancelTimeout := context.WithTimeout(baseCtx, 2*time.Second)
	defer cancelTimeout() // Ensures the context is canceled when main exits

	// Create a context with a deadline
	deadlineCtx, cancelDeadline := context.WithDeadline(baseCtx, time.Now().Add(3*time.Second))
	defer cancelDeadline() // Ensures the context is canceled when main exits

	// Create a context with manual cancellation
	cancelCtx, cancelFunc := context.WithCancel(baseCtx)
	defer cancelFunc()

	// Create a context with a value
	valueCtx := context.WithValue(baseCtx, "requestID", "12345")

	// Start goroutines that listen to different contexts
	go doWork(timeoutCtx)
	go doWork(deadlineCtx)
	go doWork(cancelCtx)

	// Simulating canceling the manual cancel context after 1 second
	time.Sleep(1 * time.Second)
	fmt.Println("Manually canceling context")
	cancelFunc()

	// Retrieve and print value from context
	requestID := valueCtx.Value("requestID").(string)
	fmt.Println("Request ID from context:", requestID)

	// Wait before exiting
	time.Sleep(4 * time.Second)
	fmt.Println("Main function exiting")
}
