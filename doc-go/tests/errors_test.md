package tests

import (
	"errors"
	"testing"
)

func Divide(a, b int) (int, error) {
	if b == 0 {
		return 0, errors.New("cannot divide by zero")
	}
	return a / b, nil
}

func TestDivide(t *testing.T) {
	// Test success case
	got, err := Divide(4, 2)
	if err != nil {
		t.Error("got unexpected error")
	}
	if got != 2 {
		t.Errorf("got %d, want 2", got)
	}

	// Test error case
	_, err = Divide(4, 0)
	if err == nil {
		t.Error("expected error but got none")
	}
}
