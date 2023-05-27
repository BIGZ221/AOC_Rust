mod yr_2015;
use yr_2015::Year2015;

pub trait Challenge {
    fn run(&mut self, input: String);
    fn part1(&self, input: &String);
    fn part2(&self, input: &String);
}

pub trait Year {
    fn execute(day: u32, input: String);
}

pub fn execute_year(input: String, year: u32, day: u32) {
    match year {
        2015 => Year2015::execute(day, input),
        _ => todo!("Year not implemented"),
    }
}
