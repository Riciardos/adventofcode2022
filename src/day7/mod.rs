mod day7 {}

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day7/input.txt") {
        let top_level_node: Node = Node {
            id: "/".to_string(),
            idx: 0,
            val: 0,
            parent: None,
            children: vec![],
        };

        let mut arena = Tree::new();
        arena.add_node(top_level_node.clone());
        arena.set_root(Some(top_level_node));

        let mut commands: Vec<Command> = vec![];

        for line in lines {
            if let Ok(command_line) = line {
                commands.push(Command::new(command_line))
            }
        }
        let mut current_node_idx: usize = 0;

        for command in commands {
            match command.command_type {
                CommandType::ChangeDirTop => {
                    continue;
                }
                CommandType::ChangeDirUp => {
                    let current_node = arena.node_at(current_node_idx);
                    current_node_idx = current_node.unwrap().parent.unwrap();
                    continue;
                }
                CommandType::ChangeDirDown => {
                    let second = command.command_string.split_at(5).1;
                    let current_node = arena.node_at(current_node_idx);
                    let new_node = current_node
                        .unwrap()
                        .children
                        .iter()
                        .find(|x1| x1.id == second)
                        .unwrap();
                    current_node_idx = new_node.idx;
                    continue;
                }
                CommandType::List => {
                    continue;
                }
                CommandType::AddDir => {
                    let second = command.command_string.split_at(4).1;
                    let new_node_index = arena.get_new_index();

                    let current_node = arena.node_at(current_node_idx);
                    let new_node = Node::new(second.to_string(), new_node_index, current_node_idx);

                    current_node.unwrap().children.push(new_node.clone());
                    arena.add_node(new_node);

                    continue;
                }
                CommandType::AddFile => {
                    let line_vec: Vec<&str> = command.command_string.split(" ").collect();
                    let value: i32 = line_vec[0].parse().unwrap();
                    let current_node = arena.node_at(current_node_idx);

                    current_node.unwrap().val += value;
                    continue;
                }
                CommandType::None => {
                    println!(
                        "Something went wrong, not doing anything with: {}",
                        command.command_string
                    );
                }
            }
        }
        arena.arena.iter().for_each(|x| {
            let val = calculate_size(x.as_ref().unwrap(), 0);
            println!("Calculated {} for {}", val, x.as_ref().unwrap().id)
        });
        println!("Day 7 Finished all commands!");
    };
}

fn calculate_size(node: &Node, mut dir_sum: i32) -> i32 {
    let val = node.val;
    return if node.children.is_empty() {
        val
    } else {
        println!("Node {} has value {}", node.id, node.val);
        dir_sum = node
            .children
            .iter()
            .map(|mut x1| calculate_size(x1, dir_sum))
            .sum();
        val + dir_sum
    };
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    idx: usize,
    val: i32,
    parent: Option<usize>,
    children: Vec<Node>,
}

impl Node {
    fn new(id: String, idx: usize, parent: usize) -> Self {
        Self {
            id,
            idx,
            val: 0,
            parent: Some(parent),
            children: vec![],
        }
    }
}
#[derive(Debug, Clone)]
struct Tree {
    arena: Vec<Option<Node>>,
    root: Option<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: Option::None,
        }
    }

    fn set_root(&mut self, root: Option<Node>) {
        self.root = root;
    }

    fn get_new_index(&self) -> usize {
        self.arena.len()
    }

    fn add_node(&mut self, mut node: Node) -> usize {
        let index = self.arena.len();
        node.idx = index;
        self.arena.push(Some(node));
        return index;
    }

    fn node_at(&mut self, index: usize) -> Option<&mut Node> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        };
    }
}

struct Command {
    command_type: CommandType,
    command_string: String,
}

impl Command {
    fn new(command_string: String) -> Command {
        Command {
            command_type: Command::from_string(&command_string),
            command_string,
        }
    }

    fn from_string(string: &String) -> CommandType {
        if string.starts_with("$ cd /") {
            return CommandType::ChangeDirTop;
        }

        if string.starts_with("$ cd ..") {
            return CommandType::ChangeDirUp;
        }

        if string.starts_with("$ cd") {
            return CommandType::ChangeDirDown;
        }
        if string.starts_with("$ ls") {
            return CommandType::List;
        }
        if string.starts_with("dir") {
            return CommandType::AddDir;
        }
        return CommandType::AddFile;
    }
}

enum CommandType {
    ChangeDirTop,
    ChangeDirUp,
    ChangeDirDown,
    List,
    AddDir,
    AddFile,
    None,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
