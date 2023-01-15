package main

import (
	"fmt"
	"os"
)

func main() {
	input := ParseInput()
	jet := NewJet(input)
	cave := NewCave(2022 * 4)
	spawner := NewSpawner()
	structure := spawner.Spawn(cave)

	fmt.Printf("input: %s\n", input)

	for spawner.turn <= 2022 {
		direction := jet.nextDirection()
		if structure.CanMove(direction, cave) {
			structure.Move(direction)
			//fmt.Printf("Moved || cave:\n%v\n", cave)
		}
		if structure.CanFall(cave) {
			structure.Fall(cave)
			//fmt.Printf("Fell || cave:\n%v\n", cave)
		} else {
			cave.PutStructure(structure.(*Structure))
			//fmt.Printf("Settled || cave:\n%v\n", cave)
			structure = spawner.Spawn(cave)
		}
	}

	h := cave.NextHeight()
	fmt.Printf("Solution is %v\n", h-3)
	fmt.Printf("Input length was %v\n", len(input))
}

type Jet struct {
	pattern []byte
	count   int
}

func NewJet(pattern []byte) Jet {
	return Jet{
		pattern: pattern,
		count:   -1,
	}
}

func (j *Jet) nextDirection() byte {
	j.count++
	return j.pattern[j.count%len(j.pattern)]
}

type Spawner struct {
	turn int
}

func NewSpawner() *Spawner {
	return &Spawner{
		turn: 0,
	}
}

func (s *Spawner) Spawn(c Cave) Moveable {
	s.turn++
	h := c.NextHeight()
	fmt.Printf("Spawning at height %v\n", h)
	if s.turn%5 == 1 {
		return NewHorizontal(2, h)
	} else if s.turn%5 == 2 {
		return NewCross(2, h)
	} else if s.turn%5 == 3 {
		return NewL(2, h)
	} else if s.turn%5 == 4 {
		return NewVertical(2, h)
	} else {
		return NewSquare(2, h)
	}
}

type Cave struct {
	height int
	cells  []byte
}

func NewCave(height int) Cave {
	c := Cave{
		height: height,
		cells:  make([]byte, height*7),
	}
	for y := 0; y < c.height; y++ {
		for x := 0; x < 7; x++ {
			c.Set(x, y, '.')
		}
	}
	return c
}

func (c *Cave) PutStructure(structure *Structure) {
	for _, p := range structure.positions {
		c.Set(p.X, p.Y, '#')
	}
}

func (c *Cave) Get(x, y int) byte {
	return c.cells[y*7+x]
}

func (c *Cave) Set(x, y int, value byte) {
	c.cells[y*7+x] = value
}

func (c *Cave) NextHeight() int {
	for y := 0; y < c.height; y++ {
		var isEmpty = true
		for x := 0; x < 7; x++ {
			if c.Get(x, y) == '#' {
				isEmpty = false
			}
		}
		if isEmpty {
			return y + 3
		}
	}
	panic("no height found")
}

func (c Cave) String() string {
	h := c.NextHeight()
	res := ""
	for y := h - 1; y >= 0; y-- {
		for x := 0; x < 7; x++ {
			res += string(c.Get(x, y))
		}
		res += "\n"
	}
	return res
}

func ParseInput() []byte {
	// read in file called input.txt
	// parse the file into a slice of Direction
	// return the slice
	dat, err := os.ReadFile("input")
	if err != nil {
		fmt.Println("Error reading file {}, err")
		panic(err)
	}
	return dat
}
