use std::{fs::File, io::Read};

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let magic_number = 811589153;
    let initial = read_in_file_called_input_and_return_vector_of_ints().into_iter()
        .map(|x| x * magic_number)
        .enumerate().collect::<Vec<(usize, i64)>>();
    let l = initial.len() as i64;
    let mut numbers = initial.clone();
    for _ in 0..10 {
        for (i, n) in initial.clone() {
            if n == 0 {
                continue;
            }
            let index_of_n = numbers.iter().enumerate().find(|(_, x)| x.1 == n && x.0 == i).unwrap().0 as i64;
            let mut shift_by = n % (l-1);
            if shift_by == 0 {
                continue;
            }
            if index_of_n + shift_by < 1 || index_of_n + shift_by >= l  {
                shift_by += -1*(shift_by/shift_by.abs()) * (l-1);
            }
            if shift_by == 0 {
                continue;
            }
            //println!("Shifting {} by {}", n, shift_by);
            let mut i = index_of_n;
            let d = shift_by/shift_by.abs();
            for _ in 0..shift_by.abs() {
                let j = i + d;
                let temp = numbers[j as usize];
                numbers[j as usize] = numbers[i as usize];
                numbers[i as usize] = temp;
                i = j
            }

            //println!("Numbers: {:?}", numbers);
        }
    }

    let index_of_0 = numbers.iter().enumerate().find(|(_, x)| x.1 == 0).unwrap().0;
    let solution = numbers[(index_of_0 + 1000)%l as usize].1 + numbers[(index_of_0 + 2000)%l as usize].1 + numbers[(index_of_0 + 3000)%l as usize].1;
    println!("Solution part 2: {}", solution);
}

fn part_1() {
    let initial = read_in_file_called_input_and_return_vector_of_ints()
        .into_iter().enumerate().collect::<Vec<(usize, i64)>>();
    let l = initial.len() as i64;
    let mut numbers = initial.clone();
    for (i, n) in initial {
        if n == 0 {
            continue;
        }
        let index_of_n = numbers.iter().enumerate().find(|(_, x)| x.1 == n && x.0 == i).unwrap().0 as i64;
        let mut shift_by = n % (l-1);
        if shift_by == 0 {
            continue;
        }
        if index_of_n + shift_by < 1 || index_of_n + shift_by >= l  {
            shift_by += -1*(shift_by/shift_by.abs()) * (l-1);
        }
        if shift_by == 0 {
            continue;
        }
        //println!("Shifting {} by {}", n, shift_by);
        let mut i = index_of_n;
        let d = shift_by/shift_by.abs();
        for _ in 0..shift_by.abs() {
            let j = i + d;
            let temp = numbers[j as usize];
            numbers[j as usize] = numbers[i as usize];
            numbers[i as usize] = temp;
            i = j
        }

        //println!("Numbers: {:?}", numbers);
    }

    let index_of_0 = numbers.iter().enumerate().find(|(_, x)| x.1 == 0).unwrap().0;
    let solution = numbers[(index_of_0 + 1000)%l as usize].1 + numbers[(index_of_0 + 2000)%l as usize].1 + numbers[(index_of_0 + 3000)%l as usize].1;
    println!("Solution part 1: {}", solution);
}


fn read_in_file_called_input_and_return_vector_of_ints() -> Vec<i64> {
    let mut file = File::open("input").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let mut numbers = Vec::new();
    for line in contents.lines() {
        numbers.push(line.parse::<i64>().unwrap());
    }
    numbers
}