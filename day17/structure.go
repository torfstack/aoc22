package main

type Position struct {
	X int
	Y int
}

type Structure struct {
	positions []Position
}

type Moveable interface {
	CanFall(Cave) bool
	Fall(Cave)
	CanMove(byte, Cave) bool
	Move(byte)
}

func (s *Structure) Start() Position {
	minX, minY := 8, 100000000000000
	for _, p := range s.positions {
		if p.X < minX {
			minX = p.X
		}
		if p.Y < minY {
			minY = p.Y
		}
	}
	return Position{minX, minY}
}

func (s *Structure) CanFall(c Cave) bool {
	for _, p := range s.positions {
		if p.Y == 0 {
			return false
		}
		if c.Get(p.X, p.Y-1) == '#' {
			return false
		}
	}
	return true
}

func (s *Structure) Fall(c Cave) {
	for i := range s.positions {
		s.positions[i].Y--
	}
}

func (s *Structure) CanMove(d byte, c Cave) bool {
	if d == '<' {
		for _, p := range s.positions {
			if p.X == 0 {
				return false
			}
			if c.Get(p.X-1, p.Y) == '#' {
				return false
			}
		}
		return true
	} else {
		for _, p := range s.positions {
			if p.X == 6 {
				return false
			}
			if c.Get(p.X+1, p.Y) == '#' {
				return false
			}
		}
		return true
	}
}

func (s *Structure) Move(d byte) {
	if d == '<' {
		for i := range s.positions {
			s.positions[i].X--
		}
	} else {
		for i := range s.positions {
			s.positions[i].X++
		}
	}
}

// The following constructors take a single position, the bottom left most if the structure would be boxed in
// This is to make it easier to create the structure in the cave

func NewVertical(x, y int) *Structure {
	return &Structure{
		positions: []Position{
			{x, y},
			{x, y + 1},
			{x, y + 2},
			{x, y + 3},
		},
	}
}

func NewHorizontal(x, y int) *Structure {
	return &Structure{
		positions: []Position{
			{x, y},
			{x + 1, y},
			{x + 2, y},
			{x + 3, y},
		},
	}
}

func NewSquare(x, y int) *Structure {
	return &Structure{
		positions: []Position{
			{x, y},
			{x + 1, y},
			{x, y + 1},
			{x + 1, y + 1},
		},
	}
}

func NewCross(x, y int) *Structure {
	return &Structure{
		positions: []Position{
			{x + 1, y},
			{x, y + 1},
			{x + 1, y + 1},
			{x + 2, y + 1},
			{x + 1, y + 2},
		},
	}
}

func NewL(x, y int) *Structure {
	return &Structure{
		positions: []Position{
			{x, y},
			{x + 1, y},
			{x + 2, y},
			{x + 2, y + 1},
			{x + 2, y + 2},
		},
	}
}
