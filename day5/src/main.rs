mod intcode;

fn main() {
    echo_test();
}

fn echo_test() {
    let mut intcodes = vec!(3,0,4,0,99);

    intcode::run(&mut intcodes);
}