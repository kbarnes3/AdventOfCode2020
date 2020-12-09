use std::collections::HashMap;
#[allow(unused_imports)]
use day07_handy_haversacks_common::{BagInfo, TARGET_BAG, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[BagInfo]) -> usize {
    let mut bag_data = HashMap::new();

    for bag in data.iter() {
        bag_data.insert(bag.color, bag);
    }

    let mut known_contained_bags = HashMap::new();

    get_contained_bags(TARGET_BAG, &bag_data, &mut known_contained_bags)
}

fn get_contained_bags(
    target_bag: &'static str,
    all_bag_data: &HashMap::<&str, &BagInfo>,
    known_counts: &mut HashMap::<&str, usize>) -> usize {

    if let Some(count) = known_counts.get(target_bag) {
        return *count;
    }

    let bag_data = all_bag_data.get(target_bag).expect(&format!("Missing data for {}", target_bag));
    let mut contained_count = 0;

    for contained_bag in bag_data.contains.iter() {
        let mut count = get_contained_bags(contained_bag.color, all_bag_data, known_counts);
        count += 1; // Include the bag itself
        contained_count += count * contained_bag.count;
    }

    known_counts.insert(target_bag, contained_count);
    contained_count
}

