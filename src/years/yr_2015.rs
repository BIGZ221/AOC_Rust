mod day_one;
use day_one::DayOne;
mod day_two;
use day_two::DayTwo;
mod day_three;
use day_three::DayThree;
mod day_four;
use day_four::DayFour;
mod day_five;
use day_five::DayFive;
mod day_six;
use day_six::DaySix;

use super::{Challenge, Year};

pub struct Year2015;

impl Year for Year2015 {
    fn execute(day: u32, input: String) {
        match day {
            1 => DayOne::default().run(input),
            2 => DayTwo::default().run(input),
            3 => DayThree::default().run(input),
            4 => DayFour::default().run(input),
            5 => DayFive::default().run(input),
            6 => DaySix::default().run(input),
            _ if day <= 25 => todo!("Day not implemented"),
            _ => panic!("Day out of range"),
        }
    }
}
