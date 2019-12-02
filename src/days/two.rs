use std::str::FromStr;
use std::error::Error;
use std::fmt;

use crate::problem::Problem;

enum Opcodes {
    Add,
    Multiply,
    Halt
}

#[derive(Debug)]
enum OperationalError {
    InvalidOpcode(usize),
    OutOfRange(usize)
}

impl fmt::Display for OperationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationalError::InvalidOpcode(code) => {
                write!(f, "{} is not a known opcode.", code)
            },
            OperationalError::OutOfRange(index) => {
                write!(f, "Index {} is outside this machine's memory.", index)
            }
        }
    }
}

impl Error for OperationalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
enum ParseError {
    NotAnInteger(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NotAnInteger(code) => {
                write!(f, "Opcodes must be integer, not: {}.", code)
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Opcodes {
    fn from_int(i: usize) -> Result<Self, OperationalError> {
        match i {
            1 => Ok(Opcodes::Add),
            2 => Ok(Opcodes::Multiply),
            99 => Ok(Opcodes::Halt),
            _ => Err(OperationalError::InvalidOpcode(i))
        }
    }
}

#[derive(Clone)]
struct Machine {
    slots: Vec<usize>,
    pointer: usize,
    is_halted: bool
}

impl Machine {
    pub fn from_str(input: &str) -> Result<Machine, ParseError> {
        let tokens = input.split(",");
        let mut slots = Vec::new();

        for token in tokens {
            slots.push(match usize::from_str_radix(token.trim(), 10) {
                Ok(code) => code,
                Err(_) => return Err(ParseError::NotAnInteger(token.to_string()))
            });
        }

        Ok(Machine{
            slots: slots,
            pointer: 0,
            is_halted: false
        })
    }

    pub fn step(&self) -> Result<Machine, OperationalError> {
        let mut next = self.clone();

        if self.is_halted {
            Ok(next)
        } else {
            let opcode = Opcodes::from_int(self.slots[self.pointer])?;

            match opcode {
                Opcodes::Halt => {
                    next.is_halted = true;
                },
                Opcodes::Add => {
                    let left = self.get(*self.get(self.pointer + 1)?)?;
                    let right = self.get(*self.get(self.pointer + 2)?)?;
                    let store = self.get(self.pointer + 3)?;

                    next.set(*store, left + right)?;
                    next.pointer = self.pointer + 4;
                },
                Opcodes::Multiply => {
                    let left = self.get(*self.get(self.pointer + 1)?)?;
                    let right = self.get(*self.get(self.pointer + 2)?)?;
                    let store = self.get(self.pointer + 3)?;

                    next.set(*store, left * right)?;
                    next.pointer = self.pointer + 4;
                }
            };

            Ok(next)
        }
    }

    pub fn run_to_halt(&self) -> Result<Machine, OperationalError> {
        let next = self.step()?;
        if next.is_halted {
            Ok(next)
        } else {
            next.run_to_halt()
        }
    }

    fn get(&self, index: usize) -> Result<&usize, OperationalError> {
        self.slots.get(index)
            .ok_or_else(|| OperationalError::OutOfRange(index))
    }

    fn set(&mut self, index: usize, new_value: usize) -> Result<(), OperationalError> {
        match self.slots.get_mut(index) {
            Some(old_value) => {
                *old_value = new_value;
                Ok(())
            },
            None => Err(OperationalError::OutOfRange(index))
        }
    }
}

pub struct DayTwo {}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();

        // Part one definition
        machine.slots[1] = 12;
        machine.slots[2] = 2;

        let halted = machine.run_to_halt().unwrap();
        format!("{}", halted.slots[0])
    }

    fn part_two(&self, input: &str) -> String {
        let mut machine = Machine::from_str(input).unwrap();
        let target = 19690720;

        // This is really dumb but I gotta go to work.
        for noun in 0..100 {
            for verb in 0..100 {
                machine.slots[1] = noun;
                machine.slots[2] = verb;

                let halted = machine.run_to_halt().unwrap();
                if halted.slots[0] == target {
                    return format!("{}", 100 * noun + verb);
                }
            }
        }

        format!("{}", "No solution found under 100")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_machine() -> Result<(), ParseError> {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let machine = Machine::from_str(input)?;

        assert_eq!(0, machine.pointer);
        assert_eq!(false, machine.is_halted);
        assert_eq!(1, machine.slots[0]);
        assert_eq!(9, machine.slots[1]);
        assert_eq!(12, machine.slots.len());

        Ok(())
    }

    #[test]
    fn step() -> Result<(), OperationalError> {
        let machine = Machine{
            slots: vec![1,9,10,3,2,3,11,0,99,30,40,50],
            pointer: 0,
            is_halted: false
        };

        let state2 = machine.step()?;
        assert_eq!(70, state2.slots[3]);
        assert_eq!(false, state2.is_halted);

        let state3 = state2.step()?;
        assert_eq!(3500, state3.slots[0]);
        assert_eq!(false, state3.is_halted);

        let state4 = state3.step()?;
        assert_eq!(true, state4.is_halted);

        Ok(())
    }

    #[test]
    fn run_to_halt() -> Result<(), OperationalError> {
        let machine = Machine{
            slots: vec![1,9,10,3,2,3,11,0,99,30,40,50],
            pointer: 0,
            is_halted: false
        };

        let halted = machine.run_to_halt()?;
        assert_eq!(3500, halted.slots[0]);
        assert_eq!(true, halted.is_halted);

        Ok(())
    }
}
