mod intcode;
mod data;

fn main() {
    let inputs = get_inputs();

    let intcodes = data::get_intcodes();
    //let intcodes = data::_sample();

    puzzle1(inputs, intcodes);
}

fn puzzle1(inputs: Vec<Vec<intcode::Int>>, intcodes: Vec<intcode::Int>) {
    let mut max = 0;

    for inputs in inputs {
        let mut last_output = 0;

        for i in 0..5 {
            let mut computer = intcode::Computer::new();

            computer.set_intcodes(intcodes.clone());
            computer.set_inputs(vec!(inputs[i], last_output));
            computer.set_print_output(false);

            let output = computer.run();
            assert!(output.len() == 1);
            last_output = output[0];
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
