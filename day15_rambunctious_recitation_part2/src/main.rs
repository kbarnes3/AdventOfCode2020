use std::collections::HashMap;
#[allow(unused_imports)]
use day15_rambunctious_recitation_common::{PART_2_ITERATIONS, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(starting_data: &[usize]) -> usize {
    let mut spoken_numbers = HashMap::new();
    let mut last_spoken = 0;

    for round in 1..=starting_data.len() {
        last_spoken = starting_data[round - 1];
        spoken_numbers.insert(last_spoken, round);
    }

    for round in starting_data.len()+1..=PART_2_ITERATIONS {
        let next_spoken;
        if spoken_numbers.contains_key(&last_spoken) {
            let last_spoken_in = spoken_numbers[&last_spoken];
            next_spoken = round - last_spoken_in - 1;
        } else {
            next_spoken = 0;
        }
        spoken_numbers.insert(last_spoken, round - 1);
        last_spoken = next_spoken;
    }

    last_spoken
}
