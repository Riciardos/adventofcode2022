mod day7 {}

use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

pub fn main() {
    // read line by line
    if let Ok(lines) = read_lines("src/day7/input.txt") {
        let mut current_dir: TreeNode<i32> = TreeNode {
            id: String::from("/"),
            value: 0,
            parent: None,
            children: vec![],
        };

        let mut command = Command {
            command_type: CommandType::None,
            command_string: String::new(),
        };

        for line in lines {
            if let Ok(command_line) = line {
                if command_line.starts_with("$ cd") {
                    command.command_type = CommandType::ChangeDir;
                    if command_line.starts_with("$ cd ..") {
                        current_dir = current_dir.parent;
                    }
                    continue;
                }

                if command_line.starts_with("$ ls") {
                    command.command_type = CommandType::List;
                    continue;
                }

                if command_line.starts_with("dir") {
                    let child_dir = TreeNode {
                        id: command_line.substring(4, command_line.len()),
                        value: 0,
                        parent: Ok(current_dir),
                        children: vec![],
                    };
                    current_dir.add_child(child_dir);
                }
            }
        }
    };
}

struct Command {
    command_type: CommandType,
    command_string: String,
}

enum CommandType {
    ChangeDir,
    List,
    None,
}

#[derive(PartialEq)]
struct TreeNode<T> {
    id: String,
    value: T,
    parent: &'static TreeNode<T>,
    children: Vec<TreeNode<T>>,
}

impl TreeNode<i32> {
    fn new() -> TreeNode<i32> {
        return TreeNode {
            id: String::new(),
            value: 0,
            children: vec![],
            parent: None,
        };
    }
    fn add_child(&mut self, new_node: Rc<RefCell<TreeNode<T>>>) {
        self.children.push(new_node)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
