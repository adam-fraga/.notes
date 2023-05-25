package main

import (
	"fmt"
	"math"
)

// Functions
func add(x int, y int) int {
	return x + y
}

func main() {
	var c, python, java bool
	fmt.Println(c, python, java)

	// Type inference, instance and declare var same as "name string = "Adam" "
	name := "Adam"
	fmt.Printf("Hello %s \n", name)

	fmt.Printf("Now you have %g problems.\n", math.Sqrt(7))

	//PTR are supported and work the same way as C lang
	value := 12
	ptr := &value
	fmt.Println(*ptr)

	//Table
	table_one := [5]int{1, 1, 1, 1, 5}
	table_two := table_one
	table_two[1] = 12

	//%#v flag to show variable representation
	fmt.Printf("Table one %#v \n Table two %#v", table_one, table_two)

	//Slices (same as rust)
	slice_reference_two_first_elem := table_two[1:]
	manual_slice := []int{1, 2, 3}

	//Len and capacity show size of the array and the capacity it can contains
	fmt.Printf("%#v \n %#v", len(slice_reference_two_first_elem), cap(manual_slice))

	//Map (same to hasmap rust key value pair)
	//map[KeyType]ValueType
	var m map[string]int
	m["route"] = 66
	m["age"] = 32
	i := m["route"]
	//If key does not exist we get the value 0
	j := m["root"]
	// j == 0

	// In this statement, the first value (i) is assigned the value stored under the key "route". If that key doesn’t exist, i is the value type’s zero value (0). The second value (ok) is a bool that is true if the key exists in the map, and false if not.
	i, ok := m["route"]
	// To test for a key without retrieving the value, use an underscore in place of the first value:
	_, ok := m["age"]

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

	//Struct (same to object)

	//Define struct
	type Vertex struct {
		X int
		Y int
	}

	//Create struct
	v := Vertex{1, 2}
	v.X = 4
	fmt.Println(v.X)

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
