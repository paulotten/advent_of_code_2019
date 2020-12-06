mod intcode;
mod data;

fn main() {
    let intcodes = data::get_intcodes();
    //let intcodes = data::_sample();

    puzzle2(intcodes);
}

fn _puzzle1(intcodes: Vec<intcode::Int>) {
    let mut computer = intcode::Computer::new();

    computer.set_intcodes(&intcodes);
    computer.add_input(1);

    loop{
        if computer.run().is_none() {
            break;
        }
    }
}

fn puzzle2(intcodes: Vec<intcode::Int>) {
    let mut computer = intcode::Computer::new();

    computer.set_intcodes(&intcodes);
    computer.add_input(2);

    loop{
        if computer.run().is_none() {
            break;
        }
    }
}
