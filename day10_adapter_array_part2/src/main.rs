use std::vec::Vec;
#[allow(unused_imports)]
use day10_adapter_array_common::{SAMPLE_DATA_1, SAMPLE_DATA_2, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[u32]) -> u64 {
    let mut adapters = data.to_vec();
    adapters.push(0);
    adapters.sort();

    let biggest_adapter = adapters[adapters.len() - 1];

    adapters.push(biggest_adapter + 3);

    let mut possible_routes = Vec::with_capacity(adapters.len());
    possible_routes.push(1);

    for i in 1..adapters.len() {
        let current_joltage = adapters[i];
        let mut current_routes = 0;
        let mut j = i - 1;

        loop {
            let prev_joltage = adapters[j];
            if prev_joltage + 3 >= current_joltage {
                current_routes += possible_routes[j];
            } else {
                break;
            }

            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }

        possible_routes.push(current_routes);
    }

    possible_routes[possible_routes.len() - 1]
}
