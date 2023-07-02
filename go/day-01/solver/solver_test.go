package solver

import (
	"testing"
)

func TestExample(t *testing.T) {
	part1, part2 := Solve("../example.txt")
	if part1 != 24000 {
		t.Errorf("Expected 24000, got %v", part1)
	}

	if part2 != 45000 {
		t.Errorf("Expected 45000, got %v", part2)
	}
}

func TestInput(t *testing.T) {
	part1, part2 := Solve("../input.txt")
	if part1 != 66186 {
		t.Errorf("Expected 66186, got %v", part1)
	}

	if part2 != 196804 {
		t.Errorf("Expected 196804, got %v", part2)
	}
}
