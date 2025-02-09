// This example demonstrates the usage of the io package from Go's standard library
// showing various ways to handle input/output operations without using deprecated ioutil.

package stdlib

import (
	"bytes"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

// Custom reader that counts bytes read
type CountingReader struct {
	reader    io.Reader
	bytesRead int64
}

func (cr *CountingReader) Read(p []byte) (n int, err error) {
	n, err = cr.reader.Read(p)
	cr.bytesRead += int64(n)
	return n, err
}

// Custom writer that counts bytes written
type CountingWriter struct {
	writer       io.Writer
	bytesWritten int64
}

func (cw *CountingWriter) Write(p []byte) (n int, err error) {
	n, err = cw.writer.Write(p)
	cw.bytesWritten += int64(n)
	return n, err
}

func main() {
	fmt.Println("IO Package Examples")
	fmt.Println("-----------------")

	// Example 1: Basic file operations using io
	fmt.Println("\n1. Basic File Operations:")

	// Writing to a file
	data := []byte("Hello, IO Package!")
	err := os.WriteFile("test.txt", data, 0644)
	if err != nil {
		log.Fatal(err)
	}

	// Reading from a file
	content, err := os.ReadFile("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("File contents: %s\n", content)

	// Example 2: Using io.Reader and io.Writer interfaces
	fmt.Println("\n2. Reader and Writer Interfaces:")

	// Create a string reader
	reader := strings.NewReader("Hello, Reader!")

	// Create a buffer to write to
	var buffer bytes.Buffer

	// Copy from reader to writer
	bytesWritten, err := io.Copy(&buffer, reader)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Bytes written: %d\n", bytesWritten)
	fmt.Printf("Buffer contents: %s\n", buffer.String())

	// Example 3: Using io.MultiReader and io.MultiWriter
	fmt.Println("\n3. MultiReader and MultiWriter:")

	reader1 := strings.NewReader("Hello ")
	reader2 := strings.NewReader("World!")

	// Combine multiple readers
	multiReader := io.MultiReader(reader1, reader2)

	// Create multiple writers
	var buffer1, buffer2 bytes.Buffer
	multiWriter := io.MultiWriter(&buffer1, &buffer2)

	// Copy from multi-reader to multi-writer
	_, err = io.Copy(multiWriter, multiReader)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Buffer1: %s\n", buffer1.String())
	fmt.Printf("Buffer2: %s\n", buffer2.String())

	// Example 4: Using io.Pipe for concurrent IO
	fmt.Println("\n4. Using io.Pipe:")

	pipeReader, pipeWriter := io.Pipe()

	// Write in a goroutine
	go func() {
		defer pipeWriter.Close()
		data := []byte("Data through pipe!")
		_, err := pipeWriter.Write(data)
		if err != nil {
			log.Fatal(err)
		}
	}()

	// Read from pipe
	buffer = bytes.Buffer{}
	_, err = io.Copy(&buffer, pipeReader)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Pipe contents: %s\n", buffer.String())

	// Example 5: Using io.LimitReader
	fmt.Println("\n5. Using LimitReader:")

	bigReader := strings.NewReader("This is a long string that we want to read partially")
	limitedReader := io.LimitReader(bigReader, 10) // Only read 10 bytes

	limitedData, err := io.ReadAll(limitedReader)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Limited read: %s\n", limitedData)

	// Example 6: Using custom Reader/Writer with counting
	fmt.Println("\n6. Custom Reader/Writer:")

	sourceData := "Custom reader and writer test"
	source := strings.NewReader(sourceData)

	// Create counting reader and writer
	countingReader := &CountingReader{reader: source}
	countingWriter := &CountingWriter{writer: &buffer}

	buffer.Reset() // Clear buffer

	_, err = io.Copy(countingWriter, countingReader)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Bytes read: %d\n", countingReader.bytesRead)
	fmt.Printf("Bytes written: %d\n", countingWriter.bytesWritten)

	// Example 7: Using io.TeeReader
	fmt.Println("\n7. Using TeeReader:")

	source = strings.NewReader("TeeReader test")
	var teeBuffer bytes.Buffer

	// TeeReader reads from source and writes to teeBuffer
	teeReader := io.TeeReader(source, &teeBuffer)

	// Read from teeReader
	finalBuffer := make([]byte, 100)
	n, err := teeReader.Read(finalBuffer)
	if err != nil && err != io.EOF {
		log.Fatal(err)
	}

	fmt.Printf("Read from TeeReader: %s\n", finalBuffer[:n])
	fmt.Printf("Tee buffer contents: %s\n", teeBuffer.String())
}

// Helper function to demonstrate chunked reading
func readInChunks(reader io.Reader, chunkSize int) error {
	buf := make([]byte, chunkSize)
	for {
		n, err := reader.Read(buf)
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
		fmt.Printf("Read chunk: %s\n", buf[:n])
	}
	return nil
}

// Helper function to demonstrate safe file copying
func copyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return err
	}
	defer sourceFile.Close()

	destFile, err := os.Create(dst)
	if err != nil {
		return err
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, sourceFile)
	return err
}
