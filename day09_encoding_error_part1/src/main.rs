#[allow(unused_imports)]
use day09_encoding_error_common::{SAMPLE_DATA_PREAMBLE, SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA, SAMPLE_DATA_PREAMBLE);
    match result {
        Some(value) => println!("{}", value),
        None => println!("Error")
    }
}

fn do_work(data: &[u32], preamble_len: usize) -> Option<u32> {
    for i in preamble_len..data.len() {
        let target = data[i];
        let candidates = &data[i - preamble_len..i];
        if !is_sum(target, candidates) {
            return Some(target);
        }
    }
    None
}

fn is_sum(target: u32, candidates: &[u32]) -> bool {
    for (i, first) in candidates.iter().enumerate() {
        let second_candidates = &candidates[i + 1..];
        for (_, second) in second_candidates.iter().enumerate() {
            if first + second == target {
                return true;
            }
        }
    }

    false
}