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

fn solve_b(input: &str) {
    let digits = input.chars().filter_map(|c| {
	match c {
	    '0'..='9' => Some(c as i32 - '0' as i32),
	    _ => None
	}
    }).collect::<Vec<i32>>();
	
    let n = f32::sqrt(digits.len() as f32) as usize;

    let counter = |(n, max_h), &h| -> (i32, i32) {
	if h < max_h {
	    (n + 1, max_h)
	} else if max_h > 0 && h >= max_h {
	    (n + 1, -1)
	} else {
	    (n, -1)
	}
    };

    println!("{}",
	     (0..n).map(|r| {
		 (0..n).map(|c| {
		     let max_h = digits[r*n + c];
		     let right = digits.iter().skip(r*n + c + 1).take(n - 1 - c).fold((0, max_h), counter).0;
		     let left = digits.iter().rev().skip((n-1-r)*n + (n-1-c) + 1).take(c).fold((0, max_h), counter).0;
		     let down = digits.iter().skip(r*n + c + n).step_by(n).take(n - 1 - r).fold((0, max_h), counter).0;
		     let up = digits.iter().rev().skip((n-1-r)*n + (n-1-c) + n).step_by(n).take(r).fold((0, max_h), counter).0;
		     right*left*down*up
		 }).max().unwrap()
	     }).max().unwrap());
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve_a(&string);
    solve_b(&string);
}
