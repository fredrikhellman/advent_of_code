pub mod inventory;
    
fn main() {
    let inventories = {
        use std::io::prelude::*;
        let handle = std::io::stdin().lock();
        let mut lines = handle.lines();
        inventory::read_all_inventories(&mut lines)
    };

    let mut total_calories : Vec<i32> = inventories.iter().map(|vec| vec.iter().sum()).collect();
    total_calories.sort();
    total_calories.reverse();
    println!("{}", total_calories[..3].iter().sum::<i32>())
}
