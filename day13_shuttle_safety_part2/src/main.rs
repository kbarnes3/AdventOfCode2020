#[allow(unused_imports)]
use day13_shuttle_safety_common::{BusRoute, BusNotes, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work(data: &BusNotes) -> u64 {
    let mut start_time = data.start_consecutive_search;
    
    loop {
        if validate_time(start_time, data.bus_routes) {
            return start_time;
        }

        start_time += 1;
    }
}

fn validate_time(start_time: u64, data: &[BusRoute]) -> bool {
    let mut time = start_time;
    for route in data.iter() {
        match route {
            BusRoute::Bus(id) => {
                if time % id != 0 {
                    return false;
                }
            },
            BusRoute::OutOfService => ()
        }

        time += 1;
    }

    true
}