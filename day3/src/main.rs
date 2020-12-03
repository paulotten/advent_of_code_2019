mod data;

use data::{Path, NUM_WIRES};
use std::collections::HashMap;

#[derive (PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let paths = data::get_paths();

    puzzle2(paths);
}

fn _puzzle1(paths: [Vec<Path>; NUM_WIRES]) {
    let wire1 = expand_paths(&paths[0]);
    let wire2 = expand_paths(&paths[1]);

    let mut min: Option<i32> = None;

    for point in wire1.keys() {
        if wire2.contains_key(point) {
            let distance = _get_distance1(point);

            if min.is_none() || min.unwrap() > distance {
                min = Some(distance);
            }
        }
    }

    if min.is_some() {
        println!("{}", min.unwrap());
    } else {
        println!("No value found");
    }
}

fn _get_distance1(point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}

fn puzzle2(paths: [Vec<Path>; NUM_WIRES]) {
    let wire1 = expand_paths(&paths[0]);
    let wire2 = expand_paths(&paths[1]);

    let mut min: Option<i32> = None;

    for point in wire1.keys() {
        if wire2.contains_key(point) {
            let distance = wire1[point] + wire2[point];

            if min.is_none() || min.unwrap() > distance {
                min = Some(distance);
            }
        }
    }

    if min.is_some() {
        println!("{}", min.unwrap());
    } else {
        println!("No value found");
    }
}

fn expand_paths(paths: &Vec<Path>) -> HashMap<Point, i32> {
    let mut points: HashMap<Point, i32> = HashMap::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut wire_length = 0;

    for path in paths {
        for _ in 0..path.length {
            wire_length += 1;

            match path.direction {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => panic!(),
            };

            points.entry(Point { x, y }).or_insert(wire_length);
        }
    }

    points
}
