use super::Challenge;

use md5;

#[derive(Default)]
pub struct DayFour;

impl Challenge for DayFour {
    fn run(&mut self, input: String) {
        self.part1(&input);
        self.part2(&input);
    }

    fn part1(&self, input: &String) {
        let mut count: u32 = 0;
        loop {
            let hash = format!("{:x}", md5::compute(format!("{}{}", input, count)));
            if hash.starts_with("00000") {
                break;
            }
            count += 1;
        }
        println!("Part 1: {}", count);
    }

    fn part2(&self, input: &String) {
        let mut count: u32 = 0;
        loop {
            let hash = format!("{:x}", md5::compute(format!("{}{}", input, count)));
            if hash.starts_with("000000") {
                break;
            }
            count += 1;
        }
        println!("Part 2: {}", count);
    }
}

impl DayFour {}
