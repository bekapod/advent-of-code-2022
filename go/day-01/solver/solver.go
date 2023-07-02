package solver

import (
	"day-01/parser"
	"os"
	"sort"
)

func Solve(filename string) (int, int) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	inventories := parser.Parse(file)
	totals := make([]int, 0)

	for _, inventory := range inventories {
		totals = append(totals, sum_calories(inventory))
	}

	sort.Ints(totals)
	last_3 := totals[len(totals)-3:]

	return totals[len(totals)-1], last_3[0] + last_3[1] + last_3[2]
}

func sum_calories(inventory []int) int {
	var sum int
	for _, value := range inventory {
		sum += value
	}
	return sum
}
