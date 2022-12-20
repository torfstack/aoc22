def initialState():
    firstStack = ['F', 'D', 'B', 'Z', 'T', 'J', 'R', 'N']
    secondStack = ['R', 'S', 'N', 'J', 'H']
    thirdStack = ['C', 'R', 'N', 'J', 'G', 'Z', 'F', 'Q']
    fourthStack = ['F', 'V', 'N', 'G', 'R', 'T', 'Q']
    fifthStack = ['L', 'T', 'Q', 'F']
    sixthStack = ['Q', 'C', 'W', 'Z', 'B', 'R', 'G', 'N']
    seventhStack = ['F', 'C', 'L', 'S', 'N', 'H', 'N']
    eigthStack = ['D', 'N', 'Q', 'M', 'T', 'J']
    ninthStack = ['P', 'G', 'S']
    return [firstStack, secondStack, thirdStack, fourthStack, fifthStack, sixthStack, seventhStack, eigthStack, ninthStack]

def readInFileCalledInputPerLine():
    state = initialState()
    with open('input') as f:
        content = f.readlines()
        for line in content:
            threeNumbersInLine = line.split()
            filterNumbers = list(filter(lambda x: x.isdigit(), threeNumbersInLine))
            amount = int(filterNumbers[0])
            fromIndex = int(filterNumbers[1]) - 1
            toIndex = int(filterNumbers[2]) - 1
            lastAmountElementsOfFromIndex = state[fromIndex][-amount:]
            state[fromIndex] = state[fromIndex][:-amount]
            state[toIndex] = state[toIndex] + lastAmountElementsOfFromIndex
    lastLetterOfEachStack = list(map(lambda x: x[-1], state))
    print("".join(lastLetterOfEachStack))

def main():
    readInFileCalledInputPerLine()

main()