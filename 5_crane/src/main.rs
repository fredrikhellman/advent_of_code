use std::io::Read;
use std::collections::HashMap;
use itertools::sorted;

type Stacks = HashMap::<char, Vec<char>>;

fn initialize(preamble: &str) -> Stacks {
    let mut reverse_preamble_lines = preamble.split('\n').rev();
    let stack_numbers = reverse_preamble_lines.next().expect("Failed to get stack numbers in preamble");
    let ordered_stack_crate_pairs = reverse_preamble_lines.map(|line| {
	stack_numbers.chars().zip(line.chars()).filter(|(stack, crat)| {
	    match (*stack, *crat) {
		('1'..='9', 'A'..='Z') => true,
		_ => false
	    }
	})
    }).flatten();

    let mut stacks = Stacks::new();
    for (stack, crat) in ordered_stack_crate_pairs {
	stacks.entry(stack).or_insert(Vec::new()).push(crat);
    }
    stacks
}

fn crane_operation_9000(stacks: &mut Stacks, count: &usize, from: &char, to: &char) { 
    let from_stack = stacks.get_mut(from).expect("Reference (from) to nonexisting stack");
    let mut crates = from_stack.split_off(from_stack.len() - *count);
    crates.reverse();
    let to_stack = stacks.get_mut(to).expect("Reference (to) to nonexisting stack");
    to_stack.append(&mut crates);
}

fn crane_operation_9001(stacks: &mut Stacks, count: &usize, from: &char, to: &char) { 
    let from_stack = stacks.get_mut(from).expect("Reference (from) to nonexisting stack");
    let mut crates = from_stack.split_off(from_stack.len() - *count);
    let to_stack = stacks.get_mut(to).expect("Reference (to) to nonexisting stack");
    to_stack.append(&mut crates);
}

fn operate_crane(stacks: &mut Stacks, instructions: &str, crane_operation: fn(&mut Stacks, &usize, &char, &char)) {
    let re = regex::Regex::new(r"move ([0-9]+) from ([1-9]) to ([1-9])").unwrap();
    for cap in re.captures_iter(instructions) {
	let count = *(&cap[1].parse::<usize>().expect("Stack number must be an integer"));
	let from = &cap[2].chars().nth(0).unwrap();
	let to = &cap[3].chars().nth(0).unwrap();
	crane_operation(stacks, &count, from, to)
    }
}

fn stack_top(stacks: &Stacks) -> String {
    let mut string = String::new();
    for key in sorted(stacks.keys()) {
	string.push(*stacks.get(key).unwrap().last().expect("Empty stack found"));
    }
    string
}


fn solve_a(input: &str) {
    let sections : [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().expect("Expected a preamble followed by move instructions separated by a blank line.");
    let mut stacks = initialize(sections[0]);
    operate_crane(&mut stacks, sections[1], crane_operation_9000);
    println!("{}", stack_top(&stacks));
}

fn solve_b(input: &str) {
    let sections : [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().expect("Expected a preamble followed by move instructions separated by a blank line.");
    let mut stacks = initialize(sections[0]);
    operate_crane(&mut stacks, sections[1], crane_operation_9001);
    println!("{}", stack_top(&stacks));
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");

    solve_a(&string);
    solve_b(&string);
}
