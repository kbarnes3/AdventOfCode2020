#[derive(Copy, Clone)]
pub enum Cube {
    Active,
    Inactive
}

use Cube::*;

pub const CYCLES: usize = 6;

// Substitute with:
// '<,'>s/^/    [ / | '<,'>s/$/],/ | '<,'>s/\./Inactive, /g | '<,'>s/#/Active, /g
pub const SAMPLE_DATA: [[Cube; 3]; 3] = [
    [ Inactive, Active, Inactive, ],
    [ Inactive, Inactive, Active, ],
    [ Active, Active, Active, ],
];
