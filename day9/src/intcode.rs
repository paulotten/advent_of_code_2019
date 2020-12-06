/*
An intcode computer as described by https://adventofcode.com/2019/day/2
updated for https://adventofcode.com/2019/day/5
and https://adventofcode.com/2019/day/7
and https://adventofcode.com/2019/day/9
*/

pub type Int = i64;

const MEM_SIZE: usize = 4000;

pub struct Computer {
    intcodes: [Int; MEM_SIZE],
    input_ints: Vec<Int>,
    inputs_read: usize,
    print_output: bool,
    position: Int,
    rel_pos: Int,
}

const ADD: Int = 1;
const MULT: Int = 2;
const INPUT: Int = 3;
const OUTPUT: Int = 4;
const JUMP_TRUE: Int = 5;
const JUMP_FALSE: Int = 6;
const LESS_THAN: Int = 7;
const EQUAL: Int = 8;
const ADJ_REL: Int = 9;

const HALT: Int = 99;

impl Computer {
    pub fn new() -> Computer {
        Computer {
            intcodes: [0; MEM_SIZE],
            input_ints: vec!(),
            inputs_read: 0,
            print_output: true,
            position: 0,
            rel_pos: 0,
        }
    }

    pub fn set_intcodes(&mut self, intcodes: &[Int]) {
        for i in 0..intcodes.len() {
            self.intcodes[i] = intcodes[i];
        }
    }

    pub fn add_input(&mut self, input: Int) {
        self.input_ints.push(input);
    }

    pub fn _set_print_output(&mut self, print_output: bool) {
        self.print_output = print_output;
    }

    pub fn run(&mut self) -> Option<Int> {
        loop {
            let mode_and_code = self.intcodes[self.position as usize];
            let intcode = mode_and_code % 100;
    
            let mut modes: Vec<u8> = vec!();
            let mut i = 100;
            while i <= 10_000 {
                modes.push((mode_and_code / i % 10) as u8);
    
                i *= 10;
            }
    
            match intcode {
                ADD => {
                    let a = self.get_param(self.position+1, modes[0]);
                    let b = self.get_param(self.position+2, modes[1]);
    
                    let output_position = self.intcodes[self.position as usize + 3];
                    self.write_output(output_position, modes[2], a + b);
    
                    self.position += 4;
                },
                MULT => {
                    let a = self.get_param(self.position+1, modes[0]);
                    let b = self.get_param(self.position+2, modes[1]);
    
                    let output_position = self.intcodes[self.position as usize + 3];
                    self.write_output(output_position, modes[2], a * b);
    
                    self.position += 4;
                },
                INPUT => {
                    let input_int: Int;

                    if self.inputs_read >= self.input_ints.len() {
                        let mut input: String;
        
                        loop {
                            println!("Enter an integer: ");
        
                            input = String::new();
                            if std::io::stdin().read_line(&mut input).is_ok() {
                                input = input.trim().to_string();
                                let maybe_int: Result<Int, _> = input.parse();
        
                                if maybe_int.is_ok() {
                                    input_int = maybe_int.unwrap();
                                    break;
                                }
                            }
                        }
                    } else {
                        input_int = self.input_ints[self.inputs_read];
                    }
                    self.inputs_read += 1;

                    let output_position = self.intcodes[self.position as usize + 1];
                    self.write_output(output_position, modes[0], input_int);
    
                    self.position += 2;
                },
                OUTPUT => {
                    let a = self.get_param(self.position+1, modes[0]);
    
                    if self.print_output {
                        println!("{}", a);
                    }
    
                    self.position += 2;

                    return Some(a);
                },
                JUMP_TRUE => {
                    let a = self.get_param(self.position+1, modes[0]);
                    let b = self.get_param(self.position+2, modes[1]);
    
                    if a > 0 {
                        self.position = b;
                    } else {
                        self.position += 3;
                    }
                },
                JUMP_FALSE => {
                    let a = self.get_param(self.position+1, modes[0]);
                    let b = self.get_param(self.position+2, modes[1]);
    
                    if a == 0 {
                        self.position = b;
                    } else {
                        self.position += 3;
                    }
                },
                LESS_THAN => {
                    let a = self.get_param(self.position+1, modes[0]);
                    let b = self.get_param(self.position+2, modes[1]);
    
                    let output_position = self.intcodes[self.position as usize + 3];
                    let output = match a < b {
                        true => 1,
                        false => 0,
                    };
                    self.write_output(output_position, modes[2], output);
    
                    self.position += 4;
                },
                EQUAL => {
                    let a = self.get_param(self.position+1, modes[0]);
                    let b = self.get_param(self.position+2, modes[1]);
    
                    let output_position = self.intcodes[self.position as usize + 3];
                    let output = match a == b {
                        true => 1,
                        false => 0,
                    };
                    self.write_output(output_position, modes[2], output);
    
                    self.position += 4;
                },
                ADJ_REL => {
                    let a = self.get_param(self.position+1, modes[0]);

                    self.rel_pos += a;

                    self.position += 2;
                }
                HALT => break,
                _ => panic!("unknown intcode {}", intcode),
            };
        }

        None
    }
    
    fn get_param(&self, position: Int, mode: u8) -> Int {
        match mode {
            0 => self.intcodes[self.intcodes[position as usize] as usize],
            1 => self.intcodes[position as usize],
            2 => self.intcodes[(self.rel_pos + self.intcodes[position as usize]) as usize],
            _ => panic!("unknown read mode {}", mode),
        }
    }

    fn write_output(&mut self, position: Int, mode: u8, value: Int) {
        match mode {
            0 => self.intcodes[position as usize] = value,
            2 => self.intcodes[(self.rel_pos + position) as usize] = value,
            _ => panic!("unknown write mode {}", mode),
        };
    }
}
