use std::{io::BufRead, collections::{HashMap, VecDeque}, ops::Index};
use regex::Regex;

fn main() {
    let input = parse_input();
    let normalized = normalize(input);
    println!("{:?}", normalized);
    let solution = solve(&normalized);
    println!("Solution is {}", solution);
}

struct Traversal {
    current: String,
    time_left: u32,
    current_score: u32,
    activated: Vec<String>,
}

impl Traversal {
    fn possible_steps(&self, graph: &Graph) -> Vec<Traversal> {
        let mut steps = vec![];
        if self.activated.len() == graph.nodes.len() {
            return steps
        }
        for edge in graph.edges_from(&self.current) {
            if edge.weight <= self.time_left {
                steps.push(self.go_to(&edge.to, graph));
            }
        }
        steps
    }

    fn go_to(&self, to: &String, graph: &Graph) -> Traversal {
        let edge_weight = graph.weight(&self.current, to);
        let value = graph.value(to);
        Traversal {
            current: to.clone(),
            time_left: self.time_left-edge_weight-1,
            current_score: self.current_score + (self.time_left-edge_weight-1)*value,
            activated: self.activated.clone()
        }
    }
}

fn solve(graph: &Graph) -> u32 {
    let initial = Traversal {
        current: String::from("AA"),
        time_left: 15, 
        current_score: 0,
        activated: vec![]
    };

    let mut q = VecDeque::<Traversal>::new();
    q.push_front(initial);
    let mut scores: Vec<u32> = vec![];
    while !q.is_empty() {
        let t = q.pop_back().unwrap();
        if t.time_left <= 0 {
            scores.push(t.current_score);
            continue
        }
        for t in t.possible_steps(graph) {
            q.push_front(t)
        }
    }

    scores.into_iter().max().unwrap()
}

fn normalize(graph: Graph) -> Graph {
    let mut nodes = vec![];
    let mut edges = vec![];

    let max = 900;
    let n = graph.nodes.len();
    let mut d = vec![vec![max; n]; n];

    for i in 0..n {
        for j in 0..n {
            let node_i = graph.nodes[i].clone();
            let node_j = graph.nodes[j].clone();
            let does_contain_edge = graph.edges.iter()
                .filter(|e| e.from == node_i.name && e.to == node_j.name)
                .count() > 0;
            if does_contain_edge {
                println!("There is an edge from {} to {}", graph.nodes[i].name, graph.nodes[j].name);
                d[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = std::cmp::min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }

    for line in &d {
        println!("{:?}", line);
    }

    nodes.push(Node { name: "AA".to_string(), pressure: 0 });
    for node in &graph.nodes {
        if node.pressure > 0 && node.name != "AA" {
            let aa_index = graph.node_index(&"AA".to_string());
            let node_index = graph.node_index(&node.name);
            edges.push(Edge {
                from: "AA".to_string(),
                to: node.name.clone(),
                weight: d[aa_index][node_index]
            });
        }
    }

    for node in &graph.nodes {
        if node.pressure > 0 {
            nodes.push(node.clone());
            for other_node in &graph.nodes {
                if other_node.pressure > 0 && other_node.name != node.name {
                    let aa_index = graph.node_index(&node.name);
                    let node_index = graph.node_index(&other_node.name);
                    edges.push(Edge {
                        from: node.name.clone(),
                        to: other_node.name.clone(),
                        weight: d[aa_index][node_index]
                    });
                }
            }
        }
    }

    Graph { nodes: nodes, edges: edges }
}

fn parse_input() -> Graph {
    let reader = std::fs::read("example").unwrap();
    let regex_flow = Regex::new(r"[0-9]+").unwrap();
    let regex_nodes = Regex::new(r"[A-Z]{2}").unwrap();

    let mut n = vec![];
    let mut e = vec![];

    for line in reader.lines() {
        let l = line.unwrap();
        let nodes: Vec<&str> = regex_nodes.find_iter(l.as_str()).map(|m| m.as_str()).collect();
        let flow: Vec<&str> = regex_flow.find_iter(l.as_str()).map(|m| m.as_str()).collect();
        let key = nodes[0].to_string();
        n.push(Node {
            name: key.clone(),
            pressure: flow[0].parse::<u32>().unwrap()
        });
        nodes[1..].into_iter()
            .map(|s| {
                Edge {
                    from: key.clone(),
                    to: s.to_string(),
                    weight: 1
                }
            })
            .for_each(|edge| e.push(edge));
    };
    Graph {
        nodes: n,
        edges: e
    }
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

impl Graph {
    fn weight(&self, from: &String, to: &String) -> u32 {
        self.edges.iter()
            .filter(|e| &e.from == from && &e.to == to)
            .map(|e| e.weight)
            .sum()
    }

    fn value(&self, of: &String) -> u32 {
        self.nodes.iter()
            .filter(|n| &n.name == of)
            .map(|n| n.pressure)
            .sum()
    }

    fn node_index(&self, node: &String) -> usize {
        for i in 0..self.nodes.len() {
            if &self.nodes[i].name == node {
                return i
            }
        }
        0
    }

    fn edges_from(&self, from: &String) -> Vec<Edge> {
        self.edges.clone().into_iter()
            .filter(|e| &e.from == from)
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Node {
    pressure: u32,
    name: String
}

#[derive(Clone, Debug)]
struct Edge {
    from: String,
    to: String,
    weight: u32,
}
