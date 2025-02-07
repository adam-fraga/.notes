// Current package (same as namespace)
package main

/* Variables declared without an explicit initial value are given their zero value.
   The zero value is:
     - 0 for numeric types,
     - false for the boolean type, and
     - "" (the empty string) for strings.
     - nil for slices, map
*/

// Packages
import (
	"fmt"
	"math"
)

// Functions
func add(x int, y int) int {
	return x + y
}

// Multiple return statement
func swap(x, y, z string) (string, string, string) {
	return y, x, z
}

// Here we named the variable to return so go can infere the return with the corresponding variable (x, y)
func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

// Structure (same to object)
type Vertex struct {
	X int
	Y int
}

// Method
func (v *Vertex) position() {
	fmt.Printf("X: %#v, Y: %#v", v.X, v.Y)
}

func OtherMain() {

	// Multiple variable declaration
	var c, python, java bool = true, false, true
	fmt.Println(c, python, java)

	// Type conversion
	var i int = 42
	var f float64 = float64(i)
	var u uint = uint(f)
	fmt.Println(u)

	// Loop

	sum := 0
	for i := 0; i < 10; i++ {
		sum += i
		fmt.Println(sum)
	}

	for sum < 1000 { // Init and increment are optionnal
		sum += sum
	}

	// for{} => infinite loop

	/* Defer statement report the execution of the defer function untill the surrounding function return
	   Defere function are pushed to the stack and are executed in LIFO order */

	defer fmt.Println("Defer statement") //Surrounded by main so executed at the end of the main func

	// Anonymous function
	func() {
		fmt.Println("OSEF")
	}()

	// Inside a function, the := short assignment statement can be used in place of a var declaration
	// with implicit type.
	name := "Adam"
	fmt.Printf("Hello %s \n", name)
	fmt.Printf("Now you have %g problems.\n", math.Sqrt(7))

	// Pointers
	value := 12
	ptr := &value
	fmt.Println(*ptr)
}
