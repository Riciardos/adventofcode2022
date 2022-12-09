mod day8 {}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day8/input.txt") {
        let mut map: Vec<Vec<Point>> = lines
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                let tree_line: Vec<Point> = line
                    .unwrap()
                    .chars()
                    .enumerate()
                    .map(|(j, tree_height)| {
                        Point::new(i, j, false, tree_height.to_digit(10).unwrap())
                    })
                    .collect();
                return tree_line;
            })
            .collect();

        let mut map_clone = map.clone();

        map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, point)| {
                map_clone[i][j].is_visible = check_is_visible(point, &map);
            })
        });

        let total_visible: usize = map_clone
            .iter()
            .map(|x| x.iter().filter(|x| x.is_visible).count())
            .sum();

        println!("Day 8 total visible: {:?}", total_visible);
    }
}

#[derive(Debug, Clone)]
struct Point {
    x_coord: usize,
    y_coord: usize,
    is_visible: bool,
    tree_height: u32,
}

impl Point {
    fn new(x: usize, y: usize, is_visible: bool, tree_height: u32) -> Point {
        Point {
            x_coord: x,
            y_coord: y,
            is_visible,
            tree_height,
        }
    }
}

fn check_is_visible(point: &Point, map: &Vec<Vec<Point>>) -> bool {
    let x_coord = point.x_coord;
    let y_coord = point.y_coord;
    let mut visible_left: bool = true;
    let mut visible_right: bool = true;
    let mut visible_top: bool = true;
    let mut visible_down: bool = true;

    for i in 0..x_coord {
        if map[i][y_coord].tree_height >= point.tree_height {
            visible_top = false;
        };
    }
    for j in (x_coord..map[x_coord].len() - 1).rev() {
        if map[j + 1][y_coord].tree_height >= point.tree_height {
            visible_down = false;
        };
    }

    for k in 0..y_coord {
        if map[x_coord][k].tree_height >= point.tree_height {
            visible_left = false;
        };
    }

    for l in (y_coord..map[x_coord].len() - 1).rev() {
        if map[x_coord][l + 1].tree_height >= point.tree_height {
            visible_right = false;
        };
    }

    return visible_left || visible_right || visible_top || visible_down;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
