fn main() {
    let min = 278384;
    let max = 824795;

    puzzle2(min, max);
}

fn _puzzle1(min: u32, max: u32) {
    let mut count = 0;

    for i in min / 100_000 .. (max / 100_000) + 1 {
        for j in i..10 {
            for k in j..10 {
                for l in k..10 {
                    for m in l..10 {
                        for n in m..10 {
                            if i==j || j == k || k == l || l == m || m == n {
                                let num = i*100_000 + j*10_000 + k*1000 + l*100 + m*10 + n;

                                if num >= min && num <= max {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn puzzle2(min: u32, max: u32) {
    let mut count = 0;

    'outer: for i in min / 100_000 .. 10 {
        for j in i..10 {
            for k in j..10 {
                for l in k..10 {
                    for m in l..10 {
                        for n in m..10 {
                            let num = i*100_000 + j*10_000 + k*1000 + l*100 + m*10 + n;

                            if num < min {
                                continue;
                            }
                            if num > max {
                                break 'outer;
                            }
                            if !lone_doubles(num) {
                                continue;
                            }

                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn lone_doubles(num: u32) -> bool {
    let num: Vec<char> = num.to_string().chars().collect();

    let mut current_char = ' ';
    let mut current_run = 0;

    for i in 0..6 {
        if num[i] != current_char {
            if current_run == 2 {
                return true;
            }

            current_char = num[i];
            current_run = 0;
        }

        current_run += 1;
    }

    current_run == 2
}
