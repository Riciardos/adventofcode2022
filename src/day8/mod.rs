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
                        Point::new(i, j, false, tree_height.to_digit(10).unwrap(), 0)
                    })
                    .collect();
                return tree_line;
            })
            .collect();

        let mut map_clone = map.clone();

        map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, point)| {
                map_clone[i][j].is_visible = check_is_visible(point, &map);
                map_clone[i][j].visibility_score = check_visibility(point, &map);
            })
        });

        let total_visible: usize = map_clone
            .iter()
            .map(|x| x.iter().filter(|x| x.is_visible).count())
            .sum();

        let highest_view_score = map_clone
            .iter()
            .flatten()
            .map(|x| x.visibility_score)
            .max()
            .unwrap();
        println!("Day 8 total visible: {:?}", total_visible);
        println!("Day 8 highest visibility score: {}", highest_view_score);
    }
}

#[derive(Debug, Clone)]
struct Point {
    x_coord: usize,
    y_coord: usize,
    is_visible: bool,
    tree_height: u32,
    visibility_score: i32,
}

impl Point {
    fn new(x: usize, y: usize, is_visible: bool, tree_height: u32, visibility_score: i32) -> Point {
        Point {
            x_coord: x,
            y_coord: y,
            is_visible,
            tree_height,
            visibility_score,
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

fn check_visibility(point: &Point, map: &Vec<Vec<Point>>) -> i32 {
    let x_coord = point.x_coord;
    let y_coord = point.y_coord;

    let mut visible_line_left: i32 = 0;
    let mut visible_line_right: i32 = 0;
    let mut visible_line_top: i32 = 0;
    let mut visible_line_bottom: i32 = 0;

    for i in (0..x_coord).rev() {
        if map[i][y_coord].tree_height >= point.tree_height {
            visible_line_top += 1;
            break;
        };
        visible_line_top += 1;
    }
    for j in x_coord..map[x_coord].len() - 1 {
        if map[j + 1][y_coord].tree_height >= point.tree_height {
            visible_line_bottom += 1;
            break;
        };
        visible_line_bottom += 1;
    }

    for k in (0..y_coord).rev() {
        if map[x_coord][k].tree_height >= point.tree_height {
            visible_line_left += 1;
            break;
        };
        visible_line_left += 1;
    }

    for l in y_coord..map[x_coord].len() - 1 {
        if map[x_coord][l + 1].tree_height >= point.tree_height {
            visible_line_right += 1;
            break;
        };
        visible_line_right += 1;
    }

    return visible_line_left * visible_line_right * visible_line_top * visible_line_bottom;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
