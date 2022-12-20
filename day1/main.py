def readInFileCalledInputPerLine():
    with open('input') as f:
        content = f.readlines()
    return [x.strip() for x in content]

def main():
    content = readInFileCalledInputPerLine()
    numbers = []
    sum = 0
    for line in content:
        if line == "":
            numbers.append(sum)
            sum = 0
            continue
        else:
            sum = sum + int(line)
    sortedNumbersDescend = sorted(numbers, reverse=True)
    sumOfFirstThree = sortedNumbersDescend[0] + sortedNumbersDescend[1] + sortedNumbersDescend[2]
    print(sumOfFirstThree)

main()