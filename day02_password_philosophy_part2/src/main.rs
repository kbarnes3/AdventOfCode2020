#[allow(unused_imports)]
use day02_password_philosophy_common::{PasswordEntry, SAMPLE_DATA, REAL_DATA};

fn main() {
    let valid_count = do_work(&REAL_DATA);
    println!("{}", valid_count);
}

fn do_work(data: &[PasswordEntry]) -> u32 {
    let mut valid_count = 0;
    for entry in data.iter() {
        if (entry.password.chars().nth(entry.min_count - 1) == Some(entry.required_char)) ^
            (entry.password.chars().nth(entry.max_count - 1) == Some(entry.required_char)){
            valid_count += 1;
        }
    }

    valid_count
}
