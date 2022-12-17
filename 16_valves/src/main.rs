use std::io::Read;
use std::collections::HashSet;

fn dijkstra(vertices: &Vec<HashSet<usize>>) -> Vec<i32> {
    let n = vertices.len();
    let mut dists : Vec<i32> = vec![i32::MAX; n*n];
    for source in 0..n {
	let dist = &mut dists[source*n..(source+1)*n];
	dist[source] = 0;
	
	let mut touched : HashSet<usize> = HashSet::new();
	touched.insert(source);
	
	let mut remains : HashSet<usize> = HashSet::new();
	for i in 0..n {
	    remains.insert(i);
	}

	while let Some((i, _)) =
	    touched.intersection(&remains)
	    .map(|&i| (i, dist[i]))
	    .fold(None,
		  |best, (i, d)| {
		      match best {
			  None => Some((i, d)),
			  Some((_, min)) if d < min  => Some((i, d)),
			  _ => best
		      }
		  }) {
		
		remains.remove(&i);
		let neighbors = &vertices[i];
		for &j in neighbors.intersection(&remains) {
		    dist[j] = std::cmp::min(dist[j], dist[i] + 1);
		    touched.insert(j);
		}
	    }
    }
    dists
}

#[derive(Debug)]
struct Cave {
    rates: Vec<i32>,
    dists: Vec<i32>,
    start: usize
}

fn remove_jammed_valves(cave: &Cave) -> Cave {
    let keep = cave.rates.iter().enumerate().filter(|(i, &r)| r > 0 || *i == cave.start).map(|(i, _)| i).collect::<Vec<usize>>();
    let n = keep.len();
    let n_old = cave.rates.len();
    let mut rates : Vec<i32> = vec![0; n];
    let mut dists : Vec<i32> = vec![0; n*n];
    for i in 0..n {
	rates[i] = cave.rates[keep[i]];
	for j in 0..n {
	    dists[i*n + j] = cave.dists[keep[i]*n_old + keep[j]];
	}
    }
    let start = keep.iter().position(|i| *i == cave.start).unwrap();
    Cave {rates, dists, start}
}

fn brute_force(cave: &Cave, start_times: &Vec<i32>) -> i32{
    fn recursive_loss(cave: &Cave, path: &mut Vec<usize>, current: Option<usize>, time_left: i32, start_times: &Vec<i32>) -> i32 {
	if time_left <= 0 {
	    if let Some(new_time_left) = start_times.iter().next() {
		let new_start_times = start_times.iter().skip(1).cloned().collect::<Vec<i32>>();
		recursive_loss(cave, path, None, *new_time_left, &new_start_times)
	    } else {
		0
	    }
	} else {
	    let n = cave.rates.len();
	    
	    let current_loss = match current {
		None => 0,
		Some(c) => time_left * cave.rates[c]
	    };
	    
	    let mut max_loss = 0;
	    for next in 0..n {
		if path.contains(&next) {
		    continue;
		} else {
		    path.push(next);
		    let current_vertex = current.unwrap_or(cave.start);
		    let loss = recursive_loss(cave, path, Some(next), time_left - cave.dists[current_vertex*n + next] - 1, start_times);
		    if loss > max_loss {
			max_loss = loss;
		    }
		    path.pop();
		}
	    }
	    // Don't do anything the remaining time
	    let loss = recursive_loss(cave, path, None, 0, start_times);
	    if loss > max_loss {
		max_loss = loss;
	    }
	    current_loss + max_loss
	}
    }

    let mut path : Vec<usize> = Vec::new();
    let loss = recursive_loss(&cave, &mut path, None, 0, start_times);
    loss
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dijkstra() {
	let vertices = vec![HashSet::from([2, 3]), HashSet::from([2]), HashSet::from([0, 1]), HashSet::from([0])];
	assert_eq!(dijkstra(&vertices),
		   vec![0, 2, 1, 1,
			2, 0, 1, 3,
			1, 1, 0, 2,
			1, 3, 2, 0]);
    }

    #[test]
    fn test_alg() {
	let rates = vec![0, 10, 1];
	let dists = vec![  0, 10,   1,
		   	  10,  0,  10,
	    		   1, 10,   0];
	let cave = Cave { rates, dists };
	assert_eq!(brute_force(&cave, 10), 9);
	assert_eq!(brute_force(&cave, 11), 10);
	assert_eq!(brute_force(&cave, 12), 21);
    }
}

fn parse_cave_from_input(input: &str) -> Cave {
    let mut names : Vec<String> = Vec::new();
    let mut rates : Vec<i32> = Vec::new();
    let mut start = None;
    for line in input.split('\n') {
	let string = line.split(';').next().expect("Expected semicolon");
	let mut tokens = string.split(' ');
	let name = tokens.nth(1).expect("Failed to get valve name").to_string();
	if name == "AA" {
	    start = Some(names.len());
	}
	let rate = tokens.nth(2).expect("Failed to find rate token").split('=').last().expect("Failed to find rate").parse::<i32>().expect("Failed to parse rate");
	names.push(name);
	rates.push(rate)
    }

    let mut vertices : Vec<HashSet<usize>> = Vec::new();
    for line in input.split('\n') {
	let string = line.split(';').last().expect("Expected semicolon");
	let tokens = string.split(' ').skip(5);

	vertices.push(HashSet::new());
	for token in tokens {
	    let name = token.chars().filter(|&c| c >= 'A' && c <= 'Z').collect::<String>();
	    let i = names.iter().position(|n| &name == n).expect("Failed to find valve name");
	    vertices.last_mut().unwrap().insert(i);
	};
    }
    println!("{:?}", rates);
    let dists = dijkstra(&vertices);

    Cave { rates, dists, start: start.expect("Found no AA node") }
}

fn solve_a(input: &str) {
    let cave = parse_cave_from_input(input);
    let cave_small = remove_jammed_valves(&cave);
    println!("{}", brute_force(&cave_small, &vec![30]));
}

fn solve_b(input: &str) {
    let cave = parse_cave_from_input(input);
    let cave_small = remove_jammed_valves(&cave);
    println!("{}", brute_force(&cave_small, &vec![26, 26]));
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");

    solve_a(&string);
    solve_b(&string);
}
