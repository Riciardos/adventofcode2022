mod day6 {}

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    // read line by line
    if let Ok(lines) = read_lines("src/day6/input.txt") {
        for line in lines {
            if let Ok(game_line) = line {
                let mut marker = Marker {
                    position: 0,
                    current_input: VecDeque::from([]),
                    marker_found: false,
                    marker_size: 14,
                };
                for character in game_line.chars() {
                    marker.add_char(character);
                    marker.check_current_input();
                    if marker.marker_found {
                        break;
                    }
                }
                println!("Day 6 Marker found! At position {}", marker.position);
            }
        }
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Marker {
    position: i32,
    current_input: VecDeque<char>,
    marker_found: bool,
    marker_size: i32,
}

impl Marker {
    fn check_current_input(&mut self) {
        if self.position < self.marker_size {
            return;
        }
        for i in 0..self.current_input.len() {
            for j in 0..self.current_input.len() {
                if i != j && self.current_input[i as usize] == self.current_input[j as usize] {
                    return;
                }
            }
        }
        self.marker_found = true
    }

    fn add_char(&mut self, character: char) {
        self.current_input.push_back(character);
        if self.current_input.len() > self.marker_size as usize {
            self.current_input.pop_front();
        }
        self.position += 1;
    }
}
