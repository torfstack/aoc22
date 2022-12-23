use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let monkeys = input_monkeys();
    let mut inspections = simulate_monkeys(Box::new(monkeys));
    println!("{:#?}", inspections);
    inspections.sort();
    inspections.reverse();
    println!("Monkey business: {}", inspections[0]*inspections[1]);
}

fn simulate_monkeys(monkeys: Box<Vec<Monkey>>) -> Vec<u64> {
    let mut inspections = Vec::<u64>::with_capacity(monkeys.len());
    let len = monkeys.len();
    for _ in 0..len {
        inspections.push(0);
    }

    let mut temp = monkeys.clone();

    for _ in 0..10000 {
        for i in 0..len {
            let m = temp[i].clone();
            let (ins, new_monkeys) = m.do_step(i, *temp);
            inspections[i] += ins;
            temp = Box::new(new_monkeys);
        }
    }

    inspections
}

#[derive(Clone)]
struct Monkey {
    items: Box<Vec<u64>>,
    on_true: usize,
    on_false: usize,
    test: Rc<dyn Fn(u64) -> bool>,
    operation: Rc<dyn Fn(u64) -> u64>
}

impl Monkey {
    fn do_step(&self, index: usize, monkeys: Vec<Monkey>) -> (u64, Vec<Monkey>) {
        let mut m = monkeys[index].clone();
        let mut new_monkeys: Vec<Monkey> = vec![];

        for i in 0..index {
            new_monkeys.push(monkeys[i].clone());
        }

        new_monkeys.push(monkeys[0].clone());

        for i in index+1..monkeys.len() {
            new_monkeys.push(monkeys[i].clone());
        }

        let mut inspections = 0;
        for item in m.items.deref() {
            inspections += 1;
            let after_worry = (m.operation.deref())(*item);
            let after_calm = after_worry;
            if (m.test.deref())(after_calm) {
                new_monkeys[m.on_true].items.push(after_calm);
            } else {
                new_monkeys[m.on_false].items.push(after_calm);
            }
        }
        m.items = Box::new(vec![]);

        new_monkeys[index] = m;

        (inspections, new_monkeys)
    }
}

fn example_monkeys() -> Vec<Monkey> {
    const MODULUS: u64 = 23*19*13*17;
    let m0 = Monkey {
        items: Box::new(vec![79, 98]),
        on_true: 2, 
        on_false: 3,
        test: Rc::new(|x| x%23 == 0),
        operation: Rc::new(|x| x.checked_mul(19).unwrap() % MODULUS)
    };
    let m1 = Monkey {
        items: Box::new(vec![54, 65, 75, 74]),
        on_true: 2, 
        on_false: 0,
        test: Rc::new(|x| x%19 == 0),
        operation: Rc::new(|x| x.checked_add(6).unwrap() % MODULUS)
    };
    let m2 = Monkey {
        items: Box::new(vec![79, 60, 97]),
        on_true: 1, 
        on_false: 3,
        test: Rc::new(|x| x%13 == 0),
        operation: Rc::new(|x| x.checked_mul(x).unwrap() % MODULUS)
    };
    let m3 = Monkey {
        items: Box::new(vec![74]),
        on_true: 0, 
        on_false: 1,
        test: Rc::new(|x| x%17 == 0),
        operation: Rc::new(|x| x.checked_add(3).unwrap() % MODULUS)
    };
    vec![m0, m1, m2, m3]
}

fn input_monkeys() -> Vec<Monkey> {
    const MODULUS: u64 = 17*19*7*11*13*3*5*2;
    let m0 = Monkey {
        items: Box::new(vec![83, 97, 95, 67]),
        on_true: 2, 
        on_false: 7,
        test: Rc::new(|x| x%17 == 0),
        operation: Rc::new(|x| x.checked_mul(19).unwrap() % MODULUS)
    };
    let m1 = Monkey {
        items: Box::new(vec![71, 70, 79, 88, 56, 70]),
        on_true: 7, 
        on_false: 0,
        test: Rc::new(|x| x%19 == 0),
        operation: Rc::new(|x| x.checked_add(2).unwrap() % MODULUS)
    };
    let m2 = Monkey {
        items: Box::new(vec![98, 51, 51, 63, 80, 85, 84, 95]),
        on_true: 4, 
        on_false: 3,
        test: Rc::new(|x| x%7 == 0),
        operation: Rc::new(|x| x.checked_add(7).unwrap() % MODULUS)
    };
    let m3 = Monkey {
        items: Box::new(vec![77, 90, 82, 80, 79]),
        on_true: 6, 
        on_false: 4,
        test: Rc::new(|x| x%11 == 0),
        operation: Rc::new(|x| x.checked_add(1).unwrap() % MODULUS)
    };
    let m4 = Monkey {
        items: Box::new(vec![68]),
        on_true: 6, 
        on_false: 5,
        test: Rc::new(|x| x%13 == 0),
        operation: Rc::new(|x| x.checked_mul(5).unwrap() % MODULUS)
    };
    let m5 = Monkey {
        items: Box::new(vec![60, 94]),
        on_true: 1, 
        on_false: 0,
        test: Rc::new(|x| x%3 == 0),
        operation: Rc::new(|x| x.checked_add(5).unwrap() % MODULUS)
    };
    let m6 = Monkey {
        items: Box::new(vec![81, 51, 85]),
        on_true: 5, 
        on_false: 1,
        test: Rc::new(|x| x%5 == 0),
        operation: Rc::new(|x| x.checked_mul(x).unwrap() % MODULUS)
    };
    let m7 = Monkey {
        items: Box::new(vec![98, 81, 63, 65, 84, 71, 84]),
        on_true: 2, 
        on_false: 3,
        test: Rc::new(|x| x%2 == 0),
        operation: Rc::new(|x| x.checked_add(3).unwrap() % MODULUS)
    };
    vec![m0, m1, m2, m3, m4, m5, m6, m7]
}

