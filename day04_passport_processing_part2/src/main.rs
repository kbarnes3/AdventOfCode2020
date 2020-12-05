#[allow(unused_imports)]
use day04_passport_processing_common::{SAMPLE_DATA, SAMPLE_DATA_2, SAMPLE_DATA_3, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
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
            if field.starts_with("byr:") {
                byr = validate_birth_year(&field[4..]);
            } else if field.starts_with("iyr:") {
                iyr = validate_issue_year(&field[4..]);
            } else if field.starts_with("eyr:") {
                eyr = validate_expiration_year(&field[4..]);
            } else if field.starts_with("hgt:") {
                hgt = validate_height(&field[4..]);
            } else if field.starts_with("hcl:") {
                hcl = validate_hair_color(&field[4..]);
            } else if field.starts_with("ecl:") {
                ecl = validate_eye_color(&field[4..]);
            } else if field.starts_with("pid:") {
                pid = validate_passport_id(&field[4..]);
            }
        }
    }

    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn validate_birth_year(value: &str) -> bool {
    validate_year(&value, 1920, 2002)
}

fn validate_issue_year(value: &str) -> bool {
    validate_year(&value, 2010, 2020)
}

fn validate_expiration_year(value: &str) -> bool {
    validate_year(&value, 2020, 2030)
}

fn validate_height(value: &str) -> bool {
    let number = &value[0..(value.len()-2)];
    let number = match number.parse::<u8>() {
        Ok(value) => value,
        Err(_) => return false
    };

    if value.ends_with("cm") {
        return (number >= 150) && (number <= 193)
    } else if value.ends_with("in") {
        return (number >= 59) && (number <= 76)
    }

    false
}

fn validate_hair_color(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }

    if value.chars().nth(0) != Some('#') {
        return false;
    }

    for character in value[1..].chars() {
        if (character < '0' && character > '9') &&
            (character < 'a' && character > 'f') {
            return false
        }
    }

    true
}

fn validate_eye_color(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn validate_passport_id(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }

    match value.parse::<u32>() {
        Ok(_) => true,
        Err(_) => false
    }
}

fn validate_year(value: &str, min: u16, max: u16) -> bool {
    let year: u16 = match value.parse::<u16>() {
        Ok(value) => value,
        Err(_) => return false
    };

    (year >= min) && (year <= max)
}
