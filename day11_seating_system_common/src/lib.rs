#[derive(PartialEq, Copy, Clone)]
pub enum Position {
    Floor,
    Empty,
    Occupied
}

use Position::*;

// Substitute with:
// '<,'>s/^/    [ / | '<,'>s/$/],/ | '<,'>s/\./Floor, /ge | '<,'>s/L/Empty, /ge | '<, '>s/#/Occupied, /ge
pub const SAMPLE_DATA: [[Position; 10]; 10] = [
    [ Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty, ],
    [ Empty, Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty, ],
    [ Empty, Floor, Empty, Floor, Empty, Floor, Floor, Empty, Floor, Floor, ],
    [ Empty, Empty, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty, ],
    [ Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty, ],
    [ Empty, Floor, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty, ],
    [ Floor, Floor, Empty, Floor, Empty, Floor, Floor, Floor, Floor, Floor, ],
    [ Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, ],
    [ Empty, Floor, Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty, ],
    [ Empty, Floor, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty, ],
];
