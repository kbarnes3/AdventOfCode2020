use std::collections::HashSet;
#[allow(unused_imports)]
use day07_handy_haversacks_common::{BagInfo, TARGET_BAG, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[BagInfo]) -> usize {
    let mut made_progress = true;
    let mut valid_bags = HashSet::new();

    while made_progress {
        made_progress = false;

        for bag in data.iter() {
            if !valid_bags.contains(bag.color) {
                for contained_bag in bag.contains.iter() {
                    if (contained_bag.color == TARGET_BAG) ||
                        (valid_bags.contains(contained_bag.color)) {
                        valid_bags.insert(bag.color);
                        made_progress = true;
                    }
                }
            }
        }
    }

    valid_bags.len()
}
