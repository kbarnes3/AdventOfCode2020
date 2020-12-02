#[allow(unused_imports)]
use day01_report_repair_common::{TARGET_NUMBER, SAMPLE_DATA, REAL_DATA};

fn main() {
    match do_work(&REAL_DATA) {
        Some(answer) => println!("{}", answer),
        None => println!("Sad"),
    }
}

fn do_work(data: &[u32]) -> Option<u32> {
    for (i, outer_element) in data.iter().enumerate() {
        for (j, inner_element) in data.iter().enumerate() {
            if i != j {
                let sum = outer_element + inner_element;
                if sum == TARGET_NUMBER {
                    let product = outer_element * inner_element;
                    return Some(product);
                }
            }
        }
    }

    None
}