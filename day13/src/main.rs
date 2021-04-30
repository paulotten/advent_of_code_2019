mod data;
mod intcode;

use intcode::Int;
use itertools::Itertools;
use std::collections::HashMap;

const BLOCK: Int = 2;
const PADDLE: Int = 3;
const BALL: Int = 4;

fn main() {
    //part1();
    part2();
}

fn part1() {
    let data = data::get_data();
    let intcodes = get_intcodes(data);

    let mut computer = intcode::Computer::new();
    computer.set_intcodes(&intcodes);
    computer.set_print_output(false);

    let mut output: Vec<Int> = vec!();

    loop {
        match computer.run() {
            None => break,
            Some(o) => output.push(o),
        }
    }

    let tiles = get_tiles(&output);

    println!("{}", get_block_count(&tiles));
}

fn part2() {
    let data = data::get_data();
    let mut intcodes = get_intcodes(data);

    // Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
    intcodes[0] = 2;

    let mut computer = intcode::Computer::new();
    computer.set_intcodes(&intcodes);
    computer.set_print_output(false);

    let mut output: Vec<Int> = vec!();

    let mut tiles: HashMap<(Int, Int), Int> = HashMap::new();

    loop {
        match computer.run() {
            None => break,
            Some(o) => {
                output.push(o);

                if output.len() == 3 {
                    tiles.insert((output[0], output[1]), output[2]);
                    output = vec!();

                    let paddle_x = get_x_pos(&tiles, PADDLE);
                    let ball_x = get_x_pos(&tiles, BALL);

                    if paddle_x.is_some() && ball_x.is_some() {
                        let paddle_x = paddle_x.unwrap();
                        let ball_x = ball_x.unwrap();

                        computer.clear_input();

                        // AI: moves the paddle to under the ball
                        if paddle_x < ball_x {
                            computer.add_input(1);
                        } else if paddle_x > ball_x {
                            computer.add_input(-1);
                        } else {
                            computer.add_input(0);
                        }

                        // comment these two lines out if you don't want to watch the AI play
                        //print_screen(&tiles);
                        //std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
            },
        }
    }

    println!("score: {}, blocks: {}", get_score(&tiles), get_block_count(&tiles));
}

fn get_intcodes(data: &'static str) -> Vec<Int> {
    data.split(',').map(|i| i.parse::<Int>().unwrap()).collect()
}

fn get_tiles(output: &Vec<Int>) -> HashMap<(Int, Int), Int> {
    let mut tiles: HashMap<(Int, Int), Int> = HashMap::new();

    for (x, y, t) in output.iter().tuples() {
        tiles.insert((*x, *y), *t);
    }

    tiles
}

fn get_block_count(tiles: &HashMap<(Int, Int), Int>) -> usize {
    tiles.iter().filter(|(_,v)| **v == BLOCK).count()
}

fn get_score(tiles: &HashMap<(Int, Int), Int>) -> Int {
    *tiles.get(&(-1, 0)).unwrap_or(&0)
}

fn print_screen(tiles: &HashMap<(Int, Int), Int>) {
    println!("screen:");

    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in tiles.keys() {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let tile: String = match tiles.get(&(x, y)) {
                None => ".".to_string(),
                Some(0) => ".".to_string(),
                Some(x) => x.to_string(),
            };
            print!("{}", tile);
        }

        println!();
    }
}

fn get_x_pos(tiles: &HashMap<(Int, Int), Int>, value: Int) -> Option<Int> {
    for ((x, y), tile) in tiles {
        if *tile == value {
            return Some(*x);
        }
    }

    None
}