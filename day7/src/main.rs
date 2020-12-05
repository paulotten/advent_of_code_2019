mod intcode;
mod data;

fn main() {
    let inputs = get_inputs();

    let intcodes = data::get_intcodes();
    //let intcodes = data::_sample();

    puzzle2(inputs, intcodes);
}

fn _puzzle1(inputs: Vec<Vec<intcode::Int>>, intcodes: Vec<intcode::Int>) {
    let mut max = 0;

    for inputs in inputs {
        let mut last_output = 0;

        for i in 0..5 {
            let mut computer = intcode::Computer::new();

            computer.set_intcodes(intcodes.clone());
            computer.add_input(inputs[i]);
            computer.add_input(last_output);
            computer.set_print_output(false);

            let output = computer.run();
            assert!(output.is_some());
            last_output = output.unwrap();
        }

        if last_output > max {
            max = last_output;
        }
    }

    println!("{}", max);
}

fn puzzle2(inputs: Vec<Vec<intcode::Int>>, intcodes: Vec<intcode::Int>) {
    let mut max = 0;

    for inputs in inputs {
        let mut last_output = 0;
        let mut computers: Vec<intcode::Computer> = vec!();

        for i in 0..5 {
            let mut computer = intcode::Computer::new();
            computer.set_intcodes(intcodes.clone());
            computer.add_input(inputs[i]+5);

            computers.push(computer);
        }

        let mut i = 0;
        loop {
            computers[i].add_input(last_output);
            computers[i].set_print_output(false);

            let output = computers[i].run();

            if output.is_none() {
                break;
            }

            last_output = output.unwrap();

            i = (i + 1) % 5;
        }

        if last_output > max {
            max = last_output;
        }
    }

    println!("{}", max);
}

fn get_inputs() -> Vec<Vec<intcode::Int>> {
    let mut inputs: Vec<Vec<intcode::Int>> = vec!();

    for i in 0..5 {
        for j in 0..5 {
            if i == j {
                continue;
            }

            for k in 0..5 {
                if i == k || j == k {
                    continue;
                }

                for l in 0..5 {
                    if i == l || j == l || k == l {
                        continue;
                    }

                    for m in 0..5 {
                        if i == m || j == m || k == m || l == m {
                            continue;
                        }

                        inputs.push(vec![i, j, k, l, m]);
                    }
                }
            }
        }
    }

    inputs
}
