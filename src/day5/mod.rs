mod day5 {}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/**
            [M] [S] [S]
        [M] [N] [L] [T] [Q]
[G]     [P] [C] [F] [G] [T]
[B]     [J] [D] [P] [V] [F] [F]
[D]     [D] [G] [C] [Z] [H] [B] [G]
[C] [G] [Q] [L] [N] [D] [M] [D] [Q]
[P] [V] [S] [S] [B] [B] [Z] [M] [C]
[R] [H] [N] [P] [J] [Q] [B] [C] [F]
 1   2   3   4   5   6   7   8   9
Start position
 */

pub fn main() {
    // read line by line
    if let Ok(lines) = read_lines("src/day5/input.txt") {
        let mut start_position = vec![
            vec!['R', 'P', 'C', 'D', 'B', 'G'],
            vec!['H', 'V', 'G'],
            vec!['N', 'S', 'Q', 'D', 'J', 'P', 'M'],
            vec!['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M'],
            vec!['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S'],
            vec!['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S'],
            vec!['B', 'Z', 'M', 'H', 'F', 'T', 'Q'],
            vec!['C', 'M', 'D', 'B', 'F'],
            vec!['F', 'C', 'Q', 'G'],
        ];
        for line in lines {
            if let Ok(game_line) = line {
                let line_vec: Vec<&str> = game_line.split(" ").collect();
                let crate_move = Move {
                    number_of_crates: line_vec[1].parse().unwrap(),
                    start_column: line_vec[3].parse().unwrap(),
                    end_column: line_vec[5].parse().unwrap(),
                };
                crate_move.execute_move_multiple(&mut start_position);
            }
        }
        let final_string: String = start_position.iter().map(|x| x.last().unwrap()).collect();
        println!("Day 5 final letters: {}", final_string);
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Copy, Clone)]
struct Move {
    number_of_crates: i32,
    start_column: i32,
    end_column: i32,
}

impl Move {
    fn execute_move(self, crate_array: &mut Vec<Vec<char>>) {
        for _i in 0..self.number_of_crates {
            let popped = crate_array[self.start_column as usize - 1].pop().unwrap();
            crate_array[self.end_column as usize - 1].push(popped);
        }
    }

    fn execute_move_multiple(self, crate_array: &mut Vec<Vec<char>>) {
        let stack_length = crate_array[self.start_column as usize - 1].len() as usize;
        let popped = crate_array[self.start_column as usize - 1]
            .split_off(stack_length - self.number_of_crates as usize);
        crate_array[self.end_column as usize - 1].extend_from_slice(&popped);
    }
}
