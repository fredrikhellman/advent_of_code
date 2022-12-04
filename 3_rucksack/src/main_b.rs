pub mod rs;

fn main() {
    let rucksacks = {
        use std::io::prelude::*;
        rs::read_rucksacks(std::io::stdin().lock().lines())
    };

    println!("{}", rucksacks.chunks_exact(3).map(|value| {
        let badge_item = rs::find_badge(value.try_into().expect("Failed to get three rucksacks"));
        rs::item_to_priority(&badge_item)
    }).sum::<i32>());
}
