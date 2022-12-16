use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
struct State {
    files: HashMap<String, usize>,
    path: Vec<String>
}

impl State {
    fn new() -> State {
	State { files: HashMap::new(), path: Vec::new() }
    }
}

fn path_to_str(path: &Vec<String>) -> String {
    let mut string = String::new();
    for dir in path {
	string.push_str("/");
	string.push_str(dir);
    }
    string
}

fn process_line(state: &mut State, line: &str) {
    let tokens = line.split(' ').collect::<Vec<_>>();
    match *tokens.get(0).unwrap_or(&"") {
	"" => {}
	"$" => {
	    match *tokens.get(1).expect("Missing command after $") {
		"cd" => {
		    match *tokens.get(2).expect("Missing argument for cd") {
			"/" => {
			    state.path.clear();
			}
			".." => {
			    state.path.pop();
			},
			dir => {
			    state.path.push(String::from(dir));
			}
		    }
		}
		"ls" => {}
		_ => panic!("Unknown command")
	    }
	},
	ls_output => {
	    match ls_output {
		"dir" => {}
		size_str => {
		    let size = size_str.parse::<usize>().expect("Expected integer as output for file size.");
		    let filename = path_to_str(&state.path) + format!("/{}", *tokens.get(1).expect("Expected filename in second column.")).as_str();
		    state.files.entry(filename).or_insert(size);
		}
	    }
	}
    };
}

fn accumulate_sizes(files: &HashMap<String, usize>) -> HashMap<String, usize>
{
    let mut output : HashMap<String, usize> = HashMap::new();
    let iter =
	files
	.iter()
	.map(|(name, size)| {
	    name.split('/')
		.scan("".to_owned(), |path_name, dir_name| {
		    *path_name += &(dir_name.to_string() + "/");
		    Some((path_name.to_owned(), *size))
		}).collect::<Vec<(String, usize)>>().into_iter().rev().skip(1)
	}).flatten();
    
    for (path_name, size) in iter {
	output.entry(path_name).and_modify(|x| *x += size).or_insert(size);
    }
    
    output
}

fn solve_a(input: &str) {
    let mut state = State::new();
    for line in input.split('\n') {
	process_line(&mut state, line)
    }
    let dir_sizes = accumulate_sizes(&state.files);

    println!("{}",
	     dir_sizes.iter().filter(|(_name, &size)| {
		 size <= 100000
	     })
	     .map(|(_, size)| size)
	     .sum::<usize>())
}

fn solve_b(input: &str) {
    let mut state = State::new();
    for line in input.split('\n') {
	process_line(&mut state, line)
    }
    let dir_sizes = accumulate_sizes(&state.files);
    
    let used_space = dir_sizes.get("/").expect("Root directory missing");
    let want_unused_space = 30000000;
    let total_space = 70000000;
    
    let mut sorted_sizes = dir_sizes.values().collect::<Vec<_>>();
    sorted_sizes.sort();
    println!("{}", sorted_sizes
	     .iter()
	     .find(|&&size| {
		 total_space - used_space + size >= want_unused_space
	     }).expect("Found no directory to remove"))
}


fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve_a(&string);
    solve_b(&string);
}
