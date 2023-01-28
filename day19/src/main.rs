use std::{io::{self, BufRead}, collections::HashMap};
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let blueprints = read_in_blueprints("input");
    let solution = blueprints.iter().map(|blueprint| {
        let score = blueprint.score(24);
        println!("Blueprint: {}, score: {}", blueprint.id, score);
        blueprint.id*score
    }).sum::<u32>();

    println!("Solution: {}\n", solution);
}

fn part2() {
    let blueprints = read_in_blueprints("input2");
    let solution = blueprints.iter().map(|blueprint| {
        let score = blueprint.score(32);
        println!("Blueprint: {}, score: {}", blueprint.id, score);
        score
    }).product::<u32>();

    println!("Solution: {}\n", solution);
}

#[derive(Debug)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
    id: u32
}

struct SetupEvaluator {
    map: HashMap<u32, u32>
}
impl SetupEvaluator {
    fn new() -> SetupEvaluator {
        SetupEvaluator {
            map: HashMap::new()
        }
    }

    fn evaluate(&mut self, setup: &Setup) -> bool {
        let key = setup.steps_to_go;
        if !self.map.contains_key(&key) {
            self.map.insert(key, setup.robots_geode);
            return true;
        }
        if self.map[&key] > setup.robots_geode {
            return false
        }
        self.map.insert(key, setup.robots_geode);
        true
    }
}


impl Blueprint {
    fn score(&self, steps: u32) -> u32 {
        let setup = Setup::new(steps);
        let mut queue = Vec::<Setup>::new();
        let mut results = Vec::<u32>::new();
        let mut evaluator = SetupEvaluator::new();
        queue.push(setup);
        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            let robot = current.choose_robot(self);
            let new_setups = current.evolve(robot, self);
            new_setups.into_iter().for_each(|new_setup| {
                if new_setup.steps_to_go == 0 {
                    results.push(new_setup.geode);
                    return;
                }
                if !evaluator.evaluate(&new_setup) {
                    return;
                }
                queue.push(new_setup)
            });
        };
        results.into_iter().max().unwrap()
    }
}

#[derive(Debug)]
struct Setup {
    robots_ore: u32,
    robots_clay: u32,
    robots_obsidian: u32,
    robots_geode: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    steps_to_go: u32
}

impl Setup {
    fn new(time: u32) -> Setup {
        Setup {
            robots_ore: 1,
            robots_clay: 0,
            robots_obsidian: 0,
            robots_geode: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            steps_to_go: time
        }
    }

    fn evolve(&self, choices: Vec<(u32, u32, u32, u32, u32)>, blueprint: &Blueprint) -> Vec<Setup> {
        choices.into_iter().map(|choice| self.evolve_single(choice, blueprint)).collect()
    }

    fn evolve_single(&self, choice: (u32, u32, u32, u32, u32), blueprint: &Blueprint) -> Setup {
        Setup {  
            robots_ore: self.robots_ore + (choice.3),
            robots_clay: self.robots_clay + (choice.2),
            robots_obsidian: self.robots_obsidian + (choice.1),
            robots_geode: self.robots_geode + (choice.0),
            ore: self.ore + self.robots_ore*choice.4 - choice.3*blueprint.ore - choice.2*blueprint.clay - choice.1*blueprint.obsidian.0 - choice.0*blueprint.geode.0,
            clay: self.clay + self.robots_clay*choice.4 - choice.1*blueprint.obsidian.1, 
            obsidian: self.obsidian + self.robots_obsidian*choice.4 - choice.0*blueprint.geode.1,
            geode: self.geode + self.robots_geode*choice.4,
            steps_to_go: self.steps_to_go - choice.4
        }
    }

    fn choose_robot(&self, blueprint: &Blueprint) -> Vec<(u32, u32, u32, u32, u32)> {
        let mut results = Vec::<(u32, u32, u32, u32, u32)>::new();

        if self.can_produce_obsidian() {
            results.push((1,0,0,0, self.until_next_geode(blueprint)));
        }

        if self.can_produce_clay() {
            results.push((0,1,0,0, self.until_next_obsidian(blueprint)));
        }

        results.push((0,0,1,0, self.until_next_clay(blueprint)));
        results.push((0,0,0,1, self.until_next_ore(blueprint)));

        results = results.into_iter().filter(|result| result.4 < self.steps_to_go).collect();
        if results.is_empty() {
            results.push((0,0,0,0, self.steps_to_go));
        }
        results
    }

    fn until_next_geode(&self, blueprint: &Blueprint) -> u32 {
        let mut t_ore = 0;
        let mut t_obsidian = 0;
        let mut result = 0;

        for t in 1.. {
            if t_ore > 0 && t_obsidian > 0 {
                result = std::cmp::max(t_ore, t_obsidian);
                break;
            }
            if t_ore == 0 && (t-1)*self.robots_ore + self.ore >= blueprint.geode.0 {
                t_ore = t;
            }
            if t_obsidian == 0 && (t-1)*self.robots_obsidian + self.obsidian >= blueprint.geode.1 {
                t_obsidian = t;
            }
        }
        result
    }

    fn until_next_obsidian(&self, blueprint: &Blueprint) -> u32 {
        let mut t_ore = 0;
        let mut t_clay = 0;
        let mut result = 0;

        for t in 1.. {
            if t_ore > 0 && t_clay > 0 {
                result = std::cmp::max(t_ore, t_clay);
                break;
            }
            if t_ore == 0 && (t-1)*self.robots_ore + self.ore >= blueprint.obsidian.0 {
                t_ore = t;
            }
            if t_clay == 0 && (t-1)*self.robots_clay + self.clay >= blueprint.obsidian.1 {
                t_clay = t;
            }
        }
        result
    }

    fn until_next_clay(&self, blueprint: &Blueprint) -> u32 {
        let mut t_ore = 0;
        let mut result = 0;

        for t in 1.. {
            if t_ore > 0 {
                result = t_ore;
                break;
            }
            if (t-1)*self.robots_ore + self.ore >= blueprint.clay {
                t_ore = t;
            }
        }
        result
    }

    fn until_next_ore(&self, blueprint: &Blueprint) -> u32 {
        let mut t_ore = 0;
        let mut result = 0;

        for t in 1.. {
            if t_ore > 0 {
                result = t_ore;
                break;
            }
            if (t-1)*self.robots_ore + self.ore >= blueprint.ore {
                t_ore = t;
            }
        }
        result
    }

    fn can_produce_clay(&self) -> bool {
        self.robots_clay > 0
    }

    fn can_produce_obsidian(&self) -> bool {
        self.robots_obsidian > 0
    }
}

fn read_in_blueprints(name: &str) -> Vec<Blueprint> {
    let regex_num = Regex::new(r"\d+").unwrap();
    let file = std::fs::File::open(name).unwrap();
    io::BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut nums = regex_num.find_iter(&line).map(|num| num.as_str().parse::<u32>().unwrap());
        Blueprint {
            id: nums.next().unwrap(),
            ore: nums.next().unwrap(),
            clay: nums.next().unwrap(),
            obsidian: (nums.next().unwrap(), nums.next().unwrap()),
            geode: (nums.next().unwrap(), nums.next().unwrap()),
        }
    }).collect()
}
