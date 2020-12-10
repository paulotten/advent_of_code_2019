mod data;
mod intcode;

use std::collections::HashSet;
use crate::intcode::Int;

fn main() {
    let intcodes = data::get_intcodes();

    _puzzle2(intcodes);
}

fn _puzzle2(intcodes: Vec<Int>) {
    const SIDE_LENGTH: usize = 100;

    let mut canvas: [[u8; SIDE_LENGTH]; SIDE_LENGTH] = [[0; SIDE_LENGTH]; SIDE_LENGTH];

    // robot position and heading
    let mut x: isize = SIDE_LENGTH as isize / 2;
    let mut y: isize = SIDE_LENGTH as isize / 2;
    let mut x_vel: isize = 0;
    let mut y_vel: isize = 1;

    // starting the robot on a single white panel
    canvas[x as usize][y as usize] = 1;

    let mut computer = intcode::Computer::new();
    computer.set_intcodes(&intcodes);
    computer._set_print_output(false);

    loop {
        // input current color
        let input = canvas[x as usize][y as usize];
        computer.add_input(input as Int);

        // color current panel
        let color = computer.run();

        if color.is_none() {
            break;
        }

        canvas[x as usize][y as usize] = color.unwrap() as u8;

        // turn
        let turn = computer.run();

        if turn.is_none() {
            break;
        }

        match turn.unwrap() {
            0 => {
                // left
                match (x_vel, y_vel) {
                    (0, 1) => {
                        x_vel = -1;
                        y_vel = 0;
                    },
                    (-1, 0) => {
                        x_vel = 0;
                        y_vel = -1;
                    },
                    (0, -1) => {
                        x_vel = 1;
                        y_vel = 0;
                    },
                    (1, 0) => {
                        x_vel = 0;
                        y_vel = 1;
                    },
                    _ => panic!(),
                };
            },
            1 => {
                // right
                match (x_vel, y_vel) {
                    (0, 1) => {
                        x_vel = 1;
                        y_vel = 0;
                    },
                    (1, 0) => {
                        x_vel = 0;
                        y_vel = -1;
                    },
                    (0, -1) => {
                        x_vel = -1;
                        y_vel = 0;
                    },
                    (-1, 0) => {
                        x_vel = 0;
                        y_vel = 1;
                    },
                    _ => panic!(),
                };
            },
            dir @ _ => {
                panic!("unknown direction: {}", dir);
            }
        };

        // move
        x += x_vel;
        y += y_vel;
    }

    // print our masterpiece
    for i in 0..SIDE_LENGTH {
        for j in 0..SIDE_LENGTH {
            let c = match canvas[i][j] {
                1 => '#',
                _ => ' ',
            };

            print!("{}", c);
        }

        println!();
    }
}

fn _puzzle1(intcodes: Vec<Int>) {
    const SIDE_LENGTH: usize = 200;

    let mut canvas: [[u8; SIDE_LENGTH]; SIDE_LENGTH] = [[0; SIDE_LENGTH]; SIDE_LENGTH];
    let mut painted_coords: HashSet<(isize, isize)> = HashSet::new();

    // robot position and heading
    let mut x: isize = SIDE_LENGTH as isize / 2;
    let mut y: isize = SIDE_LENGTH as isize / 2;
    let mut x_vel: isize = 0;
    let mut y_vel: isize = 1;

    let mut computer = intcode::Computer::new();
    computer.set_intcodes(&intcodes);
    computer._set_print_output(false);

    loop {
        // input current color
        let input = canvas[x as usize][y as usize];
        computer.add_input(input as Int);

        // color current panel
        let color = computer.run();

        if color.is_none() {
            break;
        }

        canvas[x as usize][y as usize] = color.unwrap() as u8;
        painted_coords.insert((x, y));

        // turn
        let turn = computer.run();

        if turn.is_none() {
            break;
        }

        match turn.unwrap() {
            0 => {
                // left
                match (x_vel, y_vel) {
                    (0, 1) => {
                        x_vel = -1;
                        y_vel = 0;
                    },
                    (-1, 0) => {
                        x_vel = 0;
                        y_vel = -1;
                    },
                    (0, -1) => {
                        x_vel = 1;
                        y_vel = 0;
                    },
                    (1, 0) => {
                        x_vel = 0;
                        y_vel = 1;
                    },
                    _ => panic!(),
                };
            },
            1 => {
                // right
                match (x_vel, y_vel) {
                    (0, 1) => {
                        x_vel = 1;
                        y_vel = 0;
                    },
                    (1, 0) => {
                        x_vel = 0;
                        y_vel = -1;
                    },
                    (0, -1) => {
                        x_vel = -1;
                        y_vel = 0;
                    },
                    (-1, 0) => {
                        x_vel = 0;
                        y_vel = 1;
                    },
                    _ => panic!(),
                };
            },
            dir @ _ => {
                panic!("unknown direction: {}", dir);
            }
        };

        // move
        x += x_vel;
        y += y_vel;
    }

    println!("{}", painted_coords.len());
}
