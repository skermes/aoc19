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
    TooManyParameterModes(Value),
    #[error("Cannot use immediate mode parameter as address to set value.")]
    ImmediateModeStorage,
    #[error("Impossible set of mode digits: `{0}`.")]
    InvalidModeDigits(Value)
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
    RelativeBaseOffset,
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
            9 => Ok(Opcode::RelativeBaseOffset),
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
            Opcode::RelativeBaseOffset => 1,
            Opcode::Halt => 0
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    Positional,
    Immediate,
    Relative
}

impl ParameterMode {
    fn from_int(value: Option<&Value>) -> Result<Self, OperationalError> {
        match value {
            Some(val) => {
                match val {
                    0 => Ok(ParameterMode::Positional),
                    1 => Ok(ParameterMode::Immediate),
                    2 => Ok(ParameterMode::Relative),
                    _ => Err(OperationalError::InvalidParameterMode(*val))
                }
            },
            None => {
                Ok(ParameterMode::Positional)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Parameter {
    value: Value,
    mode: ParameterMode
}

// Because we know that no instruction has more than three parameters we can
// inline this for a non-trivial performance boost.
fn mode_int_to_vec(mode: isize) -> Result<Vec<isize>, OperationalError> {
    Ok(match mode {
        0 => vec![],
        1 => vec![1],
        2 => vec![2],
        10 => vec![1, 0],
        11 => vec![1, 1],
        12 => vec![1, 2],
        20 => vec![2, 0],
        21 => vec![2, 1],
        22 => vec![2, 2],
        100 => vec![1, 0, 0],
        101 => vec![1, 0, 1],
        102 => vec![1, 0, 2],
        110 => vec![1, 1, 0],
        111 => vec![1, 1, 1],
        112 => vec![1, 1, 2],
        120 => vec![1, 2, 0],
        121 => vec![1, 2, 1],
        122 => vec![1, 2, 2],
        200 => vec![2, 0, 0],
        201 => vec![2, 0, 1],
        202 => vec![2, 0, 2],
        210 => vec![2, 1, 0],
        211 => vec![2, 1, 1],
        212 => vec![2, 1, 2],
        220 => vec![2, 2, 0],
        221 => vec![2, 2, 1],
        222 => vec![2, 2, 2],
        _ => return Err(OperationalError::InvalidModeDigits(mode))
    })
}

#[derive(Debug, PartialEq, Eq)]
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
        let mode_digits = mode_int_to_vec(*value / 100)?;

        // Reverse here because of the weird way the mode digits are set; see
        // problem description.
        Ok((opcode, mode_digits.into_iter().rev().collect()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MachineState {
    Running,
    Halted, // Halted means the machine has executed a Halt (99) instruction,
    Blocked // while Blocked means it wants to execute an Input but has no input
}

impl std::fmt::Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            MachineState::Running => "Running",
            MachineState::Halted => "Halted",
            MachineState::Blocked => "Blocked"
        })
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    slots: Vec<Value>,
    pointer: Address,
    state: MachineState,
    relative_base: Value,

    input_pointer: Address,
    input: Vec<Value>,

    output_pointer: Address,
    output: Vec<Value>,

    instruction_counter: usize
}

impl Machine {
    fn from_slots(slots: Vec<Value>) -> Machine {
        Machine {
            slots: slots,
            pointer: 0,
            state: MachineState::Running,
            relative_base: 0,

            input_pointer: 0,
            input: Vec::new(),

            output_pointer: 0,
            output: Vec::new(),

            instruction_counter: 0
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

    pub fn run(&mut self) -> Result<(), OperationalError> {
        loop {
            let instruction = self.read_instruction()?;
            self.execute_instruction(&instruction)?;

            match self.state {
                MachineState::Halted => return Ok(()),
                MachineState::Blocked => return Ok(()),
                _ => {}
            }
        }
    }

    fn grow_memory_for(&mut self, index: Address) {
        if index >= self.slots.len() {
            // +1 here so that index is a valid index, not the length.
            self.slots.resize_with(index + 1, Default::default);
        }
    }

    pub fn get<I>(&mut self, index: I) -> Result<&Value, OperationalError>
    where I: IntoAddress {
        let addr = index.into_addr()?;
        self.grow_memory_for(addr);

        // It should be impossible to get this OutOfRange error now because we
        // make every address fit, but we have to unwrap the option somehow and
        // I'd rather have this than a random panic.
        self.slots.get(addr)
            .ok_or_else(|| OperationalError::OutOfRange(addr))
    }

    pub fn set<I>(&mut self, index: I, new_value: Value) -> Result<(), OperationalError>
    where I: IntoAddress {
        let addr = index.into_addr()?;
        self.grow_memory_for(addr);

        match self.slots.get_mut(addr) {
            Some(old_value) => {
                *old_value = new_value;
                Ok(())
            },
            None => Err(OperationalError::OutOfRange(addr))
        }
    }

    fn get_parameter_val(&mut self, parameter: &Parameter) -> Result<Value, OperationalError> {
        match parameter.mode {
            ParameterMode::Positional => Ok(*self.get(parameter.value)?),
            ParameterMode::Immediate => Ok(parameter.value),
            ParameterMode::Relative => Ok(*self.get(self.relative_base + parameter.value)?)
        }
    }

    fn set_at_parameter(&mut self, parameter: &Parameter, value: Value) -> Result<(), OperationalError> {
        match parameter.mode {
            ParameterMode::Positional => self.set(parameter.value, value),
            ParameterMode::Relative => self.set(
                parameter.value + self.relative_base,
                value
            ),
            ParameterMode::Immediate => Err(OperationalError::ImmediateModeStorage)
        }
    }

    fn read_instruction(&mut self) -> Result<Instruction, OperationalError> {
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
        if self.state != MachineState::Running {
            return Ok(());
        }

        let mut advance_pointer = true;
        // Assuming in this block that we have the right number of parameters
        // in our instructions because of how we construct them in
        // read_instruction.
        match instruction.opcode {
            Opcode::Halt => {
                self.state = MachineState::Halted;
            },
            Opcode::Add => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                self.set_at_parameter(&instruction.parameters[2], left + right)?;
            },
            Opcode::Multiply => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                self.set_at_parameter(&instruction.parameters[2], left * right)?;
            },
            Opcode::Input => {
                match self.input.get(self.input_pointer) {
                    None => {
                        self.state = MachineState::Blocked;
                        advance_pointer = false;
                    },
                    Some(_) => {
                        // This is dumb, but it's my best guess of how to get
                        // around the borrow reservation conflict thing.
                        let val = *self.input.get(self.input_pointer).unwrap();
                        self.set_at_parameter(&instruction.parameters[0], val)?;
                        self.input_pointer += 1;
                    }
                }
            },
            Opcode::Output => {
                let val = self.get_parameter_val(&instruction.parameters[0])?;

                self.output.push(val);
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
                self.set_at_parameter(&instruction.parameters[2], value)?;
            },
            Opcode::Equals => {
                let left = self.get_parameter_val(&instruction.parameters[0])?;
                let right = self.get_parameter_val(&instruction.parameters[1])?;

                let value = if left == right { 1 } else { 0 };
                self.set_at_parameter(&instruction.parameters[2], value)?;
            },
            Opcode::RelativeBaseOffset => {
                let value = self.get_parameter_val(&instruction.parameters[0])?;
                self.relative_base += value;
            }
        }

        if advance_pointer {
            // +1 for the instruction itself.
            self.pointer += instruction.opcode.parameter_count() + 1;
        }

        self.instruction_counter += 1;

        Ok(())
    }

    pub fn write(&mut self, input: Value) {
        self.input.push(input);

        if self.state == MachineState::Blocked {
            self.state = MachineState::Running;
        }
    }

    pub fn read(&mut self) -> Vec<Value> {
        if self.output_pointer >= self.output.len() {
            Vec::new()
        } else {
            let outslice = &self.output[self.output_pointer..self.output.len()];
            self.output_pointer = self.output.len();
            outslice.to_vec()
        }
    }

    pub fn peek(&self) -> usize {
        if self.output_pointer >= self.output.len() {
            0
        } else {
            self.output.len() - self.output_pointer
        }
    }

    pub fn state(&self) -> MachineState {
        self.state
    }

    // Allowing dead code because this is used for debugging, not in any
    // actual puzzles.
    #[allow(dead_code)]
    pub fn instruction_counter(&self) -> usize {
        self.instruction_counter
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
        assert_eq!(MachineState::Running, machine.state);
        assert_eq!(1, machine.slots[0]);
        assert_eq!(9, machine.slots[1]);
        assert_eq!(12, machine.slots.len());

        Ok(())
    }

    #[test]
    fn step() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        let instruction = machine.read_instruction()?;
        machine.execute_instruction(&instruction)?;
        assert_eq!(70, machine.slots[3]);
        assert_eq!(MachineState::Running, machine.state);

        let instruction = machine.read_instruction()?;
        machine.execute_instruction(&instruction)?;
        assert_eq!(3500, machine.slots[0]);
        assert_eq!(MachineState::Running, machine.state);

        let instruction = machine.read_instruction()?;
        machine.execute_instruction(&instruction)?;
        assert_eq!(MachineState::Halted, machine.state);

        Ok(())
    }

    #[test]
    fn run_to_halt() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        machine.run()?;
        assert_eq!(3500, machine.slots[0]);
        assert_eq!(MachineState::Halted, machine.state);

        Ok(())
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

        machine.run()?;
        assert_eq!(vec![4, 20], machine.output);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![3, 3, 99, 0]);
        machine.write(20);

        machine.run()?;
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

        let mut low_input = machine.clone();
        low_input.write(4);
        let mut exact_input = machine.clone();
        exact_input.write(8);
        let mut high_input = machine.clone();
        high_input.write(12);

        low_input.run()?;
        assert_eq!(vec![999], low_input.output);

        exact_input.run()?;
        assert_eq!(vec![1000], exact_input.output);

        high_input.run()?;
        assert_eq!(vec![1001], high_input.output);

        Ok(())
    }

