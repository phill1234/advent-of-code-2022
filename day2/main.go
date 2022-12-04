package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var scorePart1 int = 0
	var scorePart2 int = 0
	// My options X for Rock, Y for Paper, and Z for Scissors
	// Opponent options A for Rock, B for Paper, and Z for Scissors

	// selection options
	selection := make(map[string]string)
	selection["A"] = "X"
	selection["B"] = "Y"
	selection["C"] = "Z"
	selection["X"] = "X"
	selection["Y"] = "Y"
	selection["Z"] = "Z"

	// 1 for Rock, 2 for Paper, and 3 for Scissors
	pointsPerSelection := make(map[string]int)
	pointsPerSelection["X"] = 1
	pointsPerSelection["Y"] = 2
	pointsPerSelection["Z"] = 3
	pointsPerSelection["A"] = 1
	pointsPerSelection["B"] = 2
	pointsPerSelection["C"] = 3

	readFile, err := os.Open("./input.txt")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		// PART 1
		// get first character of each line
		opponentSelection := selection[string(fileScanner.Text()[0])]
		mySelection := selection[string(fileScanner.Text()[2])]
		scorePart1 = scorePart1 + pointsPerSelection[mySelection] + Score(mySelection, opponentSelection)

		// PART 2
		// use A, B, C
		expectedResult := selection[string(fileScanner.Text()[2])]
		opponentSelection = selection[string(fileScanner.Text()[0])]

		mySelection = WhatToSelect(opponentSelection, expectedResult)
		scorePart2 = scorePart2 + pointsPerSelection[mySelection] + Score(mySelection, opponentSelection)

	}

	readFile.Close()
	fmt.Println("Score Part 1: ", scorePart1)
	fmt.Println("Score Part 2: ", scorePart2)
}

func Score(mySelection string, opponentSelection string) int {
	// 0 if you lost, 3 if the round was a draw, and 6 if you won
	if mySelection == opponentSelection {
		return 3
	} else if mySelection == "X" && opponentSelection == "Z" {
		return 6
	} else if mySelection == "Y" && opponentSelection == "X" {
		return 6
	} else if mySelection == "Z" && opponentSelection == "Y" {
		return 6
	}
	// you lost
	return 0
}

func WhatToSelect(opponentSelection string, result string) string {
	// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
	if result == "X" {
		if opponentSelection == "X" {
			return "Z"
		} else if opponentSelection == "Y" {
			return "X"
		} else {
			return "Y"
		}
	} else if result == "Y" {
		if opponentSelection == "X" {
			return "X"
		} else if opponentSelection == "Y" {
			return "Y"
		} else {
			return "Z"
		}
	} else {
		if opponentSelection == "X" {
			return "Y"
		} else if opponentSelection == "Y" {
			return "Z"
		} else {
			return "X"
		}
	}
}
