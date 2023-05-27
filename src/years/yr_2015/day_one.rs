use crate::years::Challenge;

#[derive(Default)]
pub struct DayOne;

impl Challenge for DayOne {
    fn run(&mut self, input: String) {
        self.part1(&input);
        self.part2(&input);
    }
    fn part1(&self, input: &String) {
        let mut floor: i32 = 0;
        for c in input.chars() {
            match c {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => (),
            }
        }
        println!("Answer: {}", floor);
    }
    fn part2(&self, input: &String) {
        let mut floor: i32 = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => (),
            }
            if floor < 0 {
                println!("Answer: {}", i + 1);
                break;
            }
        }
    }
}
