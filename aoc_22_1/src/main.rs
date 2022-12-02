fn read_next_inventory<B>(lines: &mut std::io::Lines<B>) -> Option<Vec<i32>> where B: std::io::BufRead {
    let mut end_of_file = true;
    let mut inventory : Vec<i32> = Vec::new();
    
    while let Some(next_result) = lines.next() {
        end_of_file = false;
        let line = next_result.expect("Failed to read line");
        let line_trimmed = line.trim();
        match line_trimmed {
            "" => break,
            _ => {
                let calories : i32 = line_trimmed.parse().expect("Failed to parse calories value");
                inventory.push(calories);
            }
        }
    }
    if end_of_file {
        None
    }
    else {
        Some(inventory)
    }
}

fn read_all_inventories<B>(lines: &mut std::io::Lines<B>) -> Vec<Vec<i32>> where B: std::io::BufRead {
    let mut inventories : Vec<Vec<i32>> = Vec::new();
    while let Some(inventory) = read_next_inventory(lines) {
        inventories.push(inventory);
    }
    inventories
}

fn main() {
    let inventories = {
        use std::io::prelude::*;
        let handle = std::io::stdin().lock();
        let mut lines = handle.lines();
        read_all_inventories(&mut lines)
    };

    match inventories.iter().map(|vec| vec.iter().sum::<i32>()).max() {
        Some(value) => println!("{}", value),
        None => println!("There were no inventories")
    }
}
