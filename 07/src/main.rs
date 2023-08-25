use indextree::{Arena, Node, NodeId};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Fil {
    name: String,
    size: usize,
}

struct Dir {
    name: String,
    files: Vec<Fil>,
}

enum Command {
    Cd(String),
    Ls,
    Noop,
}

impl Fil {
    fn new(name: &str, size: usize) -> Fil {
        Fil {
            name: name.to_string(),
            size,
        }
    }
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name: name.to_string(),
            files: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.files.iter().fold(0, |sum, f| sum + f.size)
    }
}

fn parse_command(tokens: &mut VecDeque<&str>) -> Command {
    let command = tokens.pop_front().unwrap();
    if command == "cd" {
        let dir_name = tokens.pop_front().unwrap();
        return Command::Cd(dir_name.to_string());
    } else if command == "ls" {
        return Command::Ls;
    }
    Command::Noop
}

struct Tree<'a> {
    arena: &'a mut Arena<Dir>,
    root: Option<NodeId>,
    current_dir: Option<NodeId>,
}

fn perform_command(cmd: Command, tree: &mut Tree) {
    match cmd {
        Command::Ls => (),
        Command::Noop => (),
        Command::Cd(dir_name) => {
            if dir_name == ".." {
                if let Some(cur) = tree.current_dir {
                    let node = tree.arena.get(cur).unwrap();
                    let parent = node.parent().unwrap();
                    tree.current_dir = Some(parent);
                }
            } else {
                let dir = Dir::new(&dir_name);
                let new_dir = tree.arena.new_node(dir);
                if let Some(cur) = tree.current_dir {
                    cur.append(new_dir, tree.arena);
                } else {
                    tree.root = Some(new_dir);
                }
                tree.current_dir = Some(new_dir);
            }
        }
    }
}

fn dir_size(tree: &Tree, node_id: NodeId) -> usize {
    if let Some(_node) = tree.arena.get(node_id) {
        let sum = node_id.descendants(tree.arena).fold(0, |sum, d| {
            let nd = tree.arena.get(d).unwrap().get();
            sum + nd.size()
        });
        return sum;
    }
    0
}

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let arena: &mut Arena<Dir> = &mut Arena::new();

    let mut tree = Tree {
        arena,
        root: None,
        current_dir: None,
    };

    for line in reader.lines().flatten() {
        let mut tokens = line.split_ascii_whitespace().collect::<VecDeque<&str>>();
        let first = tokens.pop_front().unwrap().clone();
        if first == "$" {
            let cmd = parse_command(&mut tokens);
            perform_command(cmd, &mut tree);
        } else if first == "dir" {
            // let dir_name = tokens.pop_front().unwrap();
            // let dir = Dir::new(dir_name);
        } else if first.chars().next().unwrap().is_numeric() {
            let file_name = tokens.pop_front().unwrap();
            let size = first.parse::<usize>().unwrap();
            let file = Fil::new(file_name, size);
            if let Some(node) = tree.arena.get_mut(tree.current_dir.unwrap()) {
                let d = node.get_mut();
                d.files.push(file);
            }
        }
    }

    let total_size = 70000000;
    let required_size = 40000000;
    let used_size = dir_size(&tree, tree.root.unwrap());
    let missing = used_size - required_size;

    println!("Hierachy:");
    let mut filtered_size = 0;
    let mut prospects = Vec::new();
    tree.root.unwrap().descendants(tree.arena).for_each(|d| {
        let node = tree.arena.get(d).unwrap();
        let dir = node.get();
        let dir_size = dir_size(&tree, d);
        if dir_size <= 100000 {
            filtered_size += dir_size;
        }
        if dir_size > missing {
            prospects.push(d);
        }
        println!("{} - {}", dir.name, dir_size);
        dir.files.iter().for_each(|f| {
            println!("\t{} - {}", f.name, f.size);
        });
    });
    println!("Filtered size: {}", filtered_size);

    println!("Used size {}", used_size);
    println!("Missing: {}", missing);

    let min_dir = prospects.iter().min_by_key(|d| dir_size(&tree, **d));
    let min_dir_size = dir_size(&tree, *min_dir.unwrap());
    println!("Min dir to delete: {}", min_dir_size);
}
