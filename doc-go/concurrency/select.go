package concurrency

import (
	"fmt"
	"time"
)

func Select() {
	ch1 := make(chan string)
	ch2 := make(chan string)

	// Simulate asynchronous operations
	go func() {
		time.Sleep(2 * time.Second)
		ch1 <- "Message from channel 1"
	}()

	go func() {
		time.Sleep(1 * time.Second)
		ch2 <- "Message from channel 2"
	}()

	// Using select to wait for messages from multiple channels
	for i := 0; i < 2; i++ {
		select {
		case msg1 := <-ch1:
			fmt.Println("Received:", msg1)
		case msg2 := <-ch2:
			fmt.Println("Received:", msg2)
		case <-time.After(3 * time.Second):
			fmt.Println("Timeout: No message received")
		}
	}
}
