use std::io::BufRead;
use regex::Regex;

fn main() {
    let lines = read_in_file();
    let sensors = parse_lines(lines);
    let max = 4_000_000;
    for i in 0..=max {
        if i%10_000 == 0 {
            println!("Doing round {}", i);
        }
        
    }
    
    //println!("Grid: {:?}", &grid);

    //let count = count_positions(&grid);
    //println!("Solution is {}", count);
}

fn count_positions(line: &Vec<char>) -> usize {
    line.into_iter().filter(|&&c| c == '#').count()
}

fn make_grid(sensors: &Vec<Sensor>, line: i64) -> (Vec<char>, i64) {
    let min_x = sensors.into_iter().map(|s| s.x-s.r).min().unwrap();
    let max_x = sensors.into_iter().map(|s| s.x+s.r).max().unwrap();
    let min_y = sensors.into_iter().map(|s| s.y-s.r).min().unwrap();
    let max_y = sensors.into_iter().map(|s| s.y+s.r).max().unwrap();

    let buffer_x = vec![0, -min_x].into_iter().max().unwrap();
    let buffer_y = vec![0, -min_y].into_iter().max().unwrap();

    //println!("Buffers x:{}, y:{}", buffer_x, buffer_y);

    let mut grid = Vec::with_capacity((max_x+buffer_x+2) as usize);
    for _ in 0..(max_x+buffer_x+2) {
        grid.push('.');
    }

    for sensor in sensors {
        draw_sensor(&mut grid, sensor, buffer_x, buffer_y, line);
    }

    (grid, buffer_x)
}

fn draw_sensor(grid: &mut Vec<char>, sensor: &Sensor, buffer_x: i64, buffer_y: i64, line: i64) {
    for point in points_with_distance(grid, sensor.x+buffer_x, sensor.y+buffer_y, sensor.r, line+buffer_y) {
        if grid[point as usize] == '.' {
            grid[point as usize] = '#';
        }
        if sensor.y == line as i64 {
            grid[(sensor.x+buffer_x) as usize] = 'S';
        }
        if sensor.by == line as i64 {
            grid[(sensor.bx+buffer_x) as usize] = 'B';
        }
    }
}

fn points_with_distance(grid: &Vec<char>, x: i64, y: i64, d: i64, line: i64) -> Vec<i64> {
    let mut points = vec![];
    for i in 0..grid.len() {
        if distance(x, y, i as i64, line as i64) <= d {
            points.push(i as i64);
        }
    }
    points
}

fn distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1-x2).abs() + (y1-y2).abs()
}

fn parse_lines(lines: Vec<String>) -> Vec<Sensor> {
    lines.into_iter()
        .map(|line| {
            parse_line(line)
        })
        .collect()
}

fn parse_line(line: String) -> Sensor {
    let regex = Regex::new(r"-?\d+").expect("invalid regex");
    let numbers: Vec<i64> = regex.find_iter(line.as_str()).map(|n| n.as_str().parse::<i64>().unwrap()).collect();
    return Sensor{x: numbers[0], y: numbers[1], r: (numbers[0]-numbers[2]).abs() + (numbers[1]-numbers[3]).abs(), bx: numbers[2], by: numbers[3]}
}

struct Sensor {
    x: i64,
    y: i64,
    r: i64,
    bx: i64,
    by: i64 
}

fn read_in_file() -> Vec<String> {
    let reader = std::fs::read("input").unwrap();
    reader.lines().into_iter().map(|line| line.unwrap()).collect()
}
