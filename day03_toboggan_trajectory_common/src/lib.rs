#[derive(Copy, Clone)]
pub enum Space {
    Open,
    Tree
}

use Space::{Open, Tree};

// Convert with:
// '<,'>s/^/    [ /
// '<,'>s/$/],
// '<,'>s/\./Open, /g
// '<,'>s/#/Tree, /g

pub const SAMPLE_DATA: [[Space; 11]; 11] = [
    [ Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open, ],
    [ Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open, ],
    [ Open, Tree, Open, Open, Open, Open, Tree, Open, Open, Tree, Open, ],
    [ Open, Open, Tree, Open, Tree, Open, Open, Open, Tree, Open, Tree, ],
    [ Open, Tree, Open, Open, Open, Tree, Tree, Open, Open, Tree, Open, ],
    [ Open, Open, Tree, Open, Tree, Tree, Open, Open, Open, Open, Open, ],
    [ Open, Tree, Open, Tree, Open, Tree, Open, Open, Open, Open, Tree, ],
    [ Open, Tree, Open, Open, Open, Open, Open, Open, Open, Open, Tree, ],
    [ Tree, Open, Tree, Tree, Open, Open, Open, Tree, Open, Open, Open, ],
    [ Tree, Open, Open, Open, Tree, Tree, Open, Open, Open, Open, Tree, ],
    [ Open, Tree, Open, Open, Tree, Open, Open, Open, Tree, Open, Tree, ],
];
