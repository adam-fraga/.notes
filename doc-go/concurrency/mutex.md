package concurrency

import (
	"fmt"
	"sync"
)

// Counter is a struct that holds a value and a mutex
// to ensure safe concurrent access.
type Counter struct {
	mu    sync.Mutex // Mutex to protect shared resource
	value int        // Shared resource
}

// Increment increases the counter value safely
func (c *Counter) Increment() {
	c.mu.Lock()         // Lock the mutex before modifying the shared resource
	defer c.mu.Unlock() // Ensure the mutex is unlocked after function execution
	c.value++
}

// GetValue safely retrieves the counter value
func (c *Counter) GetValue() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.value
}

func Mutex() {
	var wg sync.WaitGroup
	counter := Counter{}

	// Simulate concurrent increments (Span 10 go routine that increment Counter concurrently)
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			counter.Increment()
		}()
	}

	wg.Wait() // Wait for all goroutines to finish
	fmt.Println("Final Counter Value:", counter.GetValue())
}
