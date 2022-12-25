use std::io::BufRead;

fn main() {
    let (map, starts) = parse_input();
    let solution = starts.iter()
        .map(|&s| bfs(&map, s))
        .min()
        .unwrap();
    println!("Solution is {}", solution);
}

fn bfs(map: &Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut marked = Vec::<(usize, usize)>::with_capacity(map.len()*map.len());
    let mut queue = std::collections::VecDeque::<(usize, usize, usize)>::new();
    queue.push_front((start.0, start.1, 0));
    marked.push((start.0, start.1));

    while !queue.is_empty() {
        let c = queue.pop_back().unwrap();
        let e = map[c.0][c.1];
        if e == 'E' {
            //println!("Solution is {}", c.2);
            return c.2;
        }
        let neighbours = neighbours(&map, &(c.0, c.1));
        //println!("Neighbours of ({}, {}) are {:?}", c.0, c.1, neighbours);
        neighbours.into_iter()
            .for_each(|n| {
                if !marked.contains(&(n.0, n.1)) {
                    let en = map[n.0][n.1];
                    if is_step_possible(e, en) {
                        queue.push_front((n.0, n.1, c.2+1));
                        marked.push((n.0, n.1));
                    }
                }
            });
    };
    9001
}

fn is_step_possible(from: char, to: char) -> bool {
    if from == 'S' {
        return to == 'a' || to == 'b'
    }
    if to == 'E' {
        return from == 'z' || from == 'y'
    }
    let f = from as u32;
    let t = to as u32;
    t <= f + 1
}

fn neighbours(map: &Vec<Vec<char>>, c: &(usize, usize)) -> Vec<(usize, usize)> {
    let result = vec![(c.0-1, c.1), (c.0+1, c.1), (c.0, c.1-1), (c.0, c.1+1)];
    result.into_iter()
        .filter(|c| c.0 < map.len() && c.1 < map[0].len())
        .collect()
}

fn parse_input() -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let reader = std::fs::read("input").unwrap();
    let n = reader.lines().count();
    let mut nodes = Vec::<Vec<char>>::with_capacity(n);
    for _ in 0..n {
        nodes.push(Vec::with_capacity(n))
    }
    let mut i = 0;
    let mut start = vec![];
    for line in reader.lines() {
        let l = line.unwrap();
        l.chars().for_each(|c| {
            if c == 'S' || c == 'a' {
                println!("Found possible start position at ({}, {})", i, nodes[i].len());
                start.push((i, nodes[i].len()));
            }
            nodes[i].push(c);
        });
        i += 1;
    }
    (nodes, start)
}
