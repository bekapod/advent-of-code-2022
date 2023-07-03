package solver

import (
	"day-02/parser"
	"os"
)

func Solve(filename string) (int, int) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	part1, part2 := parser.Parse(file)

	part1_score := 0
	part2_score := 0

	for _, choices := range part1 {
		part1_score += choices.Score()
	}

	for _, choices := range part2 {
		part2_score += choices.Score()
	}

	return part1_score, part2_score
}
