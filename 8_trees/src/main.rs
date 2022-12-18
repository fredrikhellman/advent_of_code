use std::io::Read;

fn solve_a(input: &str) {
    let digits = input.chars().filter_map(|c| {
	match c {
	    '0'..='9' => Some(c as i32 - '0' as i32),
	    _ => None
	}
    }).collect::<Vec<i32>>();
    let mut seen = digits.clone().iter().map(|_| false).collect::<Vec<bool>>();
	
    let n = f32::sqrt(digits.len() as f32) as usize;
    
    let counter = |(n, max_h), hs : (&i32, &mut bool) | -> (i32, i32) {
	let (&h, s) = hs;
	if h > max_h {
	    if !*s {
		*s = true;
		(n + 1, h)
	    } else {
		(n, h)
	    }
	} else {
	    (n, max_h)
	}
    };

    println!("{}",
	     (0..n).map(|i| {
		 [digits.iter().zip(seen.iter_mut()).skip(i*n).take(n).fold((0, -1), counter),
		  digits.iter().zip(seen.iter_mut()).skip(i*n).take(n).rev().fold((0, -1), counter),
		  digits.iter().zip(seen.iter_mut()).skip(i).step_by(n).take(n).fold((0, -1), counter),
		  digits.iter().zip(seen.iter_mut()).skip(i).step_by(n).take(n).rev().fold((0, -1), counter)].iter().map(|(n, _)| *n).sum::<i32>()
	     }).sum::<i32>());
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve_a(&string);
}
