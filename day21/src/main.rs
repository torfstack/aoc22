use std::{collections::{HashMap, VecDeque}, fs::File, io::Read};

fn main() {
    let monkeys = read_in_file_called_input_as_monkeys();
    let solution = compute_hman(&monkeys);
    println!("Solution: {}", solution);
}

fn compute_hman(monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = &monkeys["root"];
    let magic: i64 = 94625185243550;
    match monkey {
        Monkey::Valued(_) => panic!(),
        Monkey::Computation(a, b, _) => {
            let mut start: i64 = 3715;
            let mut end: i64 = 3716;
            let mut multiplier = 100000000;
            println!("Multiplier: {}", multiplier);
            loop {
                for i in 10*start..=10*end {
                    let mut clone = monkeys.clone();
                    clone.insert(String::from("humn"), Monkey::Valued(multiplier*i));
                    let value_a = compute_name(a, &clone);
                    let value_b = compute_name(b, &clone);
                    println!("{} {} {}", i, value_a, value_b);
                    if value_a == magic {
                        return clone["humn"].value();
                    }
                    if value_a < magic {
                        end = i;
                        start = i-1;
                        multiplier = multiplier / 10;
                        break;
                    }
                }
            }
        }
    }
}

fn compute_name(name: &String, monkeys: &HashMap<String, Monkey>) -> i64 {
    let stack = compute_stack(name, monkeys);
    let mut clone = monkeys.clone();
    stack.iter().rev().for_each(|m| {
        let monkey = clone.get(m).unwrap();
        let value = compute(monkey, &clone);
        clone.insert(m.clone(), Monkey::Valued(value));
    });
    clone[name].value()
}

fn compute_stack(name: &String, monkeys: &HashMap<String, Monkey>) -> Vec<String> {
    let mut stack: Vec<String> = vec![name.clone()];
    let mut queue = VecDeque::new();
    queue.push_front(name);
    while !queue.is_empty() {
        let m = queue.pop_back();
        match &monkeys.get(m.unwrap()) {
            Some(Monkey::Valued(_)) => {},
            Some(Monkey::Computation(a, b, _)) => {
                queue.push_front(a);
                queue.push_front(b);
                stack.push(a.clone());
                stack.push(b.clone());
            },
            None => panic!("Unknown monkey")
        }
    }
    stack
}

fn compute(monkey: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    match monkey {
        Monkey::Valued(v) => *v,
        Monkey::Computation(a, b, op) => {
            let a = monkeys.get(a).unwrap().value();
            let b = monkeys.get(b).unwrap().value();
            match op {
                Operation::Add => a + b,
                Operation::Subtract => a - b,
                Operation::Multiply => a * b,
                Operation::Divide => a / b
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug, Clone)]
enum Monkey {
    Valued(i64),
    Computation(String, String, Operation)
}

impl Monkey {
    fn value(&self) -> i64 {
        if let Monkey::Valued(v) = self {
            *v
        } else {
            panic!("Not a value")
        }
    }
}

fn read_in_file_called_input_as_monkeys() -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();
    let mut content: String = String::new();
    File::open("input").unwrap().read_to_string(&mut content).unwrap();
    for line in content.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let mut name = split[0];
        name = &name[0..name.len()-1];
        if split.len() < 3 {
            let monkey = Monkey::Valued(split[1].parse().unwrap());
            monkeys.insert(name.to_string(), monkey);
        } else {
            let monkey = match split[2] {
                "+" => Monkey::Computation(split[1].to_string(), split[3].to_string(), Operation::Add),
                "-" => Monkey::Computation(split[1].to_string(), split[3].to_string(), Operation::Subtract),
                "*" => Monkey::Computation(split[1].to_string(), split[3].to_string(), Operation::Multiply),
                "/" => Monkey::Computation(split[1].to_string(), split[3].to_string(), Operation::Divide),
                _ => panic!("Unknown operation")
            };
            monkeys.insert(name.to_string(), monkey);
        }
    }
    monkeys
}