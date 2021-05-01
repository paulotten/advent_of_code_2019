mod data;

use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
    output: Component,
    inputs: Vec<Component>,
}

#[derive(Debug)]
struct Component {
    chemical: &'static str,
    quantity: u64,
}

impl Component {
    fn parse(input: &'static str) -> Component {
        let parts: Vec<_> = input.split(" ").collect();
        assert!(parts.len() == 2);

        let chemical = parts[1];
        let quantity = parts[0].parse::<u64>().unwrap();

        Component{
            chemical,
            quantity,
        }
    }
}

fn main() {
    //part1();
    part2();
}

fn part1() {
    // result 31
    //let data = data::_get_sample1();

    // result 165
    //let data = data::_get_sample2();

    // result 13312
    //let data = data::_get_sample3();

    // result 1582325
    let data = data::get_data();

    let reactions = get_reactions(data);
    //println!("{:#?}", &reactions);

    let mut requirements: HashMap<&str, u64> = HashMap::new();
    let mut excess: HashMap<&str, u64> = HashMap::new();

    // start by requiring one FUEL
    requirements.insert("FUEL", 1);

    loop {
        if requirements.len() == 1 && requirements.contains_key("ORE") {
            break;
        }

        // these initial values will never get used due to the `break;` above
        let mut target_chemical = "";
        let mut target_quantity = 0;

        // pick a chemical other than ORE
        for (chemical, quantity) in &requirements {
            if *chemical == "ORE" {
                continue;
            }

            target_chemical = chemical;
            target_quantity = *quantity;

            break;
        }

        // use any excess chemical we have left over
        let mut created_quantity = *excess.get(target_chemical).unwrap_or(&0);

        // then find the reaction to create more chemical
        let reaction = reactions.get(target_chemical).unwrap();

        // and create more until we have enough
        while created_quantity < target_quantity {
            created_quantity += reaction.output.quantity;

            // and the inputs as further requirements
            for input in &reaction.inputs {
                *requirements.entry(input.chemical).or_insert(0) += input.quantity;
            }
        }

        // remove the now satisified required chemical
        requirements.remove(target_chemical);

        // record any chemical excess
        excess.insert(target_chemical, created_quantity-target_quantity);

        println!("{:?}", &requirements);
    }

    // print our final ORE requirement
    println!("{} ORE", requirements.get("ORE").unwrap());
}

// wrong, > 631981
// this is too low because we have left over reagents after making one fuel 
/*
fn part2() {
    println!("{}", 1_000_000_000_000u64 / 1582325u64);
}
*/

fn part2() {
    // result 82892753
    //let data = data::_get_sample3();

    // result 2267486
    let data = data::get_data();

    let reactions = get_reactions(data);

    let mut requirements: HashMap<&str, u64> = HashMap::new();
    let mut excess: HashMap<&str, u64> = HashMap::new();

    requirements.insert("ORE", 0);

    let mut fuel_created: u64 = 0;

    while *requirements.get("ORE").unwrap() < 1_000_000_000_000u64 {
        requirements.insert("FUEL", 1);

        loop {
            if requirements.len() == 1 && requirements.contains_key("ORE") {
                break;
            }

            // these initial values will never get used due to the `break;` above
            let mut target_chemical = "";
            let mut target_quantity = 0;

            // pick a chemical other than ORE
            for (chemical, quantity) in &requirements {
                if *chemical == "ORE" {
                    continue;
                }

                target_chemical = chemical;
                target_quantity = *quantity;

                break;
            }

            // use any excess chemical we have left over
            let mut created_quantity = *excess.get(target_chemical).unwrap_or(&0);

            // then find the reaction to create more chemical
            let reaction = reactions.get(target_chemical).unwrap();

            // and create more until we have enough
            while created_quantity < target_quantity {
                created_quantity += reaction.output.quantity;

                // and the inputs as further requirements
                for input in &reaction.inputs {
                    *requirements.entry(input.chemical).or_insert(0) += input.quantity;
                }
            }

            // remove the now satisified required chemical
            requirements.remove(target_chemical);

            // record any chemical excess
            excess.insert(target_chemical, created_quantity-target_quantity);
        }

        fuel_created += 1;
        println!("{} FUEL, {:012} ORE", fuel_created, *requirements.get("ORE").unwrap());
    }

    println!("{} FUEL", fuel_created-1);
}

fn get_reactions(data: &'static str) -> HashMap<&str, Reaction> {
    let mut reactions = HashMap::new();

    for line in data.lines() {
        let parts: Vec<_> = line.split(" => ").collect();
        assert!(parts.len() == 2);

        let output = Component::parse(parts[1]);
        let mut inputs = vec!();

        for part in parts[0].split(", ") {
            inputs.push(Component::parse(part));
        }

        reactions.insert(output.chemical, Reaction{
            output,
            inputs,
        });
    }

    reactions
}
