mod problem;
mod days;

use std::io::Read;
use std::fs::File;
use structopt::StructOpt;

use crate::problem::Problem;
use crate::days::one::DayOne;

#[derive(Debug, StructOpt)]
struct Args {
    day: usize
}

fn day2problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        _ => None
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let mut input_file = File::open(format!("inputs/{}.txt", args.day))?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let day = day2problem(args.day).unwrap();
    println!("{}", day.part_one(&input));
    println!("{}", day.part_two(&input));

    Ok(())
}
