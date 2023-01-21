package main

import (
	"fmt"
	"os"
)

func main() {
	rocks := 1000000000000
	maxCaveHeight := 8088
	input := ParseInput()
	jet := NewJet(input)
	cave := NewCave(maxCaveHeight)
	spawner := NewSpawner()
	structure := spawner.Spawn(cave)
	memory := NewMemory()
	keepMembering := true
	heightIncrement := 0

	fmt.Printf("input: %s\n", input)

	for spawner.turn <= rocks {
		direction := jet.NextDirection()
		if structure.CanMove(direction, cave) {
			structure.Move(direction)
			//fmt.Printf("Moved || cave:\n%v\n", cave)
		}
		if structure.CanFall(cave) {
			structure.Fall(cave)
			//fmt.Printf("Fell || cave:\n%v\n", cave)
		} else {
			puttable := structure.(*Structure)
			cave.PutStructure(structure.(*Structure))
			if keepMembering {
				if memory.memberberry(spawner.turn%5, jet.count, puttable.Start().X).count == 2 {
					keepMembering = false
					fmt.Printf("Found a pattern at turn %v\n", spawner.turn)
					member := memory.memberberry(spawner.turn%5, jet.count, puttable.Start().X)
					fmt.Printf("Pattern is %v\n", member)
					repeatIterations := spawner.turn - member.turn
					increment := cave.NextHeight() - 3 - member.height
					fmt.Printf("Can repeat iterations of %v steps and increment height by %v blocks\n", repeatIterations, increment)
					heightIncrement = ((rocks - spawner.turn) / repeatIterations) * increment
					spawner.turn += ((rocks - spawner.turn) / repeatIterations) * repeatIterations
					//rocks += (rocks - spawner.turn) % repeatIterations
				}
				memory.remember(spawner.turn%5, jet.count, puttable.Start().X, cave, spawner)
			}
			//fmt.Printf("Settled || cave:\n%v\n", cave)
			structure = spawner.Spawn(cave)
		}
	}

	h := cave.NextHeight() + heightIncrement
	fmt.Printf("Solution is %v\n", h-3)
	fmt.Printf("Input length was %v\n", len(input))
}

type Combination struct {
	piece        int
	jetPosition  int
	restPosition int
}

type Timeline struct {
	height int
	count  int
	turn   int
}

type Memory struct {
	positions map[Combination]Timeline
}

func NewMemory() *Memory {
	return &Memory{
		positions: make(map[Combination]Timeline),
	}
}

func (m *Memory) remember(piece, position, restPosition int, cave Cave, spawner *Spawner) {
	c := Combination{
		piece:        piece,
		jetPosition:  position,
		restPosition: restPosition,
	}
	value, exists := m.positions[c]
	if exists {
		m.positions[c] = Timeline{
			height: cave.NextHeight() - 3,
			count:  value.count + 1,
			turn:   spawner.turn,
		}
	} else {
		m.positions[c] = Timeline{
			height: cave.NextHeight() - 3,
			count:  1,
			turn:   spawner.turn,
		}
	}
}

func (m *Memory) memberberry(piece, position, restPosition int) Timeline {
	c := Combination{
		piece:        piece,
		jetPosition:  position,
		restPosition: restPosition,
	}
	return m.positions[c]
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

func (j *Jet) NextDirection() byte {
	j.count = (j.count + 1) % len(j.pattern)
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
	//fmt.Printf("Spawning at height %v\n", h+c.shifted)
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
	height  int
	cells   []byte
	shifted int
}

func NewCave(height int) Cave {
	c := Cave{
		height:  height,
		cells:   make([]byte, height*7),
		shifted: 0,
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

func (c *Cave) Shift() {
	for y := 0; y < c.height-1; y++ {
		for x := 0; x < 7; x++ {
			c.Set(x, y, c.Get(x, y+1))
		}
	}
	for x := 0; x < 7; x++ {
		c.Set(x, c.height-1, '.')
	}
	c.shifted++
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
