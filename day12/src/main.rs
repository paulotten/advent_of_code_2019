mod data;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

fn main() {
    //part1();
    part2();
}

fn part1() {
    // sample 1
    // result 179
    //let data = data::_get_sample1();
    //let steps = 10;

    // puzzle data
    // result 12082
    let data = data::get_data();
    let steps = 1000;

    let moons = &mut get_moons(data);

    for _ in 0..steps {
        simulate_step(moons);
    }

    println!("result {}", get_energy(moons));
}

fn part2() {
    // sample 1
    // result 2772
    //let data = data::_get_sample1();

    // sample 2
    // result 4686774924
    //let data = data::_get_sample2();

    // puzzle data
    // result 295693702908636
    let data = data::get_data();

    let moons = &mut get_moons(data);
    let initial_moons = &(*moons).clone();

    /*
    I think the trick here is that the axis are fully independent.
    They don't effect each other. So while this looks like
    https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html
    it really isn't.

    So I can calculate when each axis first repeats, then calculate the Least Common Multiple.
    */
    for axis in 0..3 {
        let mut steps: u64 = 0;

        while {
            steps += 1;
            simulate_step_axis(moons, axis);

            !axis_identical(axis, moons, initial_moons)
        } {}

        println!("axis {}: {} steps", axis, steps);
    }

    /*
    I just tossed the three numbers this output into
    https://www.calculatorsoup.com/calculators/math/lcm.php to get my answer.
    */
}

fn axis_identical(axis: usize, moons: &Vec<Moon>, initial_moons: &Vec<Moon>) -> bool {
    for (moon, initial_moon) in moons.iter().zip(initial_moons) {
        if moon.pos[axis] != initial_moon.pos[axis] {
            return false;
        }

        if moon.vel[axis] != initial_moon.vel[axis] {
            return false;
        }
    }

    true
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
    for axis in 0..3 {
        simulate_step_axis(moons, axis);
    }
}

fn simulate_step_axis(moons: &mut Vec<Moon>, axis: usize) {
    // update velocities
    for i in 0..moons.len()-1 {
        for j in i+1..moons.len() {
            if moons[i].pos[axis] < moons[j].pos[axis] {
                moons[i].vel[axis] += 1;
                moons[j].vel[axis] -= 1;
            } else if moons[i].pos[axis] > moons[j].pos[axis] {
                moons[i].vel[axis] -= 1;
                moons[j].vel[axis] += 1;
            }
        }
    }

    // update positions
    for moon in moons {
        moon.pos[axis] += moon.vel[axis];
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
