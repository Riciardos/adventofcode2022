mod day1 {}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1() {
    // read line by line
    if let Ok(lines) = read_lines("src/day1/input.csv") {
        let mut current_calories: i32 = 0;
        let mut calories_vec = vec![];

        for line in lines {
            if let Ok(calorie_line) = line {
                if calorie_line == "" {
                    calories_vec.push(current_calories);
                    current_calories = 0;
                } else {
                    let calories: i32 = calorie_line.parse().unwrap();
                    current_calories += calories;
                }
            }
        }
        calories_vec.sort();
        calories_vec.reverse();
        let sum: i32 = calories_vec[..=2].iter().sum();
        println!("Day 1 top 3 summed calories {:?}", sum)
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
