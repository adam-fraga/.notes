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

// Multiple sreturn statement
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

func main() {

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

	// Arrays
	table_one := [5]int{1, 1, 1, 1, 5}
	table_two := table_one
	table_two[1] = 12
	// Or with make function
	table_four := make([]int, 5)    // len(a)=5
	table_five := make([]int, 0, 5) // len(b)=0, capacity(b)=5

	// %#v flag to show variable representation
	fmt.Printf("Table one %#v \n Table two %#v", table_one, table_two)
	fmt.Printf("Table four %#v \n Table five %#v", table_four, table_five)

	// Slices (same as rust)
	slice_reference_two_first_elem := table_two[1:]
	manual_slice := []int{1, 2, 3}

	//Append an elem to a slice or array
	new_manual_slice := append(manual_slice, 13)

	//Len and capacity show size of the array and the capacity it can contains
	fmt.Printf("%#v \n %#v\n %#v\n", len(slice_reference_two_first_elem), cap(manual_slice), new_manual_slice)

	//Map (same to hasmap rust key value pair)
	//map[KeyType]ValueType (Valuetype are the same type)
	var m map[string]int
	m["route"] = 66
	m["age"] = 32

	//If key does not exist we get the value 0
	j := m["root"] // j == 0

	// In this statement, the first value (i) is assigned the value stored under the key "route". If that key doesn’t exist, i is the value type’s zero value (0). The second value (ok) is a bool that is true if the key exists in the map, and false if not.
	i, ok := m["route"]

	fmt.Printf("m: %#v\n i: %#v\n j: %#v \n ok: %#v \n", m, i, j, ok)

	// delete element
	delete(m, "route")

	//iterate trough a map
	for key, value := range m {
		fmt.Println("Key:", key, "Value:", value)
	}

	commits := map[string]int{
		"rsc": 3711,
		"r":   2138,
		"gri": 1908,
		"adg": 912,
	}

	fmt.Printf("commits: %#v \n", commits)

	v := Vertex{1, 2}
	v.X = 4
	fmt.Println(v.X)

	v.position()

	//Access struct fields trough pointer
	p := &v
	p.X = 1e9
	fmt.Println(v)

	var (
		v1 = Vertex{1, 2}  // has type Vertex
		v2 = Vertex{X: 1}  // Y:0 is implicit
		v3 = Vertex{}      // X:0 and Y:0
		p2 = &Vertex{1, 2} // has type *Vertex
	)

	fmt.Println(v1, p2, v2, v3)

}
