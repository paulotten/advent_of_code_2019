mod data;

fn main() {
    let entries = data::get_entries();

    puzzle2(entries);
}

fn _puzzle1(entries: Vec<u32>) {
    let mut total_fuel_cost = 0;

    for entry in entries {
        total_fuel_cost += get_fuel_cost(entry);
    }

    println!("{}", total_fuel_cost);
}

fn puzzle2(entries: Vec<u32>) {
    let mut total_fuel_cost = 0;

    for entry in entries {
        let mut mass = entry;

        loop {
            let fuel = get_fuel_cost(mass);
            total_fuel_cost += fuel;

            if fuel > 0 {
                mass = fuel;
            } else {
                break;
            }
        }
    }

    println!("{}", total_fuel_cost);
}

fn get_fuel_cost(mass: u32) -> u32 {
    let fuel: i32 = mass as i32/3 - 2;

    if fuel < 0 {
        return 0;
    }

    fuel as u32
}
