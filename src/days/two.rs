use std::str::FromStr;
use std::error::Error;
use std::fmt;

use crate::problem::Problem;
use crate::intcode::Machine;

pub struct DayTwo {}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();

        // Part one definition
        machine.set(1, 12).unwrap();
        machine.set(2, 2).unwrap();

        let halted = machine.run_to_halt().unwrap();
        format!("{}", halted.get(0).unwrap())
    }

    fn part_two(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        let target = 19690720;

        // This is really dumb but I gotta go to work.
        for noun in 0..100 {
            for verb in 0..100 {
                machine.set(1, noun).unwrap();
                machine.set(2, verb).unwrap();

                let halted = machine.run_to_halt().unwrap();
                if *halted.get(0).unwrap() == target {
                    return format!("{}", 100 * noun + verb);
                }
            }
        }

        format!("{}", "No solution found under 100")
    }
}
