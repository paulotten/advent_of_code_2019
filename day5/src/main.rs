use day5::intcode;

fn main() {
    let mut intcodes = day5::data::get_intcodes();

    puzzle1(&mut intcodes);
}

fn _echo_test() {
    let mut intcodes = [3,0,4,0,99];

    intcode::run(&mut intcodes);
}

fn puzzle1(intcodes: &mut [intcode::Int]) {
    println!("(enter 1)");
    intcode::run(intcodes)
}
