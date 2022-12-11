use std::io::Read;
use std::collections::HashMap;

struct Directory {
    nodes: HashMap<String, Node>
}
impl Directory {
    fn new() -> Directory {
	Directory { nodes: HashMap::new() }
    }
}
struct File {
    size: usize
}

enum Node {
    Directory(Directory),
    File(File)
}

enum Visitor<'a> {
    Visiting(&'a Directory, Vec<&'a mut Directory>)
}

impl<'a> Visitor<'a> {
    fn cd(&mut self, arg: &str) {
	match self {
	    Visitor::Visiting(root, path) => {
		match arg {
		    "/" => {
			path.clear();
		    },
		    ".." => {
			path.pop();
		    }
		    dir => {
			{
			    path.last_mut().unwrap().nodes.entry(dir.to_string()).or_insert(Node::Directory(Directory::new()));
			}
			{
			    let node = path.last().unwrap().nodes.get(&dir.to_string()).unwrap();
			    match node {
				Node::Directory(directory) => path.push(directory),
				Node::File(_) => panic!("Cannot cd into file")
			    }
			}
		    }
		}
	    }
	}
    }
}

fn solve_a(input: &str) {
    let mut root = Directory::new();
    let mut visitor = Visitor::Visiting(&mut root, Vec::new());
    visitor.cd("..");
    visitor.cd("a");
    visitor.cd("b");
    visitor.cd("b");
}


fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).expect("Failed to read from stdin");
    
    solve_a(&string);
}
