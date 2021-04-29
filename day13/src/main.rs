mod data;
mod intcode;

use intcode::Int;
use itertools::Itertools;
use std::collections::HashMap;

const BLOCK: Int = 2;

fn main() {
    part1();
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

    let block_count = tiles.iter().filter(|(_,v)| **v == BLOCK).count();
    println!("{}", block_count);
}

fn get_intcodes(data: &'static str) -> Vec<Int> {
    data.split(',').map(|i| i.parse::<Int>().unwrap()).collect()
}

fn get_tiles(screen: &Vec<Int>) -> HashMap<(Int, Int), Int> {
    let mut tiles: HashMap<(Int, Int), Int> = HashMap::new();

    for (x, y, t) in screen.iter().tuples() {
        if *t != 0 {
            tiles.insert((*x, *y), *t);
        }
    }

    tiles
}
