/*
An intcode computer as described by https://adventofcode.com/2019/day/2
*/

const ADD: usize = 1;
const MULT: usize = 2;
const HALT: usize = 99;

pub fn run(intcodes: &mut Vec<usize>) {
    let mut position = 0;

    loop {
        let intcode = intcodes[position];

        if intcode == HALT {
            break;
        } else {
            let a = intcodes[intcodes[position+1]];
            let b = intcodes[intcodes[position+2]];

            let output_position = intcodes[position+3];

            if intcode == ADD {
                intcodes[output_position] = a + b;
            }
            if intcode == MULT {
                intcodes[output_position] = a * b;
            }

            position += 4;
        }
    }
}
