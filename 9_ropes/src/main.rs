use std::io::Read;

fn solve(input: &str, m: usize) {
    let mut lines = input.split('\n');

    let mut knots = vec![(0, 0); m+1];
    let mut visits = Vec::new();
    
    while let Some(line) = lines.next() {
	if line.len() == 0 {
	    break;
	}
	let mut tokens = line.split(' ');
	let dir = tokens.next().unwrap().chars().next().unwrap();
	let n = tokens.next().unwrap().parse::<i32>().unwrap();
	for _ in 0..n {
	    match dir {
		'R' => knots[0].0 += 1,
		'L' => knots[0].0 -= 1,
		'U' => knots[0].1 += 1,
		'D' => knots[0].1 -= 1,
		_ => panic!()
	    };
	    for j in 0..m {
		let d = (knots[j+1].0 - knots[j].0, knots[j+1].1 - knots[j].1);
		let dt = |a, b : i32| {
		    match (a, b) {
			(2, _) => -1,
			(1, x) if x.abs() == 2 => -1,
			(-2, _) => 1,
			(-1, x) if x.abs() == 2 => 1,
			_ => 0
		    }
		};
		knots[j+1].0 += dt(d.0, d.1);
		knots[j+1].1 += dt(d.1, d.0);
	    }
	    visits.push(knots[m]);
	}
    }
    visits.sort();
    visits.dedup();
    println!("{}", visits.len());
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve(&string, 1);
    solve(&string, 9);
}
