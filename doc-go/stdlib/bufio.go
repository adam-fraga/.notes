// This example demonstrates common uses of the bufio package in Go for efficient I/O operations.
// The bufio package implements buffered I/O, providing ways to efficiently read from and write to files/streams.

// Each example is thoroughly commented to explain what's happening. Some key points about bufio:
//
// The Scanner interface is great for reading line-by-line or with custom split functions
// Writers buffer the output to reduce the number of system calls
// Always remember to Flush() the writer when you're done
// You can customize buffer sizes for performance optimization
// Error handling is important for all I/O operations
//
// Would you like me to explain any particular aspect in more detail or add more examples for specific bufio features?

package stdlib

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func BUFIO() {
	// Example 1: Reading a file line by line using bufio.Scanner
	fmt.Println("Example 1: Reading file line by line")
	file, err := os.Open("sample.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	// Scanner reads the input by lines by default
	for scanner.Scan() {
		fmt.Println(scanner.Text()) // Text() returns the current line as string
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}

	// Example 2: Reading from standard input
	fmt.Println("\nExample 2: Reading from standard input")
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter text: ")
	// ReadString reads until the first occurrence of delimiter
	text, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println("Error reading input:", err)
	}
	fmt.Printf("You entered: %s", text)

	// Example 3: Using Scanner with custom split function
	fmt.Println("\nExample 3: Scanner with custom split (words)")
	input := strings.NewReader("This is a sample text for word scanning")
	wordScanner := bufio.NewScanner(input)
	wordScanner.Split(bufio.ScanWords) // Set scanner to scan by words
	for wordScanner.Scan() {
		fmt.Printf("Word: %s\n", wordScanner.Text())
	}

	// Example 4: Writing with bufio.Writer
	fmt.Println("\nExample 4: Buffered writing")
	outputFile, err := os.Create("output.txt")
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer outputFile.Close()

	writer := bufio.NewWriter(outputFile)
	// WriteString writes string to buffer
	_, err = writer.WriteString("This is a buffered write example.\n")
	if err != nil {
		fmt.Println("Error writing to buffer:", err)
		return
	}

	// Important: Flush writes buffered data to underlying io.Writer
	err = writer.Flush()
	if err != nil {
		fmt.Println("Error flushing buffer:", err)
		return
	}

	// Example 5: Using Scanner with custom buffer size
	fmt.Println("\nExample 5: Scanner with custom buffer")
	largeInput := strings.NewReader("Very large text content...")
	customScanner := bufio.NewScanner(largeInput)
	customScanner.Buffer(make([]byte, 64*1024), 1024*1024) // Set buffer size to 64KB, max token size to 1MB

	for customScanner.Scan() {
		fmt.Println(customScanner.Text())
	}
}

// Example 6: Helper function demonstrating ReadBytes
func readUntilDelimiter(reader *bufio.Reader, delimiter byte) {
	data, err := reader.ReadBytes(delimiter)
	if err != nil {
		fmt.Println("Error reading bytes:", err)
		return
	}
	fmt.Printf("Read until delimiter: %s\n", string(data))
}

// Example 7: Helper function demonstrating Peek
func peekNextBytes(reader *bufio.Reader, n int) {
	bytes, err := reader.Peek(n)
	if err != nil {
		fmt.Println("Error peeking bytes:", err)
		return
	}
	fmt.Printf("Next %d bytes: %s\n", n, string(bytes))
}
