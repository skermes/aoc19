use itertools::Itertools;

use crate::problem::Problem;
use crate::intcode::Machine;

pub struct DayFive {}

impl Problem for DayFive {
    fn name(&self) -> String {
        "Sunny With a Chance of Asteroids".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        machine.write(1);
        machine.run_to_halt().unwrap();

        format!("{}", machine.read().iter().join(" "))
    }

    fn part_two(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        machine.write(5);
        machine.run_to_halt().unwrap();

        format!("{}", machine.read().iter().join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
