pub enum Instruction {
    Noop,
    Accumulate(isize),
    Jump(isize)
}

use Instruction::*;

// Substitute with
// '<,'>s/+//e | '<,'>s/^nop .*$/    Noop,/e | '<,'>s/^acc \(.*\)$/    Accumulate(\1),/e | '<,'>s/^jmp \(.*\)/    Jump(\1),/e
pub const SAMPLE_DATA: [Instruction; 9] = [
    Noop,
    Accumulate(1),
    Jump(4),
    Accumulate(3),
    Jump(-3),
    Accumulate(-99),
    Accumulate(1),
    Jump(-4),
    Accumulate(6),
];
