def readInFileCalledInputPerLine():
    with open("input", "r") as file:
        score = 0
        for line in file:
            splitAtComma = line.split(",")
            firstListOfNumbers = twoNumbersSeperatedByDashToListOfNumbers(splitAtComma[0])
            secondListOfNumbers = twoNumbersSeperatedByDashToListOfNumbers(splitAtComma[1])

            doesFirstListOverlapWithTheSecond = firstListOfNumbers[0] <= secondListOfNumbers[0] and firstListOfNumbers[1] >= secondListOfNumbers[0]
            doesSecondListOverlapWithTheFirst = secondListOfNumbers[0] <= firstListOfNumbers[0] and secondListOfNumbers[1] >= firstListOfNumbers[0]
            if (doesFirstListOverlapWithTheSecond or doesSecondListOverlapWithTheFirst):
                score += 1

        print(score)

def twoNumbersSeperatedByDashToListOfNumbers(string):
    splitAtDash = string.split("-")
    return [int(splitAtDash[0]), int(splitAtDash[1])]

def main():
    readInFileCalledInputPerLine()

main()