mod data;

use std::cmp;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::f64::consts::PI;

type Point = (isize, isize);

fn main() {
    let data = data::get_data();
    //let data = data::_sample();
    let astroids = get_astroids(data);

    _puzzle2(astroids);
}

const F2I: f64 = 1_000_000_000.0;

fn _puzzle2(astroids: HashSet<Point>) {
    let station: Point = (37, 25); // from puzzle 1
    //let station: Point = (11,13); // sample data
    let mut a_by_a: BTreeMap<i64, Point> = BTreeMap::new(); // astroids by angle

    // from puzzle 1 we know we can see 309 astroids from this point
    // we need to find the 200th, so we only need this first scan
    for a in &astroids {
        if can_see(station, *a, &astroids) {
            let rel_x = a.0 - station.0;
            let rel_y = a.1 - station.1;

            // calculate arc angle
            let mut angle = f64::atan2(
                rel_x as f64,
                -rel_y as f64,
            );

            if angle < 0.0 {
                angle += 2.0*PI;
            }

            // can't hash a float, covert it to an int with 9 of it's decimal places
            a_by_a.insert((angle * F2I) as i64, *a);
        }
    }

    // find the 200th astroid by arc angle
    let mut a_iter = a_by_a.iter();
    for i in 1..=200 {
        let a = a_iter.next().unwrap();

        println!("{}: {},{}", i, a.1.0, a.1.1);
    }
}

fn _puzzle1(astroids: HashSet<Point>) {
    let mut max = 0;
    let mut max_point: Point = (0, 0);

    for a in &astroids {
        let mut count = 0;

        for b in &astroids {
            if can_see(*a, *b, &astroids) {
                count += 1;
            }
        }

        if count > max {
            max = count;
            max_point = *a;
        }
    }

    println!("{} at {},{}, {}", max, max_point.0, max_point.1, max_point.0*100+max_point.1); // 309 at 37,25
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

            if astroids.contains(&(a.0 + x_offest, a.1 + y_offest)) {
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
                astroids.insert((x as isize, y as isize));
            }
        }

        y += 1;
    }

    astroids
}
