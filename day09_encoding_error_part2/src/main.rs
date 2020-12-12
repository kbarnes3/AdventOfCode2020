#[allow(unused_imports)]
use day09_encoding_error_common::{SAMPLE_DATA_PREAMBLE, SAMPLE_DATA, REAL_DATA_PREAMBLE, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA, REAL_DATA_PREAMBLE);
    match result {
        Some(value) => println!("{}", value),
        None => println!("Error")
    }
}

fn do_work(data: &[u64], preamble_len: usize) -> Option<u64> {
    let invalid_number = find_invalid_number(data, preamble_len);
    if let Some(invalid_number) = invalid_number {
        let sum_range = find_contiguous_sum(data, invalid_number);
        if let Some(range) = sum_range {

            let sum = range.iter().min().unwrap() + range.iter().max().unwrap();
            return Some(sum);
        }
    }

    None
}

fn find_invalid_number(data: &[u64], preamble_len: usize) -> Option<u64> {
    for i in preamble_len..data.len() {
        let target = data[i];
        let candidates = &data[i - preamble_len..i];
        if !is_sum(target, candidates) {
            return Some(target);
        }
    }
    None
}

fn is_sum(target: u64, candidates: &[u64]) -> bool {
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

fn find_contiguous_sum(data: &[u64], target: u64) -> Option<&[u64]> {
    for i in 0..data.len() - 1 {
        let mut j = i + 1;
        let mut sum = data[i] + data[j];

        loop {
            if sum == target {
                return Some(&data[i..=j]);
            }

            j += 1;
            if j >= data.len() {
                break;
            }

            sum += data[j];
            if sum > target {
                break;
            }
        }
    }

    None
}