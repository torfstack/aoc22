use std::io::BufRead;

fn main() {
    let example_lines = read_in_file_called_example_per_line();
    let mut example_array = setup_array(LENGTH);
    let mut example_heads = setup_heads(10);
    simulate_rope(example_lines, &mut example_array, &mut example_heads);
    let example_solution = count_entries_different_from_zero(&example_array);
    //print_array(&example_array);
    println!("Solution of example: {}", example_solution);

    let lines = read_in_file_called_input_per_line();
    let mut array = setup_array(LENGTH);
    let mut heads = setup_heads(10);
    simulate_rope(lines, &mut array, &mut heads);
    let solution = count_entries_different_from_zero(&array);
    println!("Solution of day 9: {}", solution);
}

fn print_array(array: &Vec<Vec<char>>) {
    array.iter()
        .for_each(|line| { 
            line.iter().for_each(|&e| print!("{} ", e));
            println!();
        });
}

type Head = (i32, i32);
// FUCK GLOBALS OMFG
static LENGTH: i32 = 1000;

fn count_entries_different_from_zero(array: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    array.iter()
        .for_each(|line| line.iter().for_each(|&e| if e == '#' { score += 1; } ));
    score
}

fn simulate_rope(lines: Vec<String>, array: &mut Vec<Vec<char>>, heads: &mut Vec<Head>) {
    for line in lines {
        let (direction, amount) = parse_step(line);
        simulate_step(direction, amount, array, heads);
    }
}

fn simulate_step(direction: String, amount: i32, array: &mut Vec<Vec<char>>, heads: &mut Vec<Head>) {
    // println!("Simulating step into direction {} of amount {}", direction, amount);
    for _ in 0..amount {
        // println!("Doing the {}.th step", i+1);
        let mut delta_x: i32 = 0;
        let mut delta_y: i32 = 0;
        match direction.as_str() {
            "L" => {
                delta_x = -1;
            },
            "R" => {
                delta_x = 1;
            },
            "D" => {
                delta_y = 1;
            },
            "U" => {
                delta_y = -1;
            },
            _ => panic!()
        }
        assert!(delta_x.abs() + delta_y.abs() <= 1);

        heads[0] = (heads[0].0 + delta_x, heads[0].1 + delta_y);
        for i in 1..heads.len() {
            let differences = vec![(heads[i-1].0 - heads[i].0), (heads[i-1].1 - heads[i].1)];
            let distance = differences.iter().map(|&d| if d < 0 { -d } else { d }).max().unwrap();
            if distance > 1 {
                assert!(distance <= 2);
                heads[i] = (heads[i].0 + differences[0].signum(), heads[i].1 + differences[1].signum());
            }
        }
        array[heads[heads.len()-1].0 as usize][heads[heads.len()-1].1 as usize] = '#';
    }
    
    //print_array(&array);
    //println!("\n$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$\n");
}

fn parse_step(step: String) -> (String, i32) {
    let direction = step.split(" ").next().unwrap().to_string();
    let amount = step.split(" ").last().unwrap().parse::<i32>().unwrap();
    (direction, amount)
}

fn setup_heads(length: i32) -> Vec<Head> {
    let mut result = Vec::with_capacity(length as usize);
    for _ in 0..length {
        result.push((LENGTH, LENGTH));
    }
    result
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
    let file = std::fs::read("example_long").unwrap();
    file.lines().into_iter().map(|f| f.unwrap()).collect()
}