    #[test]
    fn blocked() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![
            3, 7, 3, 8, 3, 9, 99, 0, 0, 0
        ]);

        machine.run()?;

        assert_eq!(MachineState::Blocked, machine.state);

        machine.write(10);

        assert_eq!(MachineState::Running, machine.state);

        machine.run()?;

        assert_eq!(MachineState::Blocked, machine.state);

        machine.write(20);
        machine.write(30);

        machine.run()?;

        assert_eq!(MachineState::Halted, machine.state);

        machine.write(40);

        assert_eq!(MachineState::Halted, machine.state);
        assert_eq!(10, machine.slots[7]);
        assert_eq!(20, machine.slots[8]);
        assert_eq!(30, machine.slots[9]);

        Ok(())
    }

    #[test]
    fn relative_mode() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![
            21201, 3, 2, 5, 99, 0, 0, 0, 0, 0, 0, 0, 0
        ]);

        let instruction = machine.read_instruction()?;
        assert_eq!(
            Instruction {
                opcode: Opcode::Add,
                parameters: vec![
                    Parameter {
                        value: 3,
                        mode: ParameterMode::Relative
                    },
                    Parameter {
                        value: 2,
                        mode: ParameterMode::Immediate
                    },
                    Parameter {
                        value: 5,
                        mode: ParameterMode::Relative
                    }
                ]
            },
            instruction
        );

        machine.run()?;

        assert_eq!(7, machine.slots[5]);

        let mut machine = Machine::from_slots(vec![
            21201, 3, 2, 5, 99, 0, 0, 0, 10, 0, 0, 0, 0
        ]);
        machine.relative_base = 5;
        machine.run()?;

        assert_eq!(12, machine.slots[10]);

        Ok(())
    }

    #[test]
    fn relative_base_offset() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![109, 4, 99]);
        assert_eq!(0, machine.relative_base);

        machine.run()?;
        assert_eq!(4, machine.relative_base);

        Ok(())
    }

    #[test]
    fn growable_memory() -> Result<(), OperationalError> {
        let mut machine = Machine::from_slots(vec![
            1, 5, 6, 20, 99, 6, 7
        ]);
        machine.run()?;

        assert_eq!(&13, machine.get(20)?);

        Ok(())
    }

    #[test]
    fn day9_examples() -> Result<(), OperationalError> {
        let mut quine = Machine::from_slots(vec![
            109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99
        ]);
        quine.run()?;

        assert_eq!(
            vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
            quine.read()
        );

        let mut large_output_1 = Machine::from_slots(vec![
            1102,34915192,34915192,7,4,7,99,0
        ]);
        large_output_1.run()?;

        assert_eq!(16, format!("{}", large_output_1.read()[0]).len());

        let mut large_output_2 = Machine::from_slots(vec![
            104,1125899906842624,99
        ]);
        large_output_2.run()?;

        assert_eq!(vec![1125899906842624], large_output_2.read());

        Ok(())
    }
}
