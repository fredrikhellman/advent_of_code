pub mod inventory;
    
fn main() {
    let inventories = {
        use std::io::prelude::*;
        let handle = std::io::stdin().lock();
        let mut lines = handle.lines();
        inventory::read_all_inventories(&mut lines)
    };

    match inventories.iter().map(|vec| vec.iter().sum::<i32>()).max() {
        Some(value) => println!("{}", value),
        None => println!("There were no inventories")
    }
}
