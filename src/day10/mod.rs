mod day10 {}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day10/input.txt") {
        let mut register_x: i32 = 1;
        let mut num_of_cycles: i32 = 1;
        let mut commands = vec![];
        for line in lines {
            if let Ok(command_line) = line {
                commands.push(Command::new(command_line));
            }
        }

        let mut signal_strength = 0;

        let commands_pixels = commands.clone();

        for command in commands {
            match command.command_type {
                CommandType::NoOp => {
                    for i in 0..command.cycles {
                        match num_of_cycles {
                            20 | 60 | 100 | 140 | 180 | 220 => {
                                signal_strength += register_x * num_of_cycles
                            }
                            _ => (),
                        }
                        num_of_cycles += 1;
                    }
                }
                CommandType::AddX => {
                    for i in 0..command.cycles {
                        match num_of_cycles {
                            20 | 60 | 100 | 140 | 180 | 220 => {
                                signal_strength += register_x * num_of_cycles
                            }
                            _ => (),
                        }
                        num_of_cycles += 1;
                    }
                    register_x += command.value;
                }
            }
        }

        register_x = 1;
        num_of_cycles = 0;
        let mut pixels: [[char; 40]; 6] = [
            ['.'; 40], ['.'; 40], ['.'; 40], ['.'; 40], ['.'; 40], ['.'; 40],
        ];

        for command in commands_pixels {
            match command.command_type {
                CommandType::NoOp => {
                    for i in 0..command.cycles {
                        let x_coord = num_of_cycles % 40;
                        let y_coord = num_of_cycles / 40;
                        if (0 <= register_x && register_x < 40)
                            && (register_x == x_coord
                                || register_x + 1 == x_coord
                                || register_x - 1 == x_coord)
                        {
                            pixels[y_coord as usize][x_coord as usize] = '#';
                        }
                        num_of_cycles += 1;
                    }
                }
                CommandType::AddX => {
                    for i in 0..command.cycles {
                        let x_coord = num_of_cycles % 40;
                        let y_coord = num_of_cycles / 40;

                        if (0 <= register_x && register_x < 40)
                            && (register_x == x_coord
                                || register_x + 1 == x_coord
                                || register_x - 1 == x_coord)
                        {
                            pixels[y_coord as usize][x_coord as usize] = '#';
                        }
                        num_of_cycles += 1;
                    }
                    register_x += command.value;
                }
            }
        }

        println!("Day 10 Total signal strength : {}", signal_strength);
        println!("Day 10 CRT");
        pixels.iter().for_each(|x| {
            x.iter().for_each(|y| print!("{}", y));
            println!();
        });
    }
}

#[derive(Debug, Clone)]
struct Command {
    command_type: CommandType,
    command_string: String,
    cycles: u32,
    value: i32,
}

impl Command {
    fn new(command_string: String) -> Command {
        let string_clone = command_string.clone();
        Command {
            command_type: Command::from_string(&string_clone),
            command_string,
            cycles: Command::get_cycle(&string_clone),
            value: Command::get_value(&string_clone),
        }
    }

    fn from_string(string: &String) -> CommandType {
        if string.starts_with("noop") {
            return CommandType::NoOp;
        }

        if string.starts_with("addx") {
            return CommandType::AddX;
        }
        panic!["Unknown command type: {}", string]
    }

    fn get_value(string: &String) -> i32 {
        if string.starts_with("addx") {
            let split: Vec<&str> = string.split(" ").collect();
            return split.get(1).unwrap().parse().unwrap();
        }
        return 0;
    }

    fn get_cycle(string: &String) -> u32 {
        if string.starts_with("addx") {
            return 2;
        }
        return 1;
    }
}

#[derive(Debug, Clone)]
enum CommandType {
    NoOp,
    AddX,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
