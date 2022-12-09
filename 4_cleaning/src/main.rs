use itertools::Itertools;

fn main() {
    fn solve(string: &str, rule: fn(&((i32, i32), (i32, i32))) -> bool) -> usize {
	string
	    .split("\n")
	    .filter(|line| line.len() > 0)
	    .map(|pair|
		 pair
		 .split(",")
		 .map(|range|
		      range
		      .split("-")
		      .map(|value| {
			  value.parse::<i32>().expect("Expected integer range ends")
		      })
		      .collect_tuple::<(_, _)>().expect("Expected pairs of integers to form ranges"))
		 .collect_tuple::<(_, _)>().expect("Expected pairs of elves"))
	    .filter(rule)
	    .count()
    }

    fn full_overlap(pair: &((i32, i32), (i32, i32))) -> bool {
    	pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1 || pair.1.0 >= pair.0.0 && pair.1.1 <= pair.0.1
    }
    
    fn partial_overlap(pair: &((i32, i32), (i32, i32))) -> bool {
    	pair.0.0 <= pair.1.1 && pair.0.1 >= pair.1.0
    }
    
    let short_string =
r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    
    let input_string : String = std::fs::read_to_string("input").expect("Failed to read input file");
    
    println!("{}", solve(short_string, full_overlap));
    println!("{}", solve(input_string.as_str(), full_overlap));

    println!("{}", solve(short_string, partial_overlap));
    println!("{}", solve(input_string.as_str(), partial_overlap));
}
