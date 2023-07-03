package main

import (
	"day-02/solver"
	"fmt"
	"os"
	"time"
)

func main() {
	filename := os.Args[1]

	start := time.Now()
	part1, part2 := solver.Solve(filename)
	duration := time.Since(start)
	fmt.Printf("(%v, %v) time: %vµs\n", part1, part2, duration.Microseconds())
}
