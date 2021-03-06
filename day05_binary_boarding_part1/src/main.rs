#[allow(unused_imports)]
use day05_binary_boarding_common::{SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);

    println!("{}", result);
}

fn do_work(data: &[u16]) -> u16 {
    *data.iter().max().unwrap()
}
