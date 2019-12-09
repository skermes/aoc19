use itertools::Itertools;

use crate::problem::Problem;
use crate::intcode::Machine;

pub struct DayNine {}

impl Problem for DayNine {
    fn name(&self) -> String {
        "Sensor Boost".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        machine.write(1);
        machine.run().unwrap();

        machine.read().iter().join(" ").to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        machine.write(2);
        machine.run().unwrap();

        machine.read().iter().join(" ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
