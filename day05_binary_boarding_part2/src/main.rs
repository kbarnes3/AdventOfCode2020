use std::collections::HashMap;
#[allow(unused_imports)]
use day05_binary_boarding_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);

    println!("{}", result);
}

fn do_work(data: &[u16]) -> u16 {
    let mut max = data[0];
    let mut min = data[0];

    let mut seats = HashMap::new();

    for seat in data.iter() {
        if *seat > max {
            max = *seat;
        } else if *seat < min {
            min = *seat;
        }

        seats.insert(*seat, true);
    }

    for i in min..max {
        if let None = seats.get(&i) {
            return i;
        }
    }

    panic!("No seat found");
}
