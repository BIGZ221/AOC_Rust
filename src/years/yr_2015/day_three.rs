use std::collections::HashSet;

use super::Challenge;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pair {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct DayThree;

impl Challenge for DayThree {
    fn run(&mut self, input: String) {
        self.part1(&input);
        self.part2(&input)
    }

    fn part1(&self, input: &String) {
        println!("Part 1: {}", get_visits(input).len());
    }

    fn part2(&self, input: &String) {
        let santa: String = input.chars().step_by(2).collect();
        let robot: String = input.chars().skip(1).step_by(2).collect();
        let visited_by_santa = get_visits(&santa);
        let visited_by_robot = get_visits(&robot);
        println!(
            "Part 2: {}",
            visited_by_santa.union(&visited_by_robot).count()
        );
    }
}

fn get_visits(input: &String) -> HashSet<Pair> {
    let mut visited: HashSet<Pair> = HashSet::new();
    let mut pos = Pair { x: 0, y: 0 };
    visited.insert(pos);
    for c in input.chars() {
        match c {
            '^' => pos.y += 1,
            '>' => pos.x += 1,
            'v' => pos.y -= 1,
            '<' => pos.x -= 1,
            _ => (),
        }
        visited.insert(pos.clone());
    }
    visited
}
