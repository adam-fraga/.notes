// This example demonstrates common uses of the context package in Go within an HTTP server.
// The context package is used for carrying deadlines, cancellation signals,
// and request-scoped values across API boundaries and between processes.

// Key concepts demonstrated:
//
// - Always use defer cancel() to prevent context leaks
// - Context values are passed through the entire chain
// - Multiple goroutines can share the same context
// - Different types of context termination (timeout, deadline, cancellation)
// - Proper error handling patterns in HTTP handlers
//
// Some important points about context usage:
//
// - Context should be the first parameter of a function
// - Never store contexts in structs
// - Key type should be unexported to avoid collisions
// - Always propagate context when making downstream calls
// - Use context values sparingly, mainly for request-scoped data

package stdlib

import (
	"context"
	"fmt"
	"html/template"
	"net/http"
	"time"
)

// SimulateDBQuery simulates a database operation that takes time
func SimulateDBQuery(ctx context.Context) (string, error) {
	select {
	case <-time.After(2 * time.Second):
		return "Query Result", nil
	case <-ctx.Done():
		return "", ctx.Err()
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

// HandleWithTimeout demonstrates using context with timeouts in an HTTP handler
func HandleWithTimeout(w http.ResponseWriter, r *http.Request) {
	// Create a context with a timeout
	ctx, cancel := context.WithTimeout(r.Context(), 3*time.Second)
	defer cancel()

	result, err := SimulateDBQuery(ctx)
	if err != nil {
		http.Error(w, "Query failed: "+err.Error(), http.StatusGatewayTimeout)
		return
	}

	fmt.Fprintf(w, "Query succeeded: %s", result)
}

// HandleWorker demonstrates launching a worker that stops when the request context is canceled
func HandleWorker(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	go Worker(ctx, 1) // Worker will stop if request is canceled

	fmt.Fprintln(w, "Worker started. Try canceling the request.")
}

// HandleTemplateRendering demonstrates passing context into templ rendering
func HandleTemplateRendering(w http.ResponseWriter, r *http.Request) {
	tmpl := `<h1>Hello, {{.Name}}</h1><p>Request ID: {{.RequestID}}</p>`

	// Extract values from context
	ctx := r.Context()
	name := ctx.Value("userName")
	if name == nil {
		name = "Guest"
	}

	// Use request ID for tracing
	requestID := ctx.Value("requestID")
	if requestID == nil {
		requestID = "unknown"
	}

	// Render template with context values
	t, _ := template.New("webpage").Parse(tmpl)
	t.Execute(w, map[string]string{
		"Name":      name.(string),
		"RequestID": requestID.(string),
	})
}

// HandleWithValues demonstrates passing values through context
func HandleWithValues(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	// Inject values into context
	ctx = context.WithValue(ctx, "userName", "Alice")
	ctx = context.WithValue(ctx, "requestID", "req-12345")

	HandleTemplateRendering(w, r.WithContext(ctx))
}

func mainFunc() {
	mux := http.NewServeMux()
	//   /timeout: Uses context.WithTimeout() to automatically cancel long-running queries if they exceed 3 seconds.
	// /worker: Starts a worker that stops when the request is canceled.
	// /template: Demonstrates passing context values into templ-based rendering.
	// /context-values: Injects user and request ID values into context, then calls /template to use them.

	mux.HandleFunc("/timeout", HandleWithTimeout)
	mux.HandleFunc("/worker", HandleWorker)
	mux.HandleFunc("/template", HandleTemplateRendering)
	mux.HandleFunc("/context-values", HandleWithValues)

	server := &http.Server{
		Addr:    ":8080",
		Handler: mux,
	}

	fmt.Println("Starting server on :8080")
	server.ListenAndServe()
}
