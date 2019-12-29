use std::io;
use crate::problem::Problem;
use crate::intcode::{Machine, MachineState};

pub struct DayTwentyFive {}

#[allow(dead_code)]
enum SolveMode { Interactive, Automatic, Invisible }

fn solution_commands() -> Vec<String> {
    vec!["south",
         "west",
         "take hologram",
         "south",
         "west",
         "west",
         "take hypercube",
         "east",
         "east",
         "north",
         "east",
         "south",
         "take cake",
         "west",
         "north",
         "take coin",
         "south",
         "east",
         "east",
         "south",
         "east",
         "take food",
         "south",
         "south"]
        .iter()
        .map(|s| format!("{}\n", s).to_string())
        .collect()
}

fn play(mut machine: Machine) -> String {
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

        if input.trim() == "exit" {
            break;
        }

        for c in input.chars() {
            machine.write(c as isize);
        }
    }

    "".to_string()
}

fn from_digits(digits: Vec<usize>) -> usize {
    if digits.len() == 0 {
        0
    } else {
        let top = digits[0] * 10_usize.pow(digits.len() as u32 - 1);
        let bottom = from_digits(digits.iter().skip(1).map(|d| *d).collect());
        top + bottom
    }
}

fn play_auto(mut machine: Machine, show_output: bool) -> String {
    let mut output = Vec::new();

    for command in solution_commands() {
        for c in command.chars() {
            machine.write(c as isize);
        }

        machine.run().unwrap();

        output = machine.read();

        if show_output {
            for c in &output {
                print!("{}", std::char::from_u32(*c as u32).unwrap());
            }
        }
    }

    // The last output only contains one sequence of digits, our answer.
    // ASCII digits are [48, 57].
    let digits = output.iter()
        .filter(|&&c| c >= 48 && c <= 57)
        .map(|&c| (c as usize) - 48)
        .collect();
    from_digits(digits).to_string()
}

const MODE: SolveMode = SolveMode::Invisible;

impl Problem for DayTwentyFive {
    fn name(&self) -> String {
        "Cryostasis".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let machine = Machine::from_str(input).unwrap();

        let solution = match MODE {
            SolveMode::Interactive => play(machine),
            SolveMode::Automatic => play_auto(machine, true),
            SolveMode::Invisible => play_auto(machine, false)
        };

        solution.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
