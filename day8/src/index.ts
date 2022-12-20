function readInFileCalledInputAndReturnContentPerLine(): string[] {
    const fs = require('fs');
    const input = fs.readFileSync('input', 'utf8');
    const lines = input.split('\n');
    return lines;
}

function readInFileCalledInputAndParseIntoATwoDimensionalArrayWithEachDigitBeingACell(): string[][] {
    const lines = readInFileCalledInputAndReturnContentPerLine();
    const twoDimensionalArray = lines.map(line=> line.split(''));
    return twoDimensionalArray;
}

function elementCanBeSeenFromTheTop(matrix: string[][], i: number, j: number): boolean {
    let treeSize = matrix[i][j];
    for (let k = j - 1; k >= 0; k--) {
        if (matrix[i][k] >= treeSize) {
            return false;
        }
    }
    return true;
}

function elementCanBeSeenFromTheBottom(matrix: string[][], i: number, j: number): boolean {
    let treeSize = matrix[i][j];
    for (let k = j + 1; k < matrix[i].length; k++) {
        if (matrix[i][k] >= treeSize) {
            return false;
        }
    }
    return true;
}

function elementCanBeSeenFromTheRight(matrix: string[][], i: number, j: number): boolean {
    let treeSize = matrix[i][j];
    for (let k = i + 1; k < matrix.length; k++) {
        if (matrix[k][j] >= treeSize) {
            return false;
        }
    }
    return true;
}

function elementCanBeSeenFromTheLeft(matrix: string[][], i: number, j: number): boolean {
    let treeSize = matrix[i][j];
    for (let k = i - 1; k >= 0; k--) {
        if (matrix[k][j] >= treeSize) {
            return false;
        }
    }
    return true;
}

function elementCanBeSeenFromAnyDirection(matrix: string[][], i: number, j: number): boolean {
    return elementCanBeSeenFromTheTop(matrix, i, j) || elementCanBeSeenFromTheBottom(matrix, i, j) || elementCanBeSeenFromTheLeft(matrix, i, j) || elementCanBeSeenFromTheRight(matrix, i, j);
}

function iterateOverTwoDimensionalStringArrayAndCountVisibleElements(matrix: string[][]): number {
    let count = 0;
    for (let i = 1; i < matrix.length-1; i++) {
        for (let j = 1; j < matrix[i].length-1; j++) {
            count += elementCanBeSeenFromAnyDirection(matrix, i, j) ? 1 : 0;
        }
    }
    return count;
}

function iterateOverTwoDimensionalStringArrayAndComputeMaximumScenicScoreForEveryTree(matrix: string[][]): number {
    let max = 0;
    for (let i = 1; i < matrix.length-1; i++) {
        for (let j = 1; j < matrix[i].length-1; j++) {
            const score = computeScenicScoreForTree(matrix, i, j);
            if (score > max) {
                max = score;
            }
        }
    }
    return max;
}

function distanceToSeeWhenLookingUpwards(matrix: string[][], i: number, j: number): number {
    let treeSize = matrix[i][j];
    let distance = 1;
    for (let k = j - 1; k >= 0; k--) {
        if (matrix[i][k] >= treeSize) {
            return distance;
        }
        distance++;
    }
    return distance-1;
}

function distanceToSeeWhenLookingDownwards(matrix: string[][], i: number, j: number): number {
    let treeSize = matrix[i][j];
    let distance = 1;
    for (let k = j + 1; k < matrix[i].length; k++) {
        if (matrix[i][k] >= treeSize) {
            return distance;
        }
        distance++;
    }
    return distance-1;
}

function distanceToSeeWhenLookingToTheLeft(matrix: string[][], i: number, j: number): number {
    let treeSize = matrix[i][j];
    let distance = 1;
    for (let k = i - 1; k >= 0; k--) {
        if (matrix[k][j] >= treeSize) {
            return distance;
        }
        distance++;
    }
    return distance-1;
}

function distanceToSeeWhenLookingToTheRight(matrix: string[][], i: number, j: number): number {
    let treeSize = matrix[i][j];
    let distance = 1;
    for (let k = i + 1; k < matrix.length; k++) {
        if (matrix[k][j] >= treeSize) {
            return distance;
        }
        distance++;
    }
    return distance-1;
}

function multiplyAllDistancesForTree(matrix: string[][], i: number, j: number): number {
    return distanceToSeeWhenLookingUpwards(matrix, i, j) * distanceToSeeWhenLookingDownwards(matrix, i, j) * distanceToSeeWhenLookingToTheLeft(matrix, i, j) * distanceToSeeWhenLookingToTheRight(matrix, i, j);
}

function computeScenicScoreForTree(matrix: string[][], i: number, j: number): number {
    return multiplyAllDistancesForTree(matrix, i, j);
}

function main() {
    const matrix = readInFileCalledInputAndParseIntoATwoDimensionalArrayWithEachDigitBeingACell();
    const width = matrix[0].length;
    const height = matrix.length;
    const circumference = 2 * (width + height) - 4;
    console.log("Circumference: " + circumference);
    const total = width * height;
    console.log("Total: " + total);
    const innerVisible = iterateOverTwoDimensionalStringArrayAndCountVisibleElements(matrix);
    console.log("Inner visible trees: " + innerVisible);
    const visible = innerVisible + circumference;
    console.log("Totally visible trees: " + visible);

    const maxScenicScore = iterateOverTwoDimensionalStringArrayAndComputeMaximumScenicScoreForEveryTree(matrix);
    console.log("Maximum scenic score: " + maxScenicScore);
}

main();