use crate::problem::Problem;
use crate::intcode::Machine;

pub struct DaySeventeen {}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        machine.run().unwrap();
        for value in machine.read() {
            print!("{}", std::char::from_u32(value as u32).unwrap());
        }
        "5740   (solved manually by just printing and looking)".to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
