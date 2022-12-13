mod day1 {}

use super::util::read_lines;

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
