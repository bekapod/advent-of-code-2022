package parser

import (
	"bufio"
	"os"
	"strconv"
)

func Parse(file *os.File) [][]int {
	scanner := bufio.NewScanner(file)
	inventories := make([][]int, 0)

	inventories = append(inventories, make([]int, 0))

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			inventories = append(inventories, make([]int, 0))
			continue
		}

		inventory := inventories[len(inventories)-1]
		inventory = append(inventory, parseLine(line))
		inventories[len(inventories)-1] = inventory
	}

	return inventories
}

func parseLine(line string) int {
	value, err := strconv.ParseInt(line, 10, 64)
	if err != nil {
		panic(err)
	}
	return int(value)
}
