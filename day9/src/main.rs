use std::io::BufRead;

fn main() {
    let lines = read_in_file_called_input_per_line();
    let mut array = setup_array(LENGTH);
    simulate_rope(lines, &mut array);
    let solution = count_entries_different_from_zero(&array);
    println!("Solution of day 9: {}", solution);
}

fn print_array(array: &Vec<Vec<char>>) {
    array.iter()
        .for_each(|line| { 
            line.iter().for_each(|&e| print!("{}", e));
            println!();
        });
}


// FUCK GLOBALS OMFG
static LENGTH: i32 = 1000;
static mut HEAD_POSITION: (i32, i32) = (LENGTH, LENGTH);
static mut TAIL_POSITION: (i32, i32) = (LENGTH, LENGTH);

fn count_entries_different_from_zero(array: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    array.iter()
        .for_each(|line| line.iter().for_each(|&e| if e != '.' { score += 1; } ));
    score
}

fn simulate_rope(lines: Vec<String>, array: &mut Vec<Vec<char>>) {
    for line in lines {
        let (direction, amount) = parse_step(line);
        simulate_step(direction, amount, array);
    }
}

fn simulate_step(direction: String, amount: i32, array: &mut Vec<Vec<char>>) {
    // move HEAD_POSITION
    // println!("Simulating step into direction {} of amount {}", direction, amount);
    unsafe {
        match direction.as_str() {
            "L" => {
                HEAD_POSITION = (HEAD_POSITION.0 - amount, HEAD_POSITION.1);
                let mut difference = ((HEAD_POSITION.0 - TAIL_POSITION.0).abs(), (HEAD_POSITION.1 - TAIL_POSITION.1));
                if difference.0 > 1 {
                    for _ in 0..difference.0 - 1 {
                        TAIL_POSITION = (TAIL_POSITION.0 - 1, TAIL_POSITION.1 + difference.1);
                        difference.1 = 0;
                        array[TAIL_POSITION.0 as usize][TAIL_POSITION.1 as usize] = '#';
                    }
                }
            },
            "R" => {
                HEAD_POSITION = (HEAD_POSITION.0 + amount, HEAD_POSITION.1);
                let mut difference = ((HEAD_POSITION.0 - TAIL_POSITION.0).abs(), (HEAD_POSITION.1 - TAIL_POSITION.1));
                if difference.0 > 1 {
                    for _ in 0..difference.0 - 1 {
                        TAIL_POSITION = (TAIL_POSITION.0 + 1, TAIL_POSITION.1 + difference.1);
                        difference.1 = 0;
                        array[TAIL_POSITION.0 as usize][TAIL_POSITION.1 as usize] = '#';
                    }
                }
            },
            "D" => {
                HEAD_POSITION = (HEAD_POSITION.0, HEAD_POSITION.1 + amount);
                let mut difference = ((HEAD_POSITION.0 - TAIL_POSITION.0), (HEAD_POSITION.1 - TAIL_POSITION.1).abs());
                if difference.1 > 1 {
                    for _ in 0..difference.1 - 1 {
                        TAIL_POSITION = (TAIL_POSITION.0 + difference.0, TAIL_POSITION.1 + 1);
                        difference.0 = 0;
                        array[TAIL_POSITION.0 as usize][TAIL_POSITION.1 as usize] = '#';
                    }
                }
            },
            "U" => {
                HEAD_POSITION = (HEAD_POSITION.0, HEAD_POSITION.1 - amount);
                let mut difference = ((HEAD_POSITION.0 - TAIL_POSITION.0), (HEAD_POSITION.1 - TAIL_POSITION.1).abs());
                if difference.1 > 1 {
                    for _ in 0..difference.1 - 1 {
                        TAIL_POSITION = (TAIL_POSITION.0 + difference.0, TAIL_POSITION.1 - 1);
                        difference.0 = 0;
                        array[TAIL_POSITION.0 as usize][TAIL_POSITION.1 as usize] = '#';
                    }
                }
            },
            _ => panic!()
        }
    }
}

fn parse_step(step: String) -> (String, i32) {
    let direction = step.split(" ").next().unwrap().to_string();
    let amount = step.split(" ").last().unwrap().parse::<i32>().unwrap();
    (direction, amount)
}

fn setup_array(length: i32) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity((2*length + 1) as usize);
    for _ in 1..(2*length + 2) {
        let mut temp = Vec::with_capacity((2*length + 1) as usize);
        for _ in 1..(2*length + 2) {
            temp.push('.');
        }
        result.push(temp);
    }
    // println!("Length of created array: {}", result.len());
    // println!("Length of created array within the array: {}", result[0].len());
    result[length as usize][length as usize] = '#';
    result
}

fn read_in_file_called_input_per_line() -> Vec<String> {
    let file = std::fs::read("input").unwrap();
    file.lines().into_iter().map(|f| f.unwrap()).collect()
}

fn read_in_file_called_example_per_line() -> Vec<String> {
    let file = std::fs::read("example_still").unwrap();
    file.lines().into_iter().map(|f| f.unwrap()).collect()
}
