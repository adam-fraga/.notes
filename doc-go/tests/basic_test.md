/*
Test functions must start with Test
 Test file names must end with _test.go
 Test function names should be descriptive and follow the pattern Test<Function>_<Scenario>
 Run all tests in current directory
 go test

 Run tests and show coverage
 go test -cover

 Run tests in all subdirectories
 go test ./...
*/

package tests

import "testing"

func Add(a, b int) int {
	return a + b
}

func TestAdd(t *testing.T) {
	result := Add(2, 3)
	expected := 5

	if result != expected {
		t.Errorf("Add(2, 3) = %d; expected %d", result, expected)
	}
}

func TestUserService(t *testing.T) {
	t.Run("group: create user", func(t *testing.T) {
		t.Run("with valid data", func(t *testing.T) {
			t.Log("Setting up test case, for user creation")
			// Test implementation
		})

		t.Run("with invalid email", func(t *testing.T) {
			t.Log("Setting up test case, for user creation with invalid email")
			// Test implementation
		})
	})
}
