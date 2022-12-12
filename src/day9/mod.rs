mod day9 {}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day9/input.txt") {
        let mut moves: Vec<Move> = vec![];
        for line in lines {
            if let Ok(move_line) = line {
                moves.push(Move::new(move_line));
            }
        }

        let mut expanded_moves: Vec<Move> = vec![];
        for head_move in moves {
            (0..head_move.steps).for_each(|x| {
                expanded_moves.push(Move::from_direction(head_move.direction.clone()))
            })
        }
        let mut head_positions: Vec<Position> = vec![];

        let start_position = Position {
            x_coord: 0,
            y_coord: 0,
        };

        head_positions.push(start_position);

        let mut current_position = start_position;
        for step in expanded_moves {
            let next_position = next_position(&current_position, step);
            head_positions.push(next_position);
            current_position = next_position;
        }

        let mut tail_positions_1: Vec<Position> = vec![];
        tail_positions_1.push(start_position.clone());
        let mut tail_current_position = start_position;
        let multiple_heads = head_positions.clone();

        for head_position in head_positions {
            let some_tail_move = should_tail_move(&tail_current_position, &head_position);
            if let Some(tail_move) = some_tail_move {
                let tail_next_position = next_position(&tail_current_position, tail_move);
                tail_positions_1.push(tail_next_position.clone());
                tail_current_position = tail_next_position;
            }
        }

        let unique_tail_positions: HashSet<Position> = HashSet::from_iter(tail_positions_1.clone());
        println!(
            "Day 9 unique tail positions {:?}",
            unique_tail_positions.len()
        );

        let mut tail_position_vec: Vec<Vec<Position>> = vec![];
        let mut tail_current_positions: Vec<Position> = vec![];

        for i in 0..9 {
            let mut positions = vec![];
            positions.push(start_position);
            tail_position_vec.push(positions);
            tail_current_positions.push(start_position);
        }

        for head_position in multiple_heads {
            for i in 0..9 {
                let mut head;
                if i == 0 {
                    head = head_position;
                } else {
                    head = tail_current_positions[i - 1]
                }
                let some_tail_move_1 = should_tail_move(&tail_current_positions[i], &head);
                if let Some(tail_move) = some_tail_move_1 {
                    let tail_next_position = next_position(&tail_current_positions[i], tail_move);
                    tail_position_vec[i].push(tail_next_position.clone());
                    tail_current_positions[i] = tail_next_position;
                }
            }
        }

        let unique_tail_positions_long_rope: HashSet<Position> =
            HashSet::from_iter(tail_position_vec[8].clone());
        println!(
            "Day 9 unique long rope tail positions {:?}",
            unique_tail_positions_long_rope.len()
        );
    }
}

fn next_position(position: &Position, step: Move) -> Position {
    let mut new_x_coord = position.x_coord;
    let mut new_y_coord = position.y_coord;
    match step.direction {
        Direction::UP => new_y_coord += 1,
        Direction::DOWN => new_y_coord -= 1,
        Direction::LEFT => new_x_coord -= 1,
        Direction::RIGHT => new_x_coord += 1,
        Direction::UP_LEFT => {
            new_x_coord -= 1;
            new_y_coord += 1;
        }
        Direction::UP_RIGHT => {
            new_x_coord += 1;
            new_y_coord += 1;
        }
        Direction::DOWN_LEFT => {
            new_x_coord -= 1;
            new_y_coord -= 1;
        }
        Direction::DOWN_RIGHT => {
            new_x_coord += 1;
            new_y_coord -= 1;
        }
    }
    return Position {
        x_coord: new_x_coord,
        y_coord: new_y_coord,
    };
}

fn should_tail_move(tail_position: &Position, head_position: &Position) -> Option<Move> {
    if (head_position.x_coord - tail_position.x_coord).abs() > 1
        && (head_position.y_coord - tail_position.y_coord).abs() > 1
    {
        if head_position.y_coord > tail_position.y_coord {
            if head_position.x_coord > tail_position.x_coord {
                return Option::from(Move::from_direction(Direction::UP_RIGHT));
            } else {
                return Option::from(Move::from_direction(Direction::UP_LEFT));
            }
        } else {
            if head_position.x_coord > tail_position.x_coord {
                return Option::from(Move::from_direction(Direction::DOWN_RIGHT));
            } else {
                return Option::from(Move::from_direction(Direction::DOWN_LEFT));
            }
        }
    }
    if (head_position.x_coord - tail_position.x_coord).abs() > 1 {
        return if head_position.y_coord == tail_position.y_coord {
            if head_position.x_coord > tail_position.x_coord {
                return Option::from(Move::from_direction(Direction::RIGHT));
            }
            Option::from(Move::from_direction(Direction::LEFT))
        } else {
            if head_position.y_coord > tail_position.y_coord {
                if head_position.x_coord > tail_position.x_coord {
                    return Option::from(Move::from_direction(Direction::UP_RIGHT));
                }
                Option::from(Move::from_direction(Direction::UP_LEFT))
            } else {
                if head_position.x_coord > tail_position.x_coord {
                    return Option::from(Move::from_direction(Direction::DOWN_RIGHT));
                }
                Option::from(Move::from_direction(Direction::DOWN_LEFT))
            }
        };
    }

    if (head_position.y_coord - tail_position.y_coord).abs() > 1 {
        return if head_position.x_coord == tail_position.x_coord {
            if head_position.y_coord > tail_position.y_coord {
                return Option::from(Move::from_direction(Direction::UP));
            }
            Option::from(Move::from_direction(Direction::DOWN))
        } else {
            if head_position.x_coord > tail_position.x_coord {
                if head_position.y_coord > tail_position.y_coord {
                    return Option::from(Move::from_direction(Direction::UP_RIGHT));
                }
                Option::from(Move::from_direction(Direction::DOWN_RIGHT))
            } else {
                if head_position.y_coord > tail_position.y_coord {
                    return Option::from(Move::from_direction(Direction::UP_LEFT));
                }
                Option::from(Move::from_direction(Direction::DOWN_LEFT))
            }
        };
    }
    return Option::None;
}

#[derive(Debug, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UP_LEFT,
    UP_RIGHT,
    DOWN_LEFT,
    DOWN_RIGHT,
}

impl Direction {
    fn from_char(input_char: char) -> Direction {
        return match input_char {
            'U' => Direction::UP,
            'D' => Direction::DOWN,
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => panic!("Direction not possible"),
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x_coord: i32,
    y_coord: i32,
}

struct Move {
    direction: Direction,
    steps: u32,
}

impl Move {
    fn new(input: String) -> Move {
        let parts: Vec<&str> = input.split(" ").collect();
        Move {
            direction: Direction::from_char(parts.get(0).unwrap().chars().next().unwrap()),
            steps: parts.get(1).unwrap().parse().unwrap(),
        }
    }

    fn from_direction(direction: Direction) -> Move {
        Move {
            direction,
            steps: 1,
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
