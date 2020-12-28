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

pub const REAL_DATA: [[Cube; 8]; 8] = [
    [ Active, Active, Active, Active, Active, Inactive, Inactive, Active, ],
    [ Active, Inactive, Inactive, Active, Active, Active, Inactive, Active, ],
    [ Active, Active, Active, Inactive, Inactive, Inactive, Inactive, Inactive, ],
    [ Inactive, Active, Inactive, Active, Inactive, Active, Inactive, Inactive, ],
    [ Active, Active, Inactive, Active, Inactive, Inactive, Active, Inactive, ],
    [ Active, Active, Active, Active, Active, Active, Inactive, Inactive, ],
    [ Inactive, Active, Active, Inactive, Inactive, Active, Active, Active, ],
    [ Active, Active, Active, Inactive, Active, Active, Active, Active, ],
];
