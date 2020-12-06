mod data;

use std::cmp;
use std::collections::HashSet;

type Point = (usize, usize);

fn main() {
    let data = data::get_data();
    let astroids = get_astroids(data);

    puzzle1(astroids);
}

fn puzzle1(astroids: HashSet<Point>) {
    let mut max = 0;

    for a in &astroids {
        let mut count = 0;

        for b in &astroids {
            if can_see(*a, *b, &astroids) {
                count += 1;
            }
        }

        if count > max {
            max = count;
        }
    }

    println!("{}", max);
}

fn can_see(a: Point, b: Point, astroids: &HashSet<Point>) -> bool {
    let mut a = a;
    let mut b = b;

    // same point, don't count
    if a.0 == b.0 && a.1 == b.1 {
        return false;
    }

    // straight line vertical line
    if a.0 == b.0 {
        // move vertically checking each possible position
        for y in cmp::min(a.1, b.1) + 1..cmp::max(a.1, b.1) {
            if astroids.contains(&(a.0, y)) {
                return false;
            }
        }
    } else {
        // move horizontally...
        let mut x_diff: isize = b.0 as isize - a.0 as isize;

        if x_diff < 0 {
            let c = a;
            a = b;
            b = c;

            x_diff = b.0 as isize - a.0 as isize;
        }

        let y_diff: isize = b.1 as isize - a.1 as isize;

        for x_offest in 1..x_diff {
            // figure out our corresponding vertical position
            let mult = y_diff * x_offest;

            if mult % x_diff != 0 {
                // fractional y positions are always a miss
                continue;
            }

            let y_offest = mult / x_diff;

            if astroids.contains(&(
                (a.0 as isize + x_offest) as usize,
                (a.1 as isize + y_offest) as usize,
            )) {
                return false;
            }
        }
    }

    true
}

fn get_astroids(data: &str) -> HashSet<Point> {
    let mut astroids = HashSet::new();

    let mut y = 0;
    for line in data.lines() {
        let chars: Vec<char> = line.chars().collect();

        for x in 0..chars.len() {
            if chars[x] == '#' {
                astroids.insert((x, y));
            }
        }

        y += 1;
    }

    astroids
}
