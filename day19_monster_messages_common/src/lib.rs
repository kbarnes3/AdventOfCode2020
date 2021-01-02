pub enum RuleDescription {
    Literal(char),
    Sequence(&'static [u64]),
    Or(&'static [u64], &'static [u64])
}

use RuleDescription::*;

pub struct Rule {
    pub id: u64,
    pub description: RuleDescription
}

pub const TARGET_RULE: u64 = 0;

// Substitute with:
// '<,'>s/\(\d\+\): "\(\w\)"/    Rule { id: \1, description: Literal('\2') } ,/ | '<,'>s/\(\d\)$/\1,/ | '<,'>s/\(\d\) /\1, /g | '<,'>s/\(\d\+\): \(.*\) | \(.*\)/    Rule { id: \1, description: Or(\&[ \2 ], \&[ \3 ]) },/ | '<,'>s/\(\d\+\): \(.*\)/    Rule { id: \1, description: Sequence(\&[ \2 ]) },/
pub const SAMPLE_RULES: [Rule; 6] = [
    Rule { id: 0, description: Sequence(&[ 4, 1, 5, ]) },
    Rule { id: 1, description: Or(&[ 2, 3, ], &[ 3, 2, ]) },
    Rule { id: 2, description: Or(&[ 4, 4, ], &[ 5, 5, ]) },
    Rule { id: 3, description: Or(&[ 4, 5, ], &[ 5, 4, ]) },
    Rule { id: 4, description: Literal('a') } ,
    Rule { id: 5, description: Literal('b') } ,
];

// Substitute with:
// '<,'>s/\(.*\)/    "\1",/
pub const SAMPLE_MESSAGES: [&str; 5] = [
    "ababbb",
    "bababa",
    "abbbab",
    "aaabbb",
    "aaaabbb",
];
