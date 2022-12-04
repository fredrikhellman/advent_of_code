use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Compartment {
    items: HashMap<char, i32>
}

impl Compartment {
    fn new(s: &str) -> Compartment {
        let mut items = HashMap::<char, i32>::new();
        for c in s.chars() {
            items.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        }
        Compartment { items }
    }
}

pub struct Rucksack {
    pub first: Compartment,
    pub second: Compartment
}

impl Rucksack {
    fn new(s: String) -> Rucksack {
        let size = s.len();
        let s1 = s.chars().take(size/2).collect::<String>();
        let s2 = s.chars().skip(size/2).take(size/2).collect::<String>();
        Rucksack { first: Compartment::new(s1.as_str()), second: Compartment::new(s2.as_str()) }
    }

    pub fn all_items(&self) -> HashSet<char> {
        let first_items: HashSet<char> = self.first.items.keys().cloned().collect();
        let second_items: HashSet<char> = self.second.items.keys().cloned().collect();
        first_items.union(&second_items).cloned().collect()
    }

    pub fn common_items(&self) -> HashSet<char> {
        let first_items: HashSet<char> = self.first.items.keys().cloned().collect();
        let second_items: HashSet<char> = self.second.items.keys().cloned().collect();
        first_items.intersection(&second_items).cloned().collect()
    }
}

pub fn find_badge(rucksacks: &[Rucksack; 3]) -> char {
    let badge_set : HashSet<char> = rucksacks[0].all_items().intersection(&rucksacks[1].all_items()).cloned().collect::<HashSet<char>>().intersection(&rucksacks[2].all_items()).cloned().collect();
    if badge_set.len() != 1 {
        panic!("Multiple badges found");
    }
    else {
        *badge_set.iter().next().unwrap()
    }
}

pub fn item_to_priority(c: &char) -> i32 {
    match c {
        'a'..='z' => (*c as i32) - ('a' as i32) + 1,
        'A'..='Z' => (*c as i32) - ('A' as i32) + 27,
        _ => panic!("Unknown item")
    }
}

pub fn priority(items: &HashSet<char>) -> i32 {
    items.iter().map(item_to_priority).sum::<i32>()
}

pub fn read_rucksacks<B>(mut lines: std::io::Lines<B>) -> Vec<Rucksack> where B: std::io::BufRead {
    let mut rucksacks = Vec::new();
    while let Some(next_result) = lines.next() {
        let string = next_result.expect("Failed to read line");
        rucksacks.push(Rucksack::new(string));
    }
    rucksacks
}
