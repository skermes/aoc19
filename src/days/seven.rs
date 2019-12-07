use itertools::Itertools;

use crate::problem::Problem;
use crate::intcode::Machine;

fn run_amplifiers(base_machine: &Machine, phase_settings: Vec<isize>) -> isize {
    let mut signal = 0;
    for setting in phase_settings {
        signal = base_machine
            .write(setting)
            .write(signal)
            .run_to_halt().unwrap()
            .output[0];
    }
    signal
}

pub struct DaySeven {}

impl Problem for DaySeven {
    fn name(&self) -> String {
        "Amplification Circuit".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let machine = Machine::from_str(input).unwrap();
        let phase_settings = (0..5).permutations(5);
        let max_thrust = phase_settings
            .map(|setting| run_amplifiers(&machine, setting))
            .max().unwrap();

        max_thrust.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
