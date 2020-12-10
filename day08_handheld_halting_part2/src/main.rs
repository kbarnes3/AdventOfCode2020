use std::collections::HashSet;
#[allow(unused_imports)]
use day08_handheld_halting_common::{Instruction, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    match result {
        Some(value) => println!("{}", value),
        None => println!("No solution found")
    }
}

fn do_work(data: &[Instruction]) -> Option<isize> {
    let mut instructions = data.to_vec();

    for i in 0..instructions.len() {
        match instructions[i] {
            Instruction::Noop(value) => {
                instructions[i] = Instruction::Jump(value);
                let mut computer = Computer::new(&instructions);
                let result = computer.run();
                match result {
                    Some(value) => return Some(value),
                    None => {},
                }
                instructions[i] = Instruction::Noop(value);
            },
            Instruction::Accumulate(_) => {},
            Instruction::Jump(value) => {
                instructions[i] = Instruction::Noop(value);
                let mut computer = Computer::new(&instructions);
                let result = computer.run();
                match result {
                    Some(value) => return Some(value),
                    None => {},
                }
                instructions[i] = Instruction::Jump(value);
            },
        }
    }

    None
}

struct Computer<'a> {
    instructions: &'a [Instruction],
    instruction_pointer: usize,
    accumulator: isize,
    seen_instructions: HashSet::<usize>,
}

impl<'a> Computer<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Computer<'a> {
        Computer {
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
            seen_instructions: HashSet::new()
        }
    }

    pub fn run(&mut self) -> Option<isize> {
        loop {
            if self.instruction_pointer == self.instructions.len() {
                return Some(self.accumulator);
            }
            if self.seen_instructions.contains(&self.instruction_pointer) {
                return None;
            }
            self.seen_instructions.insert(self.instruction_pointer);

            match self.instructions[self.instruction_pointer] {
                Instruction::Noop(_) => self.instruction_pointer += 1,
                Instruction::Accumulate(value) => {
                    self.accumulator += value;
                    self.instruction_pointer += 1
                },
                Instruction::Jump(value) => {
                    self.instruction_pointer = (self.instruction_pointer as isize + value) as usize
                }
            }
        }
    }
}
