#[allow(unused_imports)]
use day14_docking_data_common::{Instruction, SAMPLE_DATA, REAL_DATA};
use computer::Computer;

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[Instruction]) -> u64 {
    let mut computer = Computer::new();

    for instruction in data {
        computer.process_instruction(instruction);
    }

    let memory = computer.get_memory();
    let mut sum = 0;

    for (_address, value) in memory {
        sum += value;
    }
    
    sum
}

mod computer {
    use day14_docking_data_common::Instruction;
    use std::collections::HashMap;

    pub struct Computer {
        and_mask: u64,
        or_mask: u64,
        addresses: HashMap::<u64, u64>,
    }

    impl Computer {
        pub fn new() -> Computer {
            Computer {
                and_mask: 0,
                or_mask: 0,
                addresses: HashMap::new(),
            }
        }

        pub fn process_instruction(&mut self, instruction: &Instruction) {
            match instruction {
                Instruction::SetMask { and_mask, or_mask } => {
                    self.and_mask = *and_mask;
                    self.or_mask = *or_mask;
                },
                Instruction::Store { address, value } => {
                    let value = (*value & self.and_mask) | self.or_mask;
                    self.addresses.insert(*address, value);
                }
            }
        }

        pub fn get_memory(&self) -> &HashMap::<u64, u64> {
            &self.addresses
        }
    }
}
