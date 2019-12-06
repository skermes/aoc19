use thiserror::Error;

type Value = isize;
type Address = usize;

#[derive(Debug, Error)]
pub enum OperationalError {
    #[error("`{0}` is not a known opcode.")]
    InvalidOpcode(Value),
    #[error("`{0}` is not a known parameter mode.")]
    InvalidParameterMode(Value),
    #[error("Index {0} is outside this machine's memory.")]
    OutOfRange(Address),
    #[error("Index {0} is negative.")]
    NegativeAddress(Value),
    #[error("Instruction {0} is negative.")]
    NegativeInstruction(Value),
    #[error("Instruction has too many parameter mode digits for operation: {0}.")]
    TooManyParameterModes(Value)
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

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Halt,
    Output
}

impl Opcode {
    fn from_int(i: Value) -> Result<Self, OperationalError> {
        match i {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            4 => Ok(Opcode::Output),
            99 => Ok(Opcode::Halt),
            _ => Err(OperationalError::InvalidOpcode(i))
        }
    }

    fn parameter_count(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Multiply => 3,
            Opcode::Output => 1,
            Opcode::Halt => 0
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Positional,
    Immediate
}

impl ParameterMode {
    fn from_int(value: Option<&Value>) -> Result<Self, OperationalError> {
        match value {
            Some(val) => {
                match val {
                    0 => Ok(ParameterMode::Positional),
                    1 => Ok(ParameterMode::Immediate),
                    _ => Err(OperationalError::InvalidParameterMode(*val))
                }
            },
            None => {
                Ok(ParameterMode::Positional)
            }
        }
    }
}

#[derive(Debug)]
struct Parameter {
    value: Value,
    mode: ParameterMode
}

fn digits(n: isize) -> Vec<isize> {
    if n < 10 {
        vec![n]
    } else {
        let mut rest = digits(n / 10);
        rest.push(n % 10);
        rest
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    parameters: Vec<Parameter>
}

impl Instruction {
    fn op_and_mode_digits(value: &Value) -> Result<(Opcode, Vec<isize>), OperationalError> {
        if value < &0 {
            return Err(OperationalError::NegativeInstruction(*value));
        }

        let opcode = Opcode::from_int(*value % 100)?;

        let mode_part = *value / 100;
        let mode_digits: Vec<isize>;
        if mode_part == 0 {
            mode_digits = Vec::new();
        } else {
            mode_digits = digits(mode_part);
        }

        // Reverse here because of the weird way the mode digits are set; see
        // problem description.
        Ok((opcode, mode_digits.into_iter().rev().collect()))
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    slots: Vec<Value>,
    pointer: Address,
    is_halted: bool,

    pub output: Vec<Value>
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
            is_halted: false,
            output: Vec::new()
        })
    }

    pub fn run_to_halt(&self) -> Result<Machine, OperationalError> {
        let next = self.execute_instruction(&self.read_instruction()?)?;
        // let next = self.step()?;
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

    fn get_parameter_val(&self, parameter: &Parameter) -> Result<Value, OperationalError> {
        match parameter.mode {
            ParameterMode::Positional => Ok(*self.get(parameter.value)?),
            ParameterMode::Immediate => Ok(parameter.value)
        }
    }

    fn read_instruction(&self) -> Result<Instruction, OperationalError> {
        let instruction_val = self.get(self.pointer)?;
        let (opcode, mode_digits) = Instruction::op_and_mode_digits(instruction_val)?;

        if mode_digits.len() > opcode.parameter_count() {
            return Err(OperationalError::TooManyParameterModes(*instruction_val))
        }

        let mut parameters = Vec::new();
        for i in 0..opcode.parameter_count() {
            let mode = ParameterMode::from_int(mode_digits.get(i))?;
            let param_val = self.get(self.pointer + i + 1)?;
            parameters.push(Parameter { value: *param_val, mode: mode });
        }

        Ok(Instruction{
            opcode: opcode,
            parameters: parameters
        })
    }

    fn execute_instruction(&self, instruction: &Instruction) -> Result<Machine, OperationalError> {
        let mut next = self.clone();
        if self.is_halted {
            return Ok(next);
        }

        // Assuming in this block that we have the right number of parameters
        // in our instructions because of how we construct them in
        // read_instruction.
        match instruction.opcode {
            Opcode::Halt => {
                next.is_halted = true;
            },
            Opcode::Add => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                next.set(instruction.parameters[2].value, left + right)?;
            },
            Opcode::Multiply => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                next.set(instruction.parameters[2].value, left * right)?;
            },
            Opcode::Output => {
                let val = self.get_parameter_val(&instruction.parameters[0])?;

                next.output.push(val);
            }
        }

        // +1 for the instruction itself.
        next.pointer += instruction.opcode.parameter_count() + 1;
        Ok(next)
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
        let machine = Machine {
            slots: vec![1,9,10,3,2,3,11,0,99,30,40,50],
            pointer: 0,
            is_halted: false,
            output: Vec::new()
        };


        let state2 = machine.execute_instruction(&machine.read_instruction()?)?;
        assert_eq!(70, state2.slots[3]);
        assert_eq!(false, state2.is_halted);

        let state3 = state2.execute_instruction(&state2.read_instruction()?)?;
        assert_eq!(3500, state3.slots[0]);
        assert_eq!(false, state3.is_halted);

        let state4 = state3.execute_instruction(&state3.read_instruction()?)?;
        assert_eq!(true, state4.is_halted);

        Ok(())
    }

    #[test]
    fn run_to_halt() -> Result<(), OperationalError> {
        let machine = Machine {
            slots: vec![1,9,10,3,2,3,11,0,99,30,40,50],
            pointer: 0,
            is_halted: false,
            output: Vec::new()
        };

        let halted = machine.run_to_halt()?;
        assert_eq!(3500, halted.slots[0]);
        assert_eq!(true, halted.is_halted);

        Ok(())
    }

    #[test]
    fn test_digits() {
        assert_eq!(vec![1, 0], digits(10));
    }

    #[test]
    fn op_and_modes_add() -> Result<(), OperationalError> {
        let (op, modes) = Instruction::op_and_mode_digits(&1)?;
        assert_eq!(Opcode::Add, op);
        let empty: Vec<isize> = Vec::new();
        assert_eq!(empty, modes);

        let (op2, modes2) = Instruction::op_and_mode_digits(&1001)?;
        assert_eq!(Opcode::Add, op2);
        assert_eq!(vec![0, 1], modes2);

        Ok(())
    }

    #[test]
    fn op_and_modes_halt() -> Result<(), OperationalError> {
        let (op, modes) = Instruction::op_and_mode_digits(&99)?;
        assert_eq!(Opcode::Halt, op);
        let empty: Vec<isize> = Vec::new();
        assert_eq!(empty, modes);

        Ok(())
    }

    #[test]
    fn test_output() -> Result<(), OperationalError> {
        let machine = Machine {
            slots: vec![4, 0, 104, 20, 99],
            pointer: 0,
            is_halted: false,
            output: Vec::new()
        };

        let halted = machine.run_to_halt()?;
        assert_eq!(vec![4, 20], halted.output);

        Ok(())
    }
}
