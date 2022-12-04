pub mod rs;

fn main() {
    let rucksacks = {
        use std::io::prelude::*;
        rs::read_rucksacks(std::io::stdin().lock().lines())
    };
    
    println!("{}", rucksacks.iter().map(|r| rs::priority(&r.common_items())).sum::<i32>());
}
