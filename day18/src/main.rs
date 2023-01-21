use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let cubes = parse_file_called_input_and_read_lines_into_cubes();
    let mut count = 0;
    for cube in &cubes {
        cube.adjacent().iter().for_each(|c| {
            if !cubes.contains(c) && c.can_reach_outside(&cubes) {
                count += 1;
            }
        });
    }
    println!("Solution is: {}", count);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn up(&self) -> Cube {
        Cube { x: self.x, y: self.y+1, z: self.z }
    }

    fn down(&self) -> Cube {
        Cube { x: self.x, y: self.y-1, z: self.z }
    }

    fn right(&self) -> Cube {
        Cube { x: self.x+1, y: self.y, z: self.z }
    }

    fn left(&self) -> Cube {
        Cube { x: self.x-1, y: self.y, z: self.z }
    }

    fn front(&self) -> Cube {
        Cube { x: self.x, y: self.y, z: self.z+1 }
    }

    fn back(&self) -> Cube {
        Cube { x: self.x, y: self.y, z: self.z-1 }
    }

    fn adjacent(&self) -> Vec<Cube> {
        vec![self.up(), self.down(), self.right(), self.left(), self.front(), self.back()]
    }

    fn can_reach_outside(&self, cubes: &Vec<Cube>) -> bool {
        if self.is_outside(cubes) {
            return true;
        }
        let mut searched: HashMap<Cube, bool> = HashMap::new();
        let mut queue: Vec<Cube> = Vec::new();
        queue.push(*self);
        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            if current.is_outside(cubes) {
                return true;
            }
            searched.insert(current, true);
            for cube in current.adjacent() {
                if !cubes.contains(&cube) && !searched.contains_key(&cube) {
                    queue.push(cube);
                }
            }
        }
        return false
    }

    fn is_outside(&self, cubes: &Vec<Cube>) -> bool {
        let min_x = cubes.iter().map(|c| c.x).min().unwrap();
        let max_x = cubes.iter().map(|c| c.x).max().unwrap();
        let min_y = cubes.iter().map(|c| c.y).min().unwrap();
        let max_y = cubes.iter().map(|c| c.y).max().unwrap();
        let min_z = cubes.iter().map(|c| c.z).min().unwrap();
        let max_z = cubes.iter().map(|c| c.z).max().unwrap();

        return self.x <= min_x || self.x >= max_x || self.y <= min_y || self.y >= max_y || self.z <= min_z || self.z >= max_z
    }
}

fn parse_file_called_input_and_read_lines_into_cubes() -> Vec<Cube> {
    let mut cubes = Vec::new();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let c = line.split(",").collect::<Vec<&str>>();
        let x = c[0].parse::<i32>().unwrap();
        let y = c[1].parse::<i32>().unwrap();
        let z = c[2].parse::<i32>().unwrap();
        cubes.push(Cube { x, y, z });
    }
    cubes
}