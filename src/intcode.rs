use thiserror::Error;
use std::sync::mpsc::{
    channel,
    Receiver, Sender,
    RecvError, SendError
};

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
    TooManyParameterModes(Value),
    #[error("Error receiving input")]
    InputError(#[from] RecvError),
    #[error("Error writing input")]
    OutputError(#[from] SendError<Value>)
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
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

impl Opcode {
    fn from_int(i: Value) -> Result<Self, OperationalError> {
        match i {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            99 => Ok(Opcode::Halt),
            _ => Err(OperationalError::InvalidOpcode(i))
        }
    }

    fn parameter_count(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Multiply => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::JumpIfTrue => 2,
            Opcode::JumpIfFalse => 2,
            Opcode::LessThan => 3,
            Opcode::Equals => 3,
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

#[derive(Debug)]
pub struct Machine {
    slots: Vec<Value>,
    pointer: Address,
    is_halted: bool,

    input_reader: Receiver<Value>,
    pub input: Sender<Value>,

    output_writer: Sender<Value>,
    pub output: Receiver<Value>
}

impl Machine {
    fn from_slots(slots: Vec<Value>) -> Machine {
        let (in_write, in_read) = channel();
        let (out_write, out_read) = channel();

        Machine {
            slots: slots,
            pointer: 0,
            is_halted: false,

            input_reader: in_read,
            input: in_write,

            output_writer: out_write,
            output: out_read
        }
    }

    pub fn from_str(input: &str) -> Result<Machine, ParseError> {
        let tokens = input.split(",");
        let mut slots = Vec::new();

        for token in tokens {
            slots.push(match Value::from_str_radix(token.trim(), 10) {
                Ok(code) => code,
                Err(_) => return Err(ParseError::NotAnInteger(token.to_string()))
            });
        }

        Ok(Machine::from_slots(slots))
    }

    // Returns a machine with the same memory and instruction pointer state as
    // this machine, but new I/O channels.
    pub fn duplicate(&self) -> Machine {
        let (in_write, in_read) = channel();
        let (out_write, out_read) = channel();

        Machine {
            slots: self.slots.clone(),
            pointer: self.pointer.clone(),
            is_halted: self.is_halted.clone(),

            input_reader: in_read,
            input: in_write,

            output_writer: out_write,
            output: out_read
        }
    }

    pub fn run_to_halt(&mut self) -> Result<(), OperationalError> {
        self.execute_instruction(&self.read_instruction()?)?;

        if self.is_halted {
            Ok(())
        } else {
            self.run_to_halt()
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

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), OperationalError> {
        if self.is_halted {
            return Ok(());
        }

        let mut advance_pointer = true;
        // Assuming in this block that we have the right number of parameters
        // in our instructions because of how we construct them in
        // read_instruction.
        match instruction.opcode {
            Opcode::Halt => {
                self.is_halted = true;
            },
            Opcode::Add => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                self.set(instruction.parameters[2].value, left + right)?;
            },
            Opcode::Multiply => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                self.set(instruction.parameters[2].value, left * right)?;
            },
            Opcode::Input => {
                let val = self.input_reader.recv()
                    .map_err(|e| OperationalError::InputError(e))?;
                self.set(instruction.parameters[0].value, val)?;
            },
            Opcode::Output => {
                let val = self.get_parameter_val(&instruction.parameters[0])?;
                self.output_writer.send(val)
                    .map_err(|e| OperationalError::OutputError(e))?;
            },
            Opcode::JumpIfTrue => {
                let val = self.get_parameter_val(&instruction.parameters[0])?;
                if val != 0 {
                    self.pointer = self.get_parameter_val(&instruction.parameters[1])?.into_addr()?;
                    advance_pointer = false;
                }
            },
            Opcode::JumpIfFalse => {
                let val = self.get_parameter_val(&instruction.parameters[0])?;
                if val == 0 {
                    self.pointer = self.get_parameter_val(&instruction.parameters[1])?.into_addr()?;
                    advance_pointer = false;
                }
            },
            Opcode::LessThan => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                let value = if left < right { 1 } else { 0 };
                self.set(instruction.parameters[2].value, value)?;
            },
            Opcode::Equals => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                let value = if left == right { 1 } else { 0 };
                self.set(instruction.parameters[2].value, value)?;
            }
        }

        if advance_pointer {
            // +1 for the instruction itself.
            self.pointer += instruction.opcode.parameter_count() + 1;
        }

        Ok(())
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
        let mut machine = Machine::from_slots(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        machine.execute_instruction(&machine.read_instruction()?)?;
        assert_eq!(70, machine.slots[3]);
        assert_eq!(false, machine.is_halted);

        machine.execute_instruction(&machine.read_instruction()?)?;
        assert_eq!(3500, machine.slots[0]);
        assert_eq!(false, machine.is_halted);

        machine.execute_instruction(&machine.read_instruction()?)?;
        assert_eq!(true, machine.is_halted);

        Ok(())
    }

    #[test]
    fn run_to_halt() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        machine.run_to_halt()?;
        assert_eq!(3500, machine.slots[0]);
        assert_eq!(true, machine.is_halted);

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
        let mut machine = Machine::from_slots(vec![4, 0, 104, 20, 99]);

        machine.run_to_halt()?;
        assert_eq!(4, machine.output.recv().unwrap());
        assert_eq!(20, machine.output.recv().unwrap());

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![3, 3, 99, 0]);
        machine.input.send(20).unwrap();

        machine.run_to_halt()?;
        assert_eq!(20, machine.slots[3]);

        Ok(())
    }

    #[test]
    fn jump_condition_large_example() -> Result<(), OperationalError> {
        let machine = Machine::from_slots(vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ]);

        let mut low_input = machine.duplicate();
        low_input.input.send(4).unwrap();
        let mut exact_input = machine.duplicate();
        exact_input.input.send(8).unwrap();
        let mut high_input = machine.duplicate();
        high_input.input.send(12).unwrap();

        low_input.run_to_halt()?;
        assert_eq!(999, low_input.output.recv().unwrap());

        exact_input.run_to_halt()?;
        assert_eq!(1000, exact_input.output.recv().unwrap());

        high_input.run_to_halt()?;
        assert_eq!(1001, high_input.output.recv().unwrap());

        Ok(())
    }
}
