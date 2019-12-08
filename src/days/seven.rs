use itertools::Itertools;

use crate::problem::Problem;
use crate::intcode::{Machine, MachineState};

fn run_amplifiers(base_machine: &Machine, phase_settings: Vec<isize>) -> isize {
    let mut signal = 0;
    for setting in phase_settings {
        let mut machine = base_machine.clone();
        machine.write(setting);
        machine.write(signal);
        machine.run().unwrap();
        signal = machine.read()[0];
    }
    signal
}

fn run_amplifiers_looped(base_machine: &Machine,
                         phase_settings: Vec<isize>) -> isize {
    let mut machines: Vec<Machine> = phase_settings
        .iter()
        .map(|setting| {
            let mut m = base_machine.clone();
            m.write(*setting);
            m
        })
        .collect();
    machines[0].write(0);

    let length = machines.len();

    let mut next_inputs = Vec::new();
    let mut i = 0;

    loop {
        let machine = machines.get_mut(i).unwrap();
        for val in next_inputs {
            machine.write(val);
        }

        machine.run().unwrap();
        next_inputs = machine.read();

        if machine.state() == MachineState::Halted && i == length - 1 {
            break;
        }

        i = (i + 1) % length;
    }

    next_inputs[next_inputs.len() - 1]
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
        let machine = Machine::from_str(input).unwrap();
        let phase_settings = (5..10).permutations(5);
        let max_thrust = phase_settings
            .map(|setting| run_amplifiers_looped(&machine, setting))
            .max().unwrap();

        max_thrust.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
