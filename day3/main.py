def readFileCalledInputPerLine():
    with open("input", "r") as f:
        lines = f.readlines()
        res = 0
        for line in lines:
            stripLine = line.strip()
            charToInt = [charToIntWithLowercaseAEqualToOneAndCapitalAEqualToTwentySeven(char) for char in stripLine]
            firstHalf = charToInt[:len(charToInt) // 2]
            secondHalf = charToInt[len(charToInt) // 2:]
            duplicateValues = [x for x in firstHalf if x in secondHalf][0]
            res += duplicateValues
        print(res)

def readFileCalledInputPerLineAgain():
    with open("input", "r") as f:
        lines = f.readlines()
        linesInPairsOfThree = [lines[i:i+3] for i in range(0, len(lines), 3)]
        res = 0
        for triplet in linesInPairsOfThree:
            charsToIntForTriplet = [[charToIntWithLowercaseAEqualToOneAndCapitalAEqualToTwentySeven(char) for char in line.strip()] for line in triplet]
            duplicateValueInTriplet = [x for x in charsToIntForTriplet[0] if x in charsToIntForTriplet[1] and x in charsToIntForTriplet[2]][0]
            res += duplicateValueInTriplet
        print(res)

def charToIntWithLowercaseAEqualToOneAndCapitalAEqualToTwentySeven(char):
    if char.islower():
        return ord(char) - 96
    else:
        return ord(char) - 64 + 26

def main():
    readFileCalledInputPerLine()
    readFileCalledInputPerLineAgain()

main()