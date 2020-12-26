pub struct ConsecutiveRange {
    pub min: u64,
    pub max: u64
}

pub struct Rule {
    pub name: &'static str,
    pub ranges: &'static [ConsecutiveRange],
}

pub struct Ticket {
    pub fields: &'static [u64],
}

pub struct Notes {
    pub rules: &'static [Rule],
    pub my_ticket: Ticket,
    pub nearby_tickets: &'static [Ticket],
}

// Substitute with:
// '<,'>s/\(\w\+\): \(\d\+\)-\(\d\+\) or \(\d\+\)-\(\d\+\)/        Rule { name: "\1", ranges: &[ ConsecutiveRange { min: \2, max: \3 }, ConsecutiveRange { min: \4, max: \5 } ] },
// '<,'>s/\(.*\)/        Ticket { fields: &[ \1 ] },
pub const SAMPLE_DATA: Notes = Notes {
    rules: &[
        Rule { name: "class", ranges: &[ ConsecutiveRange { min: 1, max: 3 }, ConsecutiveRange { min: 5, max: 7 } ] },
        Rule { name: "row", ranges: &[ ConsecutiveRange { min: 6, max: 11 }, ConsecutiveRange { min: 33, max: 44 } ] },
        Rule { name: "seat", ranges: &[ ConsecutiveRange { min: 13, max: 40 }, ConsecutiveRange { min: 45, max: 50 } ] },
    ],

    my_ticket: Ticket { fields: &[ 7,1,14 ] },

    nearby_tickets: &[
        Ticket { fields: &[ 7,3,47 ] },
        Ticket { fields: &[ 40,4,50 ] },
        Ticket { fields: &[ 55,2,20 ] },
        Ticket { fields: &[ 38,6,12 ] },
    ],
};
