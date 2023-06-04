use std::cmp::{max, min};

use crate::years::Challenge;

#[derive(Default)]
pub struct DayTwo {
    boxes: Vec<Box>,
}

impl Challenge for DayTwo {
    fn run(&mut self, input: String) {
        self.parse(&input);
        self.part1(&input);
        self.part2(&input);
    }

    fn part1(&mut self, _input: &String) {
        println!(
            "Part 1: {}",
            self.boxes
                .iter()
                .fold(0, |acc, val| acc + val.surface_area() + val.min_side_area())
        );
    }

    fn part2(&mut self, _input: &String) {
        println!(
            "Part 2: {}",
            self.boxes
                .iter()
                .fold(0, |acc, val| acc + val.min_perimeter() + val.volume())
        );
    }
}

impl DayTwo {
    fn parse(&mut self, input: &String) {
        for line in input.trim().split("\n") {
            if let [h, w, l] = &line
                .trim()
                .split("x")
                .map(|x| x.parse::<u32>().expect("Invalid sized box"))
                .take(3)
                .collect::<Vec<u32>>()[..]
            {
                self.boxes.push(Box {
                    h: h.to_owned(),
                    w: w.to_owned(),
                    l: l.to_owned(),
                })
            }
        }
    }
}

#[derive(Debug)]
struct Box {
    h: u32,
    w: u32,
    l: u32,
}

impl Box {
    fn surface_area(&self) -> u32 {
        2 * (self.l * self.w + self.w * self.h + self.h * self.l)
    }

    fn min_side_area(&self) -> u32 {
        min(min(self.h * self.l, self.h * self.w), self.w * self.l)
    }

    fn min_perimeter(&self) -> u32 {
        2 * (min(self.h, self.l) + min(max(self.h, self.l), self.w))
    }

    fn volume(&self) -> u32 {
        self.h * self.w * self.l
    }
}
