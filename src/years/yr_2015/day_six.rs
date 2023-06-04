use fancy_regex::Regex;

use super::Challenge;
use std::cmp::max;

pub struct DaySix {
    grid_one: Vec<Vec<bool>>,
    grid_two: Vec<Vec<i32>>,
}

impl Challenge for DaySix {
    fn run(&mut self, input: String) {
        self.part1(&input);
        self.part2(&input);
    }

    fn part1(&mut self, input: &String) {
        for line in input.trim().split("\n") {
            let instruction = parse_line(line.to_string());
            self.set_range_one(instruction);
        }
        println!("Part 1: {}", self.count_on());
    }

    fn part2(&mut self, input: &String) {
        for line in input.trim().split("\n") {
            let instruction = parse_line(line.to_string());
            self.set_range_two(instruction);
        }
        println!("Part 2: {}", self.count_brightness());
    }
}

impl Default for DaySix {
    fn default() -> Self {
        Self {
            grid_one: vec![vec![false; 1000]; 1000],
            grid_two: vec![vec![0; 1000]; 1000],
        }
    }
}

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle,
}

impl Operation {
    fn parse(input: &str) -> Operation {
        match input {
            "turn on" => Operation::On,
            "turn off" => Operation::Off,
            "toggle" => Operation::Toggle,
            _ => Operation::Off,
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn parse(input: &str) -> Coord {
        let pair: Vec<usize> = input.split(",").map(|v| v.parse().unwrap()).collect();
        Coord {
            x: pair[0],
            y: pair[1],
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    left: Coord,
    right: Coord,
}

impl DaySix {
    fn set_range_one(&mut self, instruction: Instruction) {
        for j in instruction.left.y..=instruction.right.y {
            for i in instruction.left.x..=instruction.right.x {
                self.grid_one[j][i] = match instruction.operation {
                    Operation::On => true,
                    Operation::Off => false,
                    Operation::Toggle => !self.grid_one[j][i],
                }
            }
        }
    }

    fn count_on(&self) -> usize {
        let mut count = 0;
        for i in 0..self.grid_one.len() {
            for j in 0..self.grid_one[0].len() {
                if self.grid_one[i][j] {
                    count += 1;
                }
            }
        }
        count
    }

    fn set_range_two(&mut self, instruction: Instruction) {
        for j in instruction.left.y..=instruction.right.y {
            for i in instruction.left.x..=instruction.right.x {
                self.grid_two[j][i] += match instruction.operation {
                    Operation::On => 1,
                    Operation::Off => -1,
                    Operation::Toggle => 2,
                };
                self.grid_two[j][i] = max(self.grid_two[j][i], 0);
            }
        }
    }

    fn count_brightness(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.grid_two.len() {
            for j in 0..self.grid_two[0].len() {
                count += self.grid_two[i][j];
            }
        }
        count
    }
}

fn parse_line(line: String) -> Instruction {
    let re =
        Regex::new("(?<op>turn on|toggle|turn off)\\s(?<left>\\d+,\\d+)[^\\d]+(?<right>\\d+,\\d+)")
            .expect("Regex failed to compile");
    let res = re.captures(&line).unwrap().unwrap();
    let operation = res.name("op").unwrap().as_str();
    let left = res.name("left").unwrap().as_str();
    let right = res.name("right").unwrap().as_str();
    Instruction {
        operation: Operation::parse(operation),
        left: Coord::parse(left),
        right: Coord::parse(right),
    }
}
