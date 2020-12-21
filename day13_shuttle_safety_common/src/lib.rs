pub enum BusRoute {
    Bus(u64),
    OutOfService,
}

pub struct BusNotes {
    pub earliest_time: u64,
    pub bus_routes: &'static [BusRoute],
}

use BusRoute::*;

// Substitute with:
// '<,'>s/\(\d\+\),/        Bus(\1),/ge | '<,'>s/x,/        OutOfService,/ge | '<,'>s/,/,\r/g
pub const SAMPLE_DATA: BusNotes = BusNotes {
    earliest_time: 939,
    bus_routes: &[
        Bus(7),
        Bus(13),
        OutOfService,
        OutOfService,
        Bus(59),
        OutOfService,
        Bus(31),
        Bus(19),
    ]
};
