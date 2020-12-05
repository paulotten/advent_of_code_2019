/*
An intcode computer as described by https://adventofcode.com/2019/day/2
Updated for https://adventofcode.com/2019/day/5
*/

pub type Int = i64;

const ADD: Int = 1;
const MULT: Int = 2;
const INPUT: Int = 3;
const OUTPUT: Int = 4;
const JUMP_TRUE: Int = 5;
const JUMP_FALSE: Int = 6;
const LESS_THAN: Int = 7;
const EQUAL: Int = 8;

const HALT: Int = 99;

pub fn run(intcodes: &mut [Int]) {
    let mut position: Int = 0;

    loop {
        let mode_and_code = intcodes[position as usize];
        let intcode = mode_and_code % 100;

        let mut modes: Vec<u8> = vec!();
        let mut i = 100;
        while i <= 10_000 {
            modes.push((mode_and_code / i % 10) as u8);

            i *= 10;
        }

        match intcode {
            ADD => {
                let a = get_param(intcodes, position+1, modes[0]);
                let b = get_param(intcodes, position+2, modes[1]);

                let output_position = intcodes[position as usize + 3];
                intcodes[output_position as usize] = a + b;

                position += 4;
            },
            MULT => {
                let a = get_param(intcodes, position+1, modes[0]);
                let b = get_param(intcodes, position+2, modes[1]);

                let output_position = intcodes[position as usize + 3];
                intcodes[output_position as usize] = a * b;

                position += 4;
            },
            INPUT => {
                let intput: Int;
                let mut input: String;

                loop {
                    println!("Enter an integer: ");

                    input = String::new();
                    if std::io::stdin().read_line(&mut input).is_ok() {
                        input = input.trim().to_string();
                        let maybe_int: Result<Int, _> = input.parse();

                        if maybe_int.is_ok() {
                            intput = maybe_int.unwrap();
                            break;
                        }
                    }
                }

                let output_position = intcodes[position as usize + 1];
                intcodes[output_position as usize] = intput;

                position += 2;
            },
            OUTPUT => {
                let a = get_param(intcodes, position+1, modes[0]);

                println!("{}", a);

                position += 2;
            },
            JUMP_TRUE => {
                let a = get_param(intcodes, position+1, modes[0]);
                let b = get_param(intcodes, position+2, modes[1]);

                if a > 0 {
                    position = b;
                } else {
                    position += 3;
                }
            },
            JUMP_FALSE => {
                let a = get_param(intcodes, position+1, modes[0]);
                let b = get_param(intcodes, position+2, modes[1]);

                if a == 0 {
                    position = b;
                } else {
                    position += 3;
                }
            },
            LESS_THAN => {
                let a = get_param(intcodes, position+1, modes[0]);
                let b = get_param(intcodes, position+2, modes[1]);

                let output_position = intcodes[position as usize + 3];
                intcodes[output_position as usize] = match a < b {
                    true => 1,
                    false => 0,
                };

                position += 4;
            },
            EQUAL => {
                let a = get_param(intcodes, position+1, modes[0]);
                let b = get_param(intcodes, position+2, modes[1]);

                let output_position = intcodes[position as usize + 3];
                intcodes[output_position as usize] = match a == b {
                    true => 1,
                    false => 0,
                };

                position += 4;
            },
            HALT => break,
            _ => panic!("unknown intcode {}", intcode),
        };
    }
}

fn get_param(intcodes: &mut [Int], position: Int, mode: u8) -> Int {
    match mode {
        0 => intcodes[intcodes[position as usize] as usize],
        1 => intcodes[position as usize],
        _ => panic!("unknown mode"),
    }
}
