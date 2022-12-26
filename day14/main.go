package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
        "strconv"
)

func main() {
    var fileLines = ReadInFileCalledExample()
    var rocks = ParseLines(fileLines)
    fmt.Println(rocks)
    var _, _ = Normalize(rocks)
    var grid = MakeGrid(rocks)
    fmt.Println()
    //PrintGrid(&grid)
    count := Simulate(&grid, 500)
    fmt.Println()
    fmt.Printf("Solution is %d\n", count)
}

func PrintGrid(grid *[][]byte) {
    fmt.Println()
    for _, line := range *grid {
        fmt.Printf("%c\n", line)
    }
}

func Simulate(grid *[][]byte, start int) int {
    for simulating := true; simulating; simulating = MoveSand(grid, start) != SourceFull {}
    var count = 0
    for i := range *grid {
        for j := range (*grid)[i] {
            if (*grid)[i][j] == 'O' {
                count += 1
            }
        }
    }
    return count
}

type SandMove int
const (
    Moved SandMove = 0
    Settled = 1
    NoSand = 2
    FellThrough = 3
    SourceFull = 4
)

func MoveSand(grid *[][]byte, start int) SandMove {
    for i := range *grid {
        for j := range (*grid)[i] {
            if (*grid)[i][j] == 's' {
                if i == len((*grid))-1 {
                    return FellThrough
                }
                if (*grid)[i+1][j] == '.' {
                    (*grid)[i][j] = '.'
                    (*grid)[i+1][j] = 's'
                    return Moved
                }
                if (*grid)[i+1][j-1] == '.' {
                    (*grid)[i][j] = '.'
                    (*grid)[i+1][j-1] = 's'
                    return Moved
                }
                if j+1 < len((*grid)[i]) && (*grid)[i+1][j+1] == '.' {
                    (*grid)[i][j] = '.'
                    (*grid)[i+1][j+1] = 's'
                    return Moved
                }
                (*grid)[i][j] = 'O'
                //PrintGrid(grid)
                if i == 0 && j == start {
                    return SourceFull
                }
                return Settled
            }
        }
    }
    (*grid)[0][start] = 's'
    //PrintGrid(grid)
    return NoSand
}

func Normalize(rocks []Rock) ([]Rock, int) {
    var maxX = 0
    var minX = 1000
    var maxY = 0
    for _, rock := range rocks {
        for _, coordinate := range rock.coordinates {
            if coordinate.x > maxX {
                maxX = coordinate.x
            }
            if coordinate.x < minX {
                minX = coordinate.x
            }
            if coordinate.y > maxY {
                maxY = coordinate.y
            }
        }
    }

    var decrementByX = minX - 1 - maxY/2
    var normalizedRocks []Rock
    for _, rock := range rocks {
        var normalizedCoordinates []Coordinate
        for _, coordinate := range rock.coordinates {
            normalizedCoordinates = append(normalizedCoordinates, Coordinate{x: coordinate.x - decrementByX, y: coordinate.y})
        }
        normalizedRocks = append(normalizedRocks, Rock{coordinates: normalizedCoordinates})
    }
    return normalizedRocks, 500 - decrementByX
}

func MakeGrid(rocks []Rock) [][]byte {
    var maxX = 0
    var maxY = 0
    for _, rock := range rocks {
        for _, coordinate := range rock.coordinates {
            if coordinate.x > maxX {
                maxX = coordinate.x
            }
            if coordinate.y > maxY {
                maxY = coordinate.y
            }
        }
    }

    var grid = make([][]byte, maxY+3)
    for i := range grid {
        grid[i] = make([]byte, 1001)
    }

    Fill(&grid)
    for _, rock := range rocks {
        for i := 1; i < len(rock.coordinates); i++ {
            Draw(&grid, rock.coordinates[i-1], rock.coordinates[i])
        }
    }

    return grid
}

func Fill(grid *[][]byte) {
    for i := range *grid {
        for j := range (*grid)[i] {
            (*grid)[i][j] = '.'
            if i == len(*grid)-1 {
                (*grid)[i][j] = '#'
            }
        }
    }
}

func Draw(grid *[][]byte, c1 Coordinate, c2 Coordinate) {
    var isHorizontal = c1.y == c2.y
    if isHorizontal {
        var minX = Min(c1.x, c2.x)
        var maxX = Max(c1.x, c2.x)
        for i := minX; i <= maxX; i++ {
            (*grid)[c1.y][i] = '#'
        }
    } else {
        var minY = Min(c1.y, c2.y)
        var maxY = Max(c1.y, c2.y)
        for i := minY; i <= maxY; i++ {
            (*grid)[i][c1.x] = '#'
        }
    }
}

func Min(x int, y int) int {
    if x < y {
        return x
    }
    return y
}

func Max(x int, y int) int {
    if x < y {
        return y
    }
    return x
}

func ParseLines(lines []string) []Rock {
    var rocks []Rock
    for _, line := range lines {
        var parsedCoordinates []Coordinate
        coordinates := strings.Split(line, "->")
        for _, coordinate := range coordinates {
            xAndY := strings.Split(coordinate, ",")
            x, _ := strconv.Atoi(strings.TrimSpace(xAndY[0]))
            y, _ := strconv.Atoi(strings.TrimSpace(xAndY[1]))
            parsedCoordinates = append(parsedCoordinates, Coordinate{x: x, y: y})
        }
        rocks = append(rocks, Rock{coordinates: parsedCoordinates})
    }
    return rocks
}

type Coordinate struct {
    x int
    y int
}

func (c Coordinate) String() string {
    return fmt.Sprintf("(%d, %d)", c.x, c.y)
}

type Rock struct {
    coordinates []Coordinate
}

func (r Rock) String() string {
    return fmt.Sprintf("%v", r.coordinates)
}

func ReadInFileCalledExample() []string {
    var fileLines []string
    readFile, error := os.Open("input")
    defer readFile.Close()
    if error != nil {
        fmt.Println(error)
    }
    fileScanner := bufio.NewScanner(readFile)
    fileScanner.Split(bufio.ScanLines)
    for fileScanner.Scan() {
        fileLines = append(fileLines, fileScanner.Text())
    }
    return fileLines
}
