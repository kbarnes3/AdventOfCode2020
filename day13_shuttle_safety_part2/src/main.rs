use std::cmp::Ordering;
use std::vec::Vec;
#[allow(unused_imports)]
use day13_shuttle_safety_common::{BusRoute, BusNotes, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

#[derive(Eq)]
struct BusRemainder {
    id: u64,
    remainder: u64,
}

impl Ord for BusRemainder {
    fn cmp(&self, other:&Self) -> Ordering {
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for BusRemainder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BusRemainder {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn do_work(data: &BusNotes) -> u64 {
    let remainders = build_remainders(data.bus_routes);

    find_consecutive_time(&remainders[..], data.start_consecutive_search, 1)
}

fn build_remainders(data: &[BusRoute]) -> Vec::<BusRemainder> {
    let mut result = Vec::new();
    for (i, route) in data.iter().enumerate() {
        match route {
            BusRoute::Bus(id) => {
                let i = i as u64;
                let remainder;

                if i <= *id {
                    remainder = (*id - i) % *id;
                } else {
                    let scale = (i / *id) + 1;
                    remainder = ((scale * *id) - i) % *id;
                }

                result.push(BusRemainder { 
                    id: *id,
                    remainder
                });
            },
            BusRoute::OutOfService => ()
        };
    }

    result.sort_unstable();

    result
}

fn find_consecutive_time(data: &[BusRemainder], current_time: u64, lcm: u64) -> u64 {
    if data.len() == 0 {
        return current_time;
    }

    let mut time = current_time;
    let next = &data[0];
    
    while time % next.id != next.remainder {
        time += lcm;
    }

    let lcm = lcm * next.id;

    find_consecutive_time(&data[1..], time, lcm)
}
