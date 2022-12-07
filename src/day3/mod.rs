mod day3 {}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    // read line by line
    if let Ok(lines) = read_lines("src/day3/input.txt") {
        let collected_lines: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
        let mut total_priority = 0;
        let mut i = 0;
        while i < collected_lines.len() {
            let mut rucksack = RuckSack {
                compartment_one: &collected_lines[i],
                compartment_two: &collected_lines[i + 1],
                compartment_three: &collected_lines[i + 2],
                shared_item: '0',
            };
            let _shared_item = rucksack.find_shared_item();
            let priority = rucksack.calculate_priority();
            total_priority += priority;
            i += 3;
        }

        println!("Day 3 Total Priority {}", total_priority);
    };
}

struct RuckSack<'a> {
    compartment_one: &'a str,
    compartment_two: &'a str,
    compartment_three: &'a str,
    shared_item: char,
}

impl RuckSack<'_> {
    fn find_shared_item(&mut self) -> char {
        let mut char_match: char = '0';
        for character in self.compartment_one.chars() {
            for possible_match in self.compartment_two.chars() {
                for final_match in self.compartment_three.chars() {
                    if character == possible_match && final_match == possible_match {
                        char_match = character;
                    }
                }
            }
        }
        self.shared_item = char_match;
        return char_match;
    }

    fn calculate_priority(&self) -> i32 {
        match self.shared_item {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => panic!("not possible"),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
