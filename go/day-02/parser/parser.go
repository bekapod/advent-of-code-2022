package parser

import (
	"bufio"
	"os"
	"strings"
)

type Choice int

const (
	Rock     Choice = 1
	Paper    Choice = 2
	Scissors Choice = 3
)

type Result int

const (
	Win  Result = 6
	Loss Result = 0
	Draw Result = 3
)

type Choices struct {
	One Choice
	Two Choice
}

func (c Choices) Score() int {
	score := int(c.Two)

	if c.One == c.Two {
		score += 3
	}

	if c.One == Scissors && c.Two == Rock || c.One == Rock && c.Two == Paper || c.One == Paper && c.Two == Scissors {
		score += 6
	}

	return score
}

type ChoiceWithResult struct {
	One    Choice
	Result Result
}

func (c ChoiceWithResult) Score() int {
	return int(c.get_two()) + int(c.Result)
}

func (c ChoiceWithResult) get_two() Choice {
	if c.One == Rock && c.Result == Win || c.One == Paper && c.Result == Draw || c.One == Scissors && c.Result == Loss {
		return Paper
	}

	if c.One == Rock && c.Result == Loss || c.One == Paper && c.Result == Win || c.One == Scissors && c.Result == Draw {
		return Scissors
	}

	return Rock
}

func Parse(file *os.File) ([]Choices, []ChoiceWithResult) {
	scanner := bufio.NewScanner(file)
	part1 := make([]Choices, 0)
	part2 := make([]ChoiceWithResult, 0)

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), " ")

		part1 = append(part1, Choices{One: parseChoice(line[0]), Two: parseChoice(line[1])})
		part2 = append(part2, ChoiceWithResult{One: parseChoice(line[0]), Result: parseResult(line[1])})
	}

	return part1, part2
}

func parseChoice(choice string) Choice {
	switch choice {
	case "A", "X":
		return Rock
	case "B", "Y":
		return Paper
	case "C", "Z":
		return Scissors
	}
	panic("invalid choice")
}

func parseResult(result string) Result {
	switch result {
	case "X":
		return Loss
	case "Y":
		return Draw
	case "Z":
		return Win
	}
	panic("invalid result")
}
