#[allow(unused_imports)]
use day04_passport_processing_common::{SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work(data: &[&str]) -> usize {
    let mut valid_count: usize = 0;
    let mut start_line: usize = 0;
    let mut end_line: usize;

    for (i, line) in data.iter().enumerate() {
        if line.trim() == "" {
            end_line = i;
            if process_passport(&data[start_line..end_line+1]) {
                valid_count += 1;
            }
            start_line = end_line + 1;
        }
    }

    if start_line < data.len() {
        end_line = data.len() - 1;
        if process_passport(&data[start_line..end_line+1]) {
            valid_count += 1;
        }
    }
    
    valid_count
}

fn process_passport(data: &[&str]) -> bool {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    for line in data.iter() {
        for field in line.trim().split_whitespace() {
            if field.starts_with("byr") {
                byr = true;
            } else if field.starts_with("iyr") {
                iyr = true;
            } else if field.starts_with("eyr") {
                eyr = true;
            } else if field.starts_with("hgt") {
                hgt = true;
            } else if field.starts_with("hcl") {
                hcl = true;
            } else if field.starts_with("ecl") {
                ecl = true;
            } else if field.starts_with("pid") {
                pid = true;
            }
        }
    }

    byr && iyr && eyr && hgt && hcl && ecl && pid
}
