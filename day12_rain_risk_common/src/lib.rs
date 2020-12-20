pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}

use Instruction::*;

// Substitute with:
// '<,'>s/^/    / | '<,'>s/$/),/ | '<,'>s/N/North(/e | '<,'>s/S/South(/e | '<,'>s/E/East(/e | '<,'>s/W/West(/e | '<,'>s/L/Left(/e | '<,'>s/R/Right(/e | '<,'>s/F/Forward(/e
pub const SAMPLE_DATA: [Instruction; 5] = [
    Forward(10),
    North(3),
    Forward(7),
    Right(90),
    Forward(11),
];
