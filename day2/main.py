def readInFileCalledInputPerLine():
    score = 0
    with open('input') as f:
        for line in f:
            score += getScore(line.strip())
    print(score)

def getScore(line):
    if (line == "A X"): return 3
    if (line == "A Y"): return 4
    if (line == "A Z"): return 8
    if (line == "B X"): return 1
    if (line == "B Y"): return 5
    if (line == "B Z"): return 9
    if (line == "C X"): return 2
    if (line == "C Y"): return 6
    if (line == "C Z"): return 7

def main():
    readInFileCalledInputPerLine()

main()