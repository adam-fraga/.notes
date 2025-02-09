package tests

import "testing"

func TestAdd_Multiple(t *testing.T) {
	tests := []struct {
		name string
		a, b int
		want int
	}{
		{"simple addition", 2, 2, 4},
		{"negative numbers", -1, -2, -3},
		{"zero value", 0, 0, 0},
	}

	for _, tt := range tests {
		got := Add(tt.a, tt.b)
		if got != tt.want {
			t.Errorf("%s: Add(%d, %d) = %d; want %d",
				tt.name, tt.a, tt.b, got, tt.want)
		}
	}
}
