mod data;

use std::collections::HashMap;

fn main() {
    let data = data::get_data();
    let orbits = build_orbital_map(data);

    puzzle2(orbits);
}

fn build_orbital_map(data: &str) -> HashMap<&str, &str> {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split(')').collect();

        assert!(parts.len() == 2);

        orbits.insert(parts[1], parts[0]);
    }

    orbits
}

fn _puzzle1(orbits: HashMap<&str, &str>) {
    let mut total_orbits = 0;
    
    for (mut node, _) in &orbits {
        while *node != "COM" {
            total_orbits += 1;
            node = orbits.get(node).unwrap();
        }
    }

    println!("{}", total_orbits);
}

fn puzzle2(orbits: HashMap<&str, &str>) {
    let mut santa_nodes: HashMap<&str, i32> = HashMap::new();
    let mut node = "SAN";
    let mut i = 0;

    while node != "COM" {
        node = orbits.get(node).unwrap();
        santa_nodes.insert(node, i);
        i += 1;
    }

    let mut node = orbits.get("YOU").unwrap();
    let mut j = 0;

    loop {
        if santa_nodes.contains_key(node) {
            println!("{}", santa_nodes.get(node).unwrap() + j);
            return;
        } else {
            node = orbits.get(node).unwrap();
            j += 1;
        }
    }
}
