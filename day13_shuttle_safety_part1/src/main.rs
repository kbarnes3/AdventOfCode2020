use std::cmp::Ordering;
use std::vec::Vec;
#[allow(unused_imports)]
use day13_shuttle_safety_common::{BusRoute, BusNotes, SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

#[derive(Eq)]
struct PossibleBus {
    id: u64,
    departure_time: u64
}

impl Ord for PossibleBus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.departure_time.cmp(&other.departure_time)
    }
}

impl PartialOrd for PossibleBus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PossibleBus {
    fn eq(&self, other: &Self) -> bool {
        self.departure_time == other.departure_time
    }
}

fn do_work(data: &BusNotes) -> u64 {
    let mut departure_times = Vec::with_capacity(data.bus_routes.len());

    for route in data.bus_routes.iter() {
        match route {
            BusRoute::Bus(id) => {
                let divisor = data.earliest_time / id;
                let mut departure_time = id * divisor;
                if departure_time == data.earliest_time {
                    // We can leave immediately
                } else if departure_time < data.earliest_time {
                    departure_time += id;
                } else {
                    panic!("I don't know how division works!");
                }

                departure_times.push(PossibleBus {
                    id: *id,
                    departure_time
                });
            },
            BusRoute::OutOfService => (),
        }
    }

    let earliest_bus = departure_times.iter().min().unwrap();
    let waiting_time = earliest_bus.departure_time - data.earliest_time;

    waiting_time * earliest_bus.id
}
