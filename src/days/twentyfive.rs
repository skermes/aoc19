use std::io;
use crate::problem::Problem;
use crate::intcode::{Machine, MachineState};

pub struct DayTwentyFive {}

impl Problem for DayTwentyFive {
    fn part_one(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();

        loop {
            machine.run().unwrap();

            for c in machine.read() {
                print!("{}", std::char::from_u32(c as u32).unwrap());
            }

            if machine.state() == MachineState::Halted {
                break;
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input == "exit" {
                break;
            }

            for c in input.chars() {
                machine.write(c as isize);
            }
        }

        "".to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
