#[allow(unused_imports)]
use day03_toboggan_trajectory_common::{Space, SLOPES, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<Grid: AsRef<[Row]>, Row: AsRef<[Space]>>(data: Grid) -> u32
{
    let mut slope_product: u32 = 1;

    for slope in SLOPES.iter() {
        slope_product *= check_slope(&data, slope.x_increment, slope.y_increment);
    }

    slope_product
}

fn check_slope<Grid: AsRef<[Row]>, Row: AsRef<[Space]>>(
    data: Grid,
    x_increment: usize,
    y_increment: usize) -> u32
{
    let mut tree_count = 0;
    let mut x: usize = x_increment;
    let mut y: usize = y_increment;

    let max_x = data.as_ref()[0].as_ref().len();
    let max_y = data.as_ref().len();

    while y < max_y {
        while x >= max_x {
            x -= max_x;
        }

        let space = data.as_ref()[y].as_ref()[x];
        if let Space::Tree = space {
            tree_count += 1;
        }

        x += x_increment;
        y += y_increment;

    }

    tree_count
}
