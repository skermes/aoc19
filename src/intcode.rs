use thiserror::Error;

type Value = isize;
type Address = usize;

#[derive(Debug, Error)]
pub enum OperationalError {
    #[error("`{0}` is not a known opcode.")]
    InvalidOpcode(Value),
    #[error("Index {0} is outside this machine's memory.")]
    OutOfRange(Address),
    #[error("Index {0} is negative.")]
    NegativeAddress(Value)
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Intcode programs must be integers, not `{0}`.")]
    NotAnInteger(String)
}

pub trait IntoAddress {
    fn into_addr(self) -> Result<Address, OperationalError>;
}

impl IntoAddress for Address {
    fn into_addr(self) -> Result<Address, OperationalError> {
        Ok(self)
    }
}

impl IntoAddress for Value {
    fn into_addr(self) -> Result<Address, OperationalError> {
        if self < 0 {
            Err(OperationalError::NegativeAddress(self))
        } else {
            Ok(self as Address)
        }
    }
}

// Not sure why I need this but the compiler is angry at me when I try to use
// constants...
impl IntoAddress for i32 {
    fn into_addr(self) -> Result<Address, OperationalError> {
        if self < 0 {
            Err(OperationalError::NegativeAddress(self as Value))
        } else {
            Ok(self as Address)
        }
    }
}

#[derive(Debug)]
pub enum Opcodes {
    Add,
    Multiply,
    Halt
}

impl Opcodes {
    fn from_int(i: Value) -> Result<Self, OperationalError> {
        match i {
            1 => Ok(Opcodes::Add),
            2 => Ok(Opcodes::Multiply),
            99 => Ok(Opcodes::Halt),
            _ => Err(OperationalError::InvalidOpcode(i))
        }
    }
}

#[derive(Clone)]
pub struct Machine {
    slots: Vec<Value>,
    pointer: Address,
    is_halted: bool
}

impl Machine {
    pub fn from_str(input: &str) -> Result<Machine, ParseError> {
        let tokens = input.split(",");
        let mut slots = Vec::new();

        for token in tokens {
            slots.push(match Value::from_str_radix(token.trim(), 10) {
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

    pub fn get<I>(&self, index: I) -> Result<&Value, OperationalError>
    where I: IntoAddress {
        let addr = index.into_addr()?;
        self.slots.get(addr)
            .ok_or_else(|| OperationalError::OutOfRange(addr))
    }

    pub fn set<I>(&mut self, index: I, new_value: Value) -> Result<(), OperationalError>
    where I: IntoAddress {
        let addr = index.into_addr()?;
        match self.slots.get_mut(addr) {
            Some(old_value) => {
                *old_value = new_value;
                Ok(())
            },
            None => Err(OperationalError::OutOfRange(addr))
        }
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
