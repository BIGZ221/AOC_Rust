mod years;
use years::execute_year;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Cli {
    /// Year of AOC to execute
    year: u32,
    /// Day of AOC to execute
    day: u32,
    /// Test input filename
    input: String,
}

fn main() {
    let args = Cli::parse();
    let input = fs::read_to_string(args.input).expect("File does not exist.");
    execute_year(input, args.year, args.day);
}
