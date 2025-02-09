// This example demonstrates the usage of the image/png package from Go's standard library
// for reading, writing, and manipulating PNG images.

package stdlib

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"log"
	"os"
)

func PNG() {
	// Example 1: Creating and saving a PNG image
	fmt.Println("1. Creating a new PNG image")

	// Create a new RGBA image
	width, height := 200, 200
	img := image.NewRGBA(image.Rect(0, 0, width, height))

	// Draw something (a simple gradient)
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			// Create a gradient based on position
			img.Set(x, y, color.RGBA{
				R: uint8(x),
				G: uint8(y),
				B: 100,
				A: 255,
			})
		}
	}

	// Create output file
	outFile, err := os.Create("gradient.png")
	if err != nil {
		log.Fatal("Error creating file:", err)
	}
	defer outFile.Close()

	// Encode and save the image
	err = png.Encode(outFile, img)
	if err != nil {
		log.Fatal("Error encoding PNG:", err)
	}

	// Example 2: Reading a PNG file
	fmt.Println("2. Reading a PNG file")

	// Open the file
	inFile, err := os.Open("gradient.png")
	if err != nil {
		log.Fatal("Error opening file:", err)
	}
	defer inFile.Close()

	// Decode the PNG image
	decodedImg, err := png.Decode(inFile)
	if err != nil {
		log.Fatal("Error decoding PNG:", err)
	}

	// Get image bounds
	bounds := decodedImg.Bounds()
	fmt.Printf("Image dimensions: %dx%d\n", bounds.Max.X, bounds.Max.Y)

	// Example 3: Using PNG Encoder with custom settings
	fmt.Println("\n3. Using PNG Encoder with custom settings")

	customFile, err := os.Create("custom_compression.png")
	if err != nil {
		log.Fatal("Error creating file:", err)
	}
	defer customFile.Close()

	// Create encoder with custom settings
	encoder := &png.Encoder{
		CompressionLevel: png.BestCompression,
	}

	// Encode using custom settings
	err = encoder.Encode(customFile, img)
	if err != nil {
		log.Fatal("Error encoding with custom settings:", err)
	}

	// Example 4: Reading PNG metadata
	fmt.Println("\n4. Reading PNG metadata")

	metadataFile, err := os.Open("gradient.png")
	if err != nil {
		log.Fatal("Error opening file:", err)
	}
	defer metadataFile.Close()

	// Configure decoder to receive metadata
	decoder := png.Decoder{
		ChunkDecoder: png.DefaultChunkDecoder(),
	}

	// Decode image with metadata
	_, err = decoder.Decode(metadataFile)
	if err != nil {
		log.Fatal("Error decoding with metadata:", err)
	}

	// Example 5: Creating a PNG with transparency
	fmt.Println("\n5. Creating PNG with transparency")

	transpImg := image.NewRGBA(image.Rect(0, 0, 100, 100))

	// Draw a partially transparent pattern
	for y := 0; y < 100; y++ {
		for x := 0; x < 100; x++ {
			// Make every other pixel transparent
			alpha := uint8(255)
			if (x+y)%2 == 0 {
				alpha = 0
			}
			transpImg.Set(x, y, color.RGBA{
				R: 255,
				G: 0,
				B: 0,
				A: alpha,
			})
		}
	}

	// Save transparent PNG
	transFile, err := os.Create("transparent.png")
	if err != nil {
		log.Fatal("Error creating transparent file:", err)
	}
	defer transFile.Close()

	err = png.Encode(transFile, transpImg)
	if err != nil {
		log.Fatal("Error encoding transparent PNG:", err)
	}
}

// Helper function to demonstrate reading pixel data
func readPixelAt(img image.Image, x, y int) color.Color {
	return img.At(x, y)
}

// Helper function to demonstrate checking if image is PNG
func isPNG(filename string) (bool, error) {
	file, err := os.Open(filename)
	if err != nil {
		return false, err
	}
	defer file.Close()

	// Read first 8 bytes (PNG signature)
	signature := make([]byte, 8)
	_, err = file.Read(signature)
	if err != nil {
		return false, err
	}

	// Check PNG signature
	return string(signature) == "\x89PNG\r\n\x1a\n", nil
}
