#[allow(unused_imports)]
use day10_adapter_array_common::{SAMPLE_DATA_1, SAMPLE_DATA_2, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[u32]) -> u32 {
    let mut adapters = data.to_vec();
    adapters.sort();
    let biggest_adapter = adapters[adapters.len() - 1];

    adapters.insert(0, 0);
    adapters.insert(adapters.len(), biggest_adapter + 3);

    let mut one_differences = 0;
    let mut three_differences = 0;

    for i in 0..adapters.len() - 1 {
        let difference  = adapters[i + 1] - adapters[i];
        if difference == 1 {
            one_differences += 1;
        } else if difference == 3 {
            three_differences += 1;
        }
    }

    one_differences * three_differences
}
