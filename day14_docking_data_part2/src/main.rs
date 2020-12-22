#[allow(unused_imports)]
use day14_docking_data_common::{Instruction, SAMPLE_DATA, SAMPLE_DATA_2, REAL_DATA};
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
        or_mask: u64,
        floating_mask: String,
        addresses: HashMap::<u64, u64>,
    }

    impl Computer {
        pub fn new() -> Computer {
            Computer {
                or_mask: 0,
                floating_mask: String::new(),
                addresses: HashMap::new(),
            }
        }

        pub fn process_instruction(&mut self, instruction: &Instruction) {
            match instruction {
                Instruction::SetMask { orig_mask, .. } => self.process_mask(orig_mask),
                Instruction::Store { address, value } => self.store(*address, *value),
            }
        }

        pub fn get_memory(&self) -> &HashMap::<u64, u64> {
            &self.addresses
        }

        fn process_mask(&mut self, mask: &str) {
            let mask = String::from(mask);
            let or_mask = mask.replace("X", "0");
            self.or_mask = u64::from_str_radix(&or_mask, 2).unwrap();

            self.floating_mask = mask;

        }

        fn store(&mut self, address: u64, value: u64) {
            let ored_address = address | self.or_mask;
            let floating_mask = String::from(&self.floating_mask);
            self.store_at_permutations(ored_address, value, &floating_mask);
        }

        fn store_at_permutations(&mut self, base_address: u64, value: u64, floating_mask: &String) {
            match floating_mask.find("X") {
                Some(..) => {
                    let target_bit = floating_mask.replacen("X", "?", 1);
                    let and_mask = target_bit.replace("X", "1").replace("0", "1").replace("?", "0");
                    let and_mask = u64::from_str_radix(&and_mask, 2).unwrap();
                    let address = base_address & and_mask;

                    let zero_version = target_bit.replace("?", "0");
                    self.store_at_permutations(address, value, &zero_version);

                    let or_mask = target_bit.replace("X", "0").replace("1", "0").replace("?", "1");
                    let or_mask = u64::from_str_radix(&or_mask, 2).unwrap();
                    let one_version = floating_mask.replacen("X", "1", 1);
                    let address = base_address | or_mask;

                    let one_version = target_bit.replace("?", "1");
                    self.store_at_permutations(address, value, &one_version);
                },
                None => {
                    self.addresses.insert(base_address, value);
                }
            }
        }
    }
}
