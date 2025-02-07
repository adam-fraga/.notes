package main

import (
	"fmt"
	"sync"
	"time"
)

func do_something(wg *sync.WaitGroup, c chan []string) {
	// Say to the waiting group that we are done after the function finishes
	defer wg.Done()
	fmt.Print("\nDoing something")
	// Send data to the channel
	c <- []string{"a", "b", "c"}
	for i := 0; i < 6; i++ {
		fmt.Print(".")
		time.Sleep(500 * time.Millisecond)
	}
	fmt.Println("")
}

func do_anotherthing(wg *sync.WaitGroup, c chan []string) {
	defer wg.Done()
	fmt.Print("\nDo another thing")
	c <- []string{"x", "y", "z"}
	for i := 0; i < 6; i++ {
		fmt.Print("-")
		time.Sleep(500 * time.Millisecond)
	}
	fmt.Println("")
}

func do_something_else(wg *sync.WaitGroup, c chan []string) {
	defer wg.Done()
	fmt.Print("\nDo something else")
	c <- []string{"d", "e", "f"}
	for i := 0; i < 6; i++ {
		fmt.Print("*")
		time.Sleep(500 * time.Millisecond)
	}
	fmt.Println("")
}

func main() {
	//Create a wait group that store go routine
	wg := sync.WaitGroup{}
	c := make(chan []string)
	// Add 3 (number of goroutines) to the waiting group
	wg.Add(3)

	// Run the goroutines and pass the waiting group as a reference and the channel
	go do_something(&wg, c)
	go do_anotherthing(&wg, c)
	go do_something_else(&wg, c)

	go func() {
		// Loop through the channel and print the data
		for data := range c {
			fmt.Println("DATA", data)
		}
	}()

	// Wait until all the goroutines are done
	wg.Wait()
	// Close the channel
	close(c)
}
