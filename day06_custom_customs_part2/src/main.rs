use std::collections::HashMap;
#[allow(unused_imports)]
use day06_custom_customs_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[&str]) -> usize {
    let mut total: usize = 0;
    let mut start_line: usize = 0;
    let mut end_line: usize;

    for (i, line) in data.iter().enumerate() {
        if line.trim() == "" {
            end_line = i;
            total += process_group(&data[start_line..end_line]);
            start_line = end_line + 1;
        }
    }

    if start_line < data.len() {
        total += process_group(&data[start_line..data.len()]);
    }
    
    total
}

fn process_group(group: &[&str]) -> usize {
    let mut yes_answers = HashMap::<char, usize>::new();

    for person in group.iter() {
        for yes_answer in person.trim().chars() {
            let counter = yes_answers.entry(yes_answer).or_insert(0);
            *counter += 1;
        }
    }

    let group_size = group.iter().len();
    let mut all_match = 0;
    for (_answer, count) in yes_answers.iter() {
        if *count == group_size {
            all_match += 1;
        }
    }

    all_match
}
