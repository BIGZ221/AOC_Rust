mod day_one;
use day_one::DayOne;
mod day_two;

use super::{Challenge, Year};

pub struct Year2015;

impl Year for Year2015 {
    fn execute(day: u32, input: String) {
        match day {
            1 => DayOne::default().run(input),
            _ if day <= 25 => todo!("Day not implemented"),
            _ => panic!("Day out of range"),
        }
    }
}
