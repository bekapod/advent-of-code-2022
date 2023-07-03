package solver

import (
	"testing"
)

func TestExample(t *testing.T) {
	part1, part2 := Solve("../example.txt")
	if part1 != 15 {
		t.Errorf("Expected 15, got %v", part1)
	}

	if part2 != 12 {
		t.Errorf("Expected 12, got %v", part2)
	}
}

func TestInput(t *testing.T) {
	part1, part2 := Solve("../input.txt")
	if part1 != 13924 {
		t.Errorf("Expected 13924, got %v", part1)
	}

	if part2 != 13448 {
		t.Errorf("Expected 13448, got %v", part2)
	}
}
