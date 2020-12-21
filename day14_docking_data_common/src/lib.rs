pub enum Instruction {
    SetMask { and_mask: u64, or_mask: u64 },
    Store { address: u64, value: u64 }
}

use Instruction::*;

// Substitute with:
// '<,'>s/mask = \(\w*\)/    SetMask { and_mask: 0b\1, or_mask: 0b\1 },/e | '<,'>s/mem\[\(\d\+\)\] = \(\d\+\)/    Store { address: \1, value: \2 },/e | '<,'>s/and_mask: \zs\w\+\ze,/\=substitute(submatch(0), 'X', '1', 'g')/ | '<,'>s/or_mask: \zs\w\+\ze /\=substitute(submatch(0), 'X', '0', 'g')/
pub const SAMPLE_DATA: [Instruction; 4] = [
    SetMask { and_mask: 0b111111111111111111111111111111111101, or_mask: 0b000000000000000000000000000001000000 },
    Store { address: 8, value: 11 },
    Store { address: 7, value: 101 },
    Store { address: 8, value: 0 },
];
