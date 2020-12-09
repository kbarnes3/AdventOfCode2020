use std::collections::HashSet;
#[allow(unused_imports)]
use day08_handheld_halting_common::{Instruction, SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work(data: &'static [Instruction]) -> isize {
    let mut computer = Computer::new(data);
    computer.run()
}

struct Computer {
    instructions: &'static [Instruction],
    instruction_pointer: usize,
    accumulator: isize,
    seen_instructions: HashSet::<usize>,
}

impl Computer {
    pub fn new(instructions: &'static [Instruction]) -> Computer {
        Computer {
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
            seen_instructions: HashSet::new()
        }
    }

    pub fn run(&mut self) -> isize {
        loop {
            if self.seen_instructions.contains(&self.instruction_pointer) {
                return self.accumulator;
            }
            self.seen_instructions.insert(self.instruction_pointer);

            match self.instructions[self.instruction_pointer] {
                Instruction::Noop => self.instruction_pointer += 1,
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
