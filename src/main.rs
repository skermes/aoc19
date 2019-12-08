// The stubs for future days have a lot of these, we don't need the warnings.
#![allow(unused_variables, unused_imports)]

#[macro_use] extern crate itertools;

mod problem;
mod days;
mod intcode;

use std::io::Read;
use std::fs::File;
use structopt::StructOpt;
use std::time::{Instant, Duration};

use crate::problem::Problem;
use crate::days::one::DayOne;
use crate::days::two::DayTwo;
use crate::days::three::DayThree;
use crate::days::four::DayFour;
use crate::days::five::DayFive;
use crate::days::six::DaySix;
use crate::days::seven::DaySeven;
use crate::days::eight::DayEight;
use crate::days::nine::DayNine;
use crate::days::ten::DayTen;
use crate::days::eleven::DayEleven;
use crate::days::twelve::DayTwelve;
use crate::days::thirteen::DayThirteen;
use crate::days::fourteen::DayFourteen;
use crate::days::fifteen::DayFifteen;
use crate::days::sixteen::DaySixteen;
use crate::days::seventeen::DaySeventeen;
use crate::days::eighteen::DayEighteen;
use crate::days::nineteen::DayNineteen;
use crate::days::twenty::DayTwenty;
use crate::days::twentyone::DayTwentyOne;
use crate::days::twentytwo::DayTwentyTwo;
use crate::days::twentythree::DayTwentyThree;
use crate::days::twentyfour::DayTwentyFour;
use crate::days::twentyfive::DayTwentyFive;

#[derive(Debug, StructOpt)]
struct Args {
    day: Option<usize>
}

fn day2problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        2 => Some(Box::new(DayTwo{})),
        3 => Some(Box::new(DayThree{})),
        4 => Some(Box::new(DayFour{})),
        5 => Some(Box::new(DayFive{})),
        6 => Some(Box::new(DaySix{})),
        7 => Some(Box::new(DaySeven{})),
        8 => Some(Box::new(DayEight{})),
        9 => Some(Box::new(DayNine{})),
        10 => Some(Box::new(DayTen{})),
        11 => Some(Box::new(DayEleven{})),
        12 => Some(Box::new(DayTwelve{})),
        13 => Some(Box::new(DayThirteen{})),
        14 => Some(Box::new(DayFourteen{})),
        15 => Some(Box::new(DayFifteen{})),
        16 => Some(Box::new(DaySixteen{})),
        17 => Some(Box::new(DaySeventeen{})),
        18 => Some(Box::new(DayEighteen{})),
        19 => Some(Box::new(DayNineteen{})),
        20 => Some(Box::new(DayTwenty{})),
        21 => Some(Box::new(DayTwentyOne{})),
        22 => Some(Box::new(DayTwentyTwo{})),
        23 => Some(Box::new(DayTwentyThree{})),
        24 => Some(Box::new(DayTwentyFour{})),
        25 => Some(Box::new(DayTwentyFive{})),
        _ => None
    }
}

fn duration_str(duration: Duration) -> String {
    let millis = duration.as_millis();

    if millis == 0 {
        format!("{}\u{03BC}s", duration.as_micros())
    } else {
        format!("{}ms", millis)
    }
}

fn run_day(day: usize) -> std::io::Result<()> {
    let open = File::open(format!("inputs/{}.txt", day));
    let input = match open {
        Ok(mut input_file) => {
            let mut buffer = String::new();
            input_file.read_to_string(&mut buffer)?;
            buffer
        },
        Err(_) => {
            format!("")
        }
    };

    let problem = day2problem(day).unwrap();

    let title = format!(" Day {}: {} ", day, problem.name());
    println!("=========={:=<60}\n", title);

    let start_one = Instant::now();
    let solution_one = problem.part_one(&input);
    let duration_one = duration_str(start_one.elapsed());

    println!("Part One:");
    println!("    Solution: {}", solution_one);
    println!("    Duration: {}\n", duration_one);

    let start_two = Instant::now();
    let solution_two = problem.part_two(&input);
    let duration_two = duration_str(start_two.elapsed());

    println!("Part Two:");
    println!("    Solution: {}", solution_two);
    println!("    Duration: {}\n", duration_two);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    match args.day {
        Some(day) => run_day(day),
        None => {
            for day in 1..26 {
                run_day(day)?;
            }
            Ok(())
        }
    }
}
