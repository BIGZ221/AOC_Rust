use fancy_regex::Regex;

use super::Challenge;

#[derive(Default)]
pub struct DayFive;

impl Challenge for DayFive {
    fn run(&mut self, input: String) {
        self.part1(&input);
        self.part2(&input);
    }

    fn part1(&mut self, input: &String) {
        let mut count = 0;
        for line in input.trim().split("\n") {
            let line = line.to_string();
            let vowels = count_vowels(&line);
            if vowels >= 3 && has_double_char(&line) && !contains_illegal_sequence(&line) {
                count += 1;
            }
        }
        println!("Part 1: {}", count);
    }

    fn part2(&mut self, input: &String) {
        let mut count = 0;
        for line in input.trim().split("\n") {
            let line = line.to_string();
            if has_double_pair(&line) && has_seperated_char(&line) {
                count += 1;
            }
        }
        println!("Part 2: {}", count);
    }
}

static VOWELS: &str = "aeiou";

fn count_vowels(input: &String) -> u32 {
    let mut count = 0;
    for c in input.chars() {
        if VOWELS.contains(c) {
            count += 1;
        }
    }
    count
}

fn has_double_char(input: &String) -> bool {
    let re = Regex::new("(.)\\1").unwrap();
    re.is_match(input).unwrap()
}

const ILLEGAL_SEQUENCES: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

fn contains_illegal_sequence(input: &String) -> bool {
    for seq in ILLEGAL_SEQUENCES {
        if input.contains(seq) {
            return true;
        }
    }
    false
}

fn has_double_pair(input: &String) -> bool {
    let re = Regex::new("(..).*\\1").unwrap();
    re.is_match(input).unwrap()
}

fn has_seperated_char(input: &String) -> bool {
    let re = Regex::new("(.).\\1").unwrap();
    re.is_match(input).unwrap()
}
