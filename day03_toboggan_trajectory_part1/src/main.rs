#[allow(unused_imports)]
use day03_toboggan_trajectory_common::{Space, SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<Grid: AsRef<[Row]>, Row: AsRef<[Space]>>(data: Grid) -> u32
{
    let mut tree_count = 0;
    const X_INCREMENT: usize = 3;
    const Y_INCREMENT: usize = 1;
    let mut x: usize = X_INCREMENT;
    let mut y: usize = Y_INCREMENT;

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

        x += X_INCREMENT;
        y += Y_INCREMENT;

    }

    tree_count
}