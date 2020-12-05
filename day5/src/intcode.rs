/*
An intcode computer as described by https://adventofcode.com/2019/day/2
Updated for https://adventofcode.com/2019/day/5
*/

const ADD: usize = 1;
const MULT: usize = 2;
const INPUT: usize = 3;
const OUTPUT: usize = 4;
const HALT: usize = 99;

pub fn run(intcodes: &mut Vec<usize>) {
    let mut position = 0;

    loop {
        let intcode = intcodes[position];

        match intcode {
            ADD => {
                let a = intcodes[intcodes[position+1]];
                let b = intcodes[intcodes[position+2]];

                let output_position = intcodes[position+3];
                intcodes[output_position] = a + b;

                position += 4;
            },
            MULT => {
                let a = intcodes[intcodes[position+1]];
                let b = intcodes[intcodes[position+2]];

                let output_position = intcodes[position+3];
                intcodes[output_position] = a * b;

                position += 4;
            },
            INPUT => {
                let intput: usize;
                let mut input: String;

                loop {
                    println!("Enter an integer: ");

                    input = String::new();
                    if std::io::stdin().read_line(&mut input).is_ok() {
                        input = input.trim().to_string();
                        let maybe_int: Result<usize, _> = input.parse();

                        if maybe_int.is_ok() {
                            intput = maybe_int.unwrap();
                            break;
                        }
                    }
                }

                let output_position = intcodes[position+1];
                intcodes[output_position] = intput;

                position += 2;
            },
            OUTPUT => {
                let a = intcodes[intcodes[position+1]];

                println!("{}", a);

                position += 2;
            },
            HALT => break,
            _ => panic!("unknown intcode {}", intcode),
        };
    }
}
