use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let instructions = input
        .trim()
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<Instruction>, _>>()
        .unwrap();

    let mut interpreter = Interpreter::new(&instructions);
    let mut instruction_seen = HashSet::new();
    loop {
        if !instruction_seen.insert(interpreter.instruction_pointer) {
            break;
        }

        interpreter.step().unwrap();
    }
    println!("{}", interpreter.accumulator);
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl std::str::FromStr for Instruction {
    type Err = InterpreterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err(InterpreterError::ParseError);
        }
        let inst = &s[0..3];
        let n = s[4..]
            .parse::<i64>()
            .map_err(|_| InterpreterError::ParseError)?;
        match inst {
            "nop" => Ok(Instruction::Nop(n)),
            "jmp" => Ok(Instruction::Jmp(n)),
            "acc" => Ok(Instruction::Acc(n)),
            _ => Err(InterpreterError::ParseError),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum InterpreterError {
    #[error("Parse error")]
    ParseError,

    #[error("Instruction pointer out of range")]
    InstructionPointerOutOfRange,
}

struct Interpreter<'a> {
    instruction_pointer: i64,
    accumulator: i64,
    instructions: &'a [Instruction],
}

impl<'a> Interpreter<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instruction_pointer: 0,
            accumulator: 0,
            instructions,
        }
    }

    fn step(&mut self) -> Result<(), InterpreterError> {
        let instruction = self.fetch_instruction()?;
        self.interpret_instruction(instruction)?;
        Ok(())
    }

    fn fetch_instruction(&self) -> Result<Instruction, InterpreterError> {
        if self.instruction_pointer < 0 {
            return Err(InterpreterError::InstructionPointerOutOfRange);
        }
        let instruction_pointer = self.instruction_pointer as usize;
        if let Some(instruction) = self.instructions.get(instruction_pointer) {
            Ok(*instruction)
        } else {
            Err(InterpreterError::InstructionPointerOutOfRange)
        }
    }

    fn interpret_instruction(&mut self, instruction: Instruction) -> Result<(), InterpreterError> {
        match instruction {
            Instruction::Nop(_) => {
                self.instruction_pointer += 1;
            }
            Instruction::Acc(acc) => {
                self.instruction_pointer += 1;
                self.accumulator += acc;
            }
            Instruction::Jmp(off) => {
                self.instruction_pointer += off;
            }
        }
        Ok(())
    }
}
