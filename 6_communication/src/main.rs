use std::io::Read;
use itertools::Itertools;

fn find_marker(input: &str, length: usize) -> usize {
    length +
	input
	.chars()
	.collect::<Vec<char>>()
	.windows(length)
	.position(|chars| {
	    chars.iter().unique().count() == chars.len()
	}).expect("Found no marker")
}

fn solve_a(input: &str) {
    println!("{}", find_marker(input, 4));
}

fn solve_b(input: &str) {
    println!("{}", find_marker(input, 14));
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve_a(&string);
    solve_b(&string);
}
