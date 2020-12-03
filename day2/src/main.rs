mod data;
mod intcode;

fn main() {
    let mut intcodes = data::get_intcodes();

    puzzle2(&mut intcodes);
}

fn _puzzle1(intcodes: &mut Vec<usize>) {
    // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2
    intcodes[1] = 12;
    intcodes[2] = 2;

    intcode::run(intcodes);

    // What value is left at position 0 after the program halts?
    println!("{}", intcodes[0]);
}

fn puzzle2(original_intcodes: &mut Vec<usize>) {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut intcodes = original_intcodes.clone();

            intcodes[1] = noun;
            intcodes[2] = verb;

            intcode::run(&mut intcodes);

            if intcodes[0] == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}
