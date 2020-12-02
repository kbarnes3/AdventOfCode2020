#[allow(unused_imports)]
use day02_password_philosophy_common::{PasswordEntry, SAMPLE_DATA, REAL_DATA};

fn main() {
    let valid_count = do_work(&REAL_DATA);
    println!("{}", valid_count);
}

fn do_work(data: &[PasswordEntry]) -> u32 {
    let mut valid_count = 0;
    for entry in data.iter() {
        let mut char_count = 0;
        for c in entry.password.chars() {
            if c == entry.required_char {
                char_count += 1;
            }
        }

        if (char_count >= entry.min_count) && (char_count <= entry.max_count) {
            valid_count += 1;
        }
    }

    valid_count
}
