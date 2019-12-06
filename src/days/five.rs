use itertools::Itertools;

use crate::problem::Problem;
use crate::intcode::Machine;

pub struct DayFive {}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let machine = Machine::from_str(input).unwrap();
        let with_input = machine.write(1);
        let halted = with_input.run_to_halt().unwrap();

        format!("{}", halted.output.iter().join(" "))
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
