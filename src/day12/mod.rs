mod day11 {}

use super::util::read_lines;

pub fn main() {
    if let Ok(lines) = read_lines("src/day12/input.txt") {
        let mut points: Vec<Point> = vec![];
        let mut idx_counter = 0;

        for (x, line) in lines.enumerate() {
            if let Ok(l) = line {
                l.chars().enumerate().for_each(|(y, letter)| {
                    points.push(Point::new(idx_counter, x, y, letter));
                    idx_counter += 1
                })
            }
        }

        for i in 0..points.len() {
            points[i].neighbours = find_neighbours(&points[i], points.clone());
        }

        let mut current_point = points.iter().find(|x| x.elevation == 27).unwrap().idx;
        let mut unvisited_points = vec![points[current_point].clone()];

        while unvisited_points.len() > 0 {
            if points[current_point].elevation == 1 {
                break;
            }

            if points[current_point].visited {
                unvisited_points.retain(|p| p.idx != current_point);
                current_point = unvisited_points.first().unwrap().idx;
                continue;
            }

            let current_distance = points[current_point].distance;
            let mut neighbours = points[current_point].neighbours.clone();

            neighbours.iter_mut().for_each(|neighbour_idx| {
                let idx = *neighbour_idx;
                if points[idx].distance > (current_distance + 1) {
                    points[idx].distance = current_distance + 1;
                }

                unvisited_points.push(points[idx].clone());
            });
            points[current_point].visited = true;
            unvisited_points.sort_by(|x, y| x.distance.cmp(&y.distance));
            current_point = unvisited_points.first().unwrap().idx;
        }
        let end_point = points
            .iter()
            .filter(|x| x.elevation == 1)
            .map(|x| x.distance)
            .min()
            .unwrap();
        println!("Day 12 result: distance {:?}", end_point);
    }
}

fn find_neighbours(point: &Point, points: Vec<Point>) -> Vec<usize> {
    points
        .iter()
        .filter(|possible_neighbour| {
            ((possible_neighbour.x_coord.abs_diff(point.x_coord) == 1
                && possible_neighbour.y_coord == point.y_coord)
                || (possible_neighbour.y_coord.abs_diff(point.y_coord) == 1
                    && possible_neighbour.x_coord == point.x_coord))
        })
        .filter(|possible_neighbour| point.elevation - possible_neighbour.elevation <= 1)
        .map(|x| x.idx)
        .collect::<Vec<usize>>()
}

#[derive(Debug, Clone)]
struct Point {
    idx: usize,
    x_coord: usize,
    y_coord: usize,
    elevation: i32,
    distance: i32,
    visited: bool,
    neighbours: Vec<usize>,
}

impl Point {
    fn new(idx: usize, x_coord: usize, y_coord: usize, elevation: char) -> Point {
        Point {
            idx,
            x_coord,
            y_coord,
            elevation: Point::elevation_from_char(elevation),
            distance: Point::get_distance_from_char(elevation),
            visited: Point::new_visited(elevation),
            neighbours: vec![],
        }
    }

    fn new_visited(letter: char) -> bool {
        return if letter == 'E' { false } else { false };
    }

    fn get_distance_from_char(letter: char) -> i32 {
        return if letter == 'E' { 0 } else { i32::MAX };
    }

    fn elevation_from_char(letter: char) -> i32 {
        return match letter {
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
            'S' => 0,
            'E' => 27,
            _ => panic!["Elevation not possible: {}", letter],
        };
    }
}
