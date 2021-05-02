mod data;
mod intcode;

use intcode::Int;

fn main() {
    part1();
}

fn part1() {
    let data = data::get_data();
    let intcodes = get_intcodes(data);
    let mut computer = intcode::Computer::new();

    computer.set_intcodes(&intcodes);
    while computer.run().is_some() {};

    // see part1.xlsx and part2.xlsx
}

fn get_intcodes(data: &str) -> Vec<Int>{
    data.split(",").map(|x| x.parse::<Int>().unwrap()).collect()
}
