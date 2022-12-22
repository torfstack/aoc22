use std::io::BufRead;

fn main() {
    let lines = read_in_file_called_input_per_line();
    let cycles = simulate_instructions(lines);
    let solution1 = add_up_relevant_cycles(&cycles);
    println!("Solution part 1: {}", solution1);

    println!("Solution part 2:");
    draw_sprite(&cycles);
}

fn draw_sprite(cycles: &Vec<i32>) {
    for i in 0..=5 {
        for j in 0..40 {
            let current_pixel: i32 = j;
            let current_cycle: i32 = j + i*40;
            let middle_pos_of_sprite = cycles[current_cycle as usize];
            let positions_of_sprite = vec![middle_pos_of_sprite - 1, middle_pos_of_sprite, middle_pos_of_sprite + 1];
            if positions_of_sprite.contains(&current_pixel) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn add_up_relevant_cycles(cycles: &Vec<i32>) -> i32 {
    cycles[19]*20 + cycles[59]*60 + cycles[99]*100 + cycles[139]*140 + cycles[179]*180 + cycles[219]*220
}

fn simulate_instructions(lines: Vec<String>) -> Vec<i32> {
    let mut result = Vec::with_capacity(lines.len()*2);
    let mut current_value = 1;
    for line in lines {
        match parse_instruction(line) {
            Instruction::Noop => {
                result.push(current_value)
            },
            Instruction::AddX(v) => {
                result.push(current_value);
                result.push(current_value);
                current_value += v;
            }
        }
    }
    result
}

enum Instruction {
    Noop,
    AddX(i32)
}

fn parse_instruction(line: String) -> Instruction {
    if line == "noop" {
        return Instruction::Noop;
    }
    let amount = line.split(" ").last().unwrap().parse::<i32>().unwrap();
    Instruction::AddX(amount)
}

fn read_in_file_called_example_per_line() -> Vec<String> {
    let reader = std::fs::read("example").unwrap();
    reader.lines().into_iter()
        .map(|line| line.unwrap().trim().to_string())
        .collect()
}

fn read_in_file_called_input_per_line() -> Vec<String> {
    let reader = std::fs::read("input").unwrap();
    reader.lines().into_iter()
        .map(|line| line.unwrap().trim().to_string())
        .collect()
}
