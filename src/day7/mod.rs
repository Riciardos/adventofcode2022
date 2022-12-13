mod day7 {}

use super::util::read_lines;
use std::borrow::{Borrow, BorrowMut};

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

        let commands: Vec<Command> = lines
            .map(|line| {
                return if let Ok(command_line) = line {
                    Command::new(command_line)
                } else {
                    Command::new("NOOP".to_string())
                };
            })
            .collect();

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

                    let mut current_node = arena.node_at(current_node_idx).unwrap();
                    let new_node = Node::new(second.to_string(), new_node_index, current_node_idx);
                    current_node.children.push(new_node.clone());
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
        let mut sum: i32 = arena
            .arena
            .iter()
            .map(|x| {
                let node = x.clone();
                let node_ref = &mut node.unwrap();
                return calculate_size(node_ref, 0, arena.clone());
            })
            .filter(|x| x < &100_000)
            .sum();

        let mut potentials: Vec<Node> = vec![];
        let dir_list: Vec<&Option<Node>> = arena
            .arena
            .iter()
            .filter(|x| {
                let x1 = *x;
                let node = x1.clone();
                let potential = x1.clone();
                let node_ref = &mut node.unwrap();
                let val = calculate_size(node_ref, 0, arena.clone());
                if val > 358913 {
                    let mut potential_copy = potential.clone().unwrap();
                    potential_copy.val = val;
                    potentials.push(potential_copy);
                }
                return val >= 358913;
            })
            .collect();

        potentials.sort_by(|x, y| x.val.cmp(&y.val));
        println!("Day 7 total sum: {:?}", sum);
        println!("Day 7 potential dir: {:?}", potentials[0]);
    };
}

fn calculate_size(node: &mut Node, mut dir_sum: i32, mut arena: Tree) -> i32 {
    return if node.children.is_empty() {
        node.val
    } else {
        let sum_children: i32 = node
            .children
            .iter_mut()
            .map(|x| {
                let arena_clone = arena.clone();
                let child_node = arena.node_at(x.idx).unwrap();
                calculate_size(child_node, dir_sum + child_node.val, arena_clone)
            })
            .sum();
        node.val += sum_children;
        return node.val;
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
        if string.starts_with("NOOP") {
            return CommandType::None;
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
