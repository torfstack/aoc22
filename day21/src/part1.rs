use std::{collections::{HashMap, VecDeque}, fs::File, io::Read};

fn main() {
    let monkeys = read_in_file_called_input_as_monkeys();
    let solution = compute_root(&monkeys);
    println!("Solution: {}", solution);
}

fn compute_root(monkeys: &HashMap<String, Monkey>) -> i64 {
    let mut stack = vec!["root"];
    let mut queue = VecDeque::new();
    queue.push_front("root");
    while !queue.is_empty() {
        let m = queue.pop_back();
        match &monkeys.get(m.unwrap()) {
            Some(Monkey::Valued(_)) => {},
            Some(Monkey::Computation(a, b, _)) => {
                queue.push_front(a);
                queue.push_front(b);
                stack.push(a);
                stack.push(b);
            },
            None => panic!("Unknown monkey")
        }
    }
    let mut clone = monkeys.clone();
    stack.iter().rev().for_each(|m| {
        let monkey = &monkeys.get(&m.to_string()).unwrap();
        clone.insert(m.to_string(), Monkey::Valued(compute(monkey, &clone)));
    });

    clone["root"].value()
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