mod input;

use input::*;

fn main() {
    part1();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let input = parse_input(input);

    let output = fft(input, 100);
    let output = format_output(output);
    println!("{output}");
}

fn parse_input(input: &str) -> Vec<i8> {
    let mut parsed = vec![];

    for c in input.chars() {
        parsed.push(c.to_digit(10).unwrap() as i8);
    }

    parsed
}

fn format_output(output: Vec<i8>) -> String {
    let mut formatted = String::new();

    for o in output {
        formatted += &format!("{o}");
    }

    formatted
}

fn fft(input: Vec<i8>, phases: u32) -> Vec<i8> {
    let pattern: Vec<i8> = vec![1, 0, -1, 0];

    let mut cur = input;

    for _ in 0..phases {
        let mut next: Vec<i8> = vec![];

        for i in 0..cur.len() {
            let mut sum: i32 = 0;

            for j in 0..cur.len() - i {
                let a = cur[i + j];
                let b = pattern[(j / (i + 1)) % pattern.len()];

                sum += a as i32 * b as i32;
            }

            sum %= 10;
            sum = sum.abs();

            next.push(sum as i8);
        }

        cur = next;
    }

    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase1() {
        let output = fft(vec![1, 2, 3, 4, 5, 6, 7, 8], 1);

        assert_eq!(output, vec![4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn phase2() {
        let output = fft(vec![1, 2, 3, 4, 5, 6, 7, 8], 2);

        assert_eq!(output, vec![3, 4, 0, 4, 0, 4, 3, 8]);
    }
}
