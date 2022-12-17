use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

fn dijkstra(vertices: &Vec<HashSet<usize>>) -> Vec<i32> {
    let n = vertices.len();
    let mut dists : Vec<i32> = vec![i32::MAX; n*n];
    for source in 0..n {
	let mut dist = &mut dists[source*n..(source+1)*n];
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
		    println!("dj = {}, di = {}", dist[j], dist[i]);
		    dist[j] = std::cmp::min(dist[j], dist[i] + 1);
		    touched.insert(j);
		}
	    }
    }
    dists
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
}

fn solve_a(input: &str) {
    let a = input.split('\n').map(|line| line.split(';').collect::<Vec<_>>()).collect::<Vec<_>>();
    
    println!("{:?}", a);
}

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve_a(&string);
}
