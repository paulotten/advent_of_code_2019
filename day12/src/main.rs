mod data;

#[derive(Debug)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

fn main() {
    // part 1, sample 1
    //let data = data::_get_sample1();
    //let steps = 10;

    // part 1
    let data = data::get_data();
    let steps = 1000;

    let moons = &mut get_moons(data);

    for _ in 0..steps {
        simulate_step(moons);
    }

    println!("{}", get_energy(moons));
}

fn get_moons(data: &str) -> Vec<Moon> {
    data.lines().map(|line| {
        let parts: Vec<_> = line[1..line.len()-1].split(", ").collect();
        assert!(parts.len() == 3);

        let mut pos = [0; 3];

        for i in 0..pos.len() {
            let parts: Vec<_> = parts[i].split("=").collect();
            assert!(parts.len() == 2);

            pos[i] = parts[1].parse().unwrap();
        }

        Moon {
            pos,
            vel: [0, 0, 0],
        }
    }).collect()
}

fn simulate_step(moons: &mut Vec<Moon>) {
    // update velocities
    for i in 0..moons.len()-1 {
        for j in i+1..moons.len() {
            for axis in 0..3 {
                if moons[i].pos[axis] < moons[j].pos[axis] {
                    moons[i].vel[axis] += 1;
                    moons[j].vel[axis] -= 1;
                } else if moons[i].pos[axis] > moons[j].pos[axis] {
                    moons[i].vel[axis] -= 1;
                    moons[j].vel[axis] += 1;
                }
            }
        }
    }

    // update positions
    for moon in moons {
        for axis in 0..3 {
            moon.pos[axis] += moon.vel[axis];
        }
    }
}

fn get_energy(moons: &Vec<Moon>) -> i32 {
    let mut energy = 0;

    for moon in moons {
        let potential = i32::abs(moon.pos[0]) + i32::abs(moon.pos[1]) + i32::abs(moon.pos[2]);
        let kinetic = i32::abs(moon.vel[0]) + i32::abs(moon.vel[1]) + i32::abs(moon.vel[2]);

        energy += potential*kinetic;
    }

    energy
}
