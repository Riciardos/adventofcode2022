mod day4 {}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    // read line by line
    if let Ok(lines) = read_lines("src/day4/input.txt") {
        let mut total_score = 0;
        for line in lines {
            if let Ok(game_line) = line {
                let split = game_line.split(",");
                let vec: Vec<&str> = split.collect();

                let assignment_one: Vec<&str> = vec[0].split("-").collect();
                let assignment_two: Vec<&str> = vec[1].split("-").collect();

                let mut assignment_pair = AssignmentPair {
                    start_position_one: assignment_one[0].parse().unwrap(),
                    end_position_one: assignment_one[1].parse().unwrap(),
                    start_position_two: assignment_two[0].parse().unwrap(),
                    end_position_two: assignment_two[1].parse().unwrap(),
                    fully_contains: false,
                    overlap: false,
                };
                let fully_contains = assignment_pair.calculate_fully_contains();
                let overlap = assignment_pair.calculate_overlap();
                if fully_contains || overlap {
                    total_score += 1;
                }
            }
        }
        println!("Day 4 Total Score: {}", total_score);
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct AssignmentPair {
    start_position_one: i32,
    end_position_one: i32,
    start_position_two: i32,
    end_position_two: i32,
    fully_contains: bool,
    overlap: bool,
}

impl AssignmentPair {
    fn calculate_fully_contains(&mut self) -> bool {
        if self.start_position_one >= self.start_position_two
            && self.end_position_one <= self.end_position_two
        {
            self.fully_contains = true
        }
        if self.start_position_two >= self.start_position_one
            && self.end_position_two <= self.end_position_one
        {
            self.fully_contains = true
        }
        return self.fully_contains;
    }

    fn calculate_overlap(&mut self) -> bool {
        if self.start_position_one >= self.start_position_two
            && self.start_position_one <= self.end_position_two
        {
            self.overlap = true;
        }
        if self.end_position_one >= self.start_position_two
            && self.end_position_one <= self.end_position_two
        {
            self.overlap = true;
        }
        return self.overlap;
    }
}
