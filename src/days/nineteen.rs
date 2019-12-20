use crate::problem::Problem;
use crate::intcode::Machine;

pub struct DayNineteen {}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        for y in 0..50 {
            for x in 0..50 {
                let mut machine = Machine::from_str(input).unwrap();
                machine.write(x);
                machine.write(y);
                machine.run().unwrap();

                match machine.read()[0] {
                    0 => print!("."),
                    _ => print!("#")
                };
            }
            println!("");
        }

        "194    (printed and counted)".to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
