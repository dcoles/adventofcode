use std::path::Path;
use std::fs;

type Input = Vec<u64>;

const BASE: u64 = 7;
const PRIME: u64 = 20201227;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");

    input.lines()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect()
}

/// Find `loop_number` of transforms applied to `subject_number` to get `x`.
fn find_loop_number(x: u64, subject_number: u64) -> u64 {
    let mut val = 1;
    for n in 1.. {
        val = (val * subject_number) % PRIME;
        if val == x {
            return n;
        }
    }
    unreachable!()
}

/// Apply transform to `subject_number` `loop_number` of times.
fn transform(subject_number: u64, loop_number: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loop_number {
        val = (val * subject_number) % PRIME;
    }

    val
}

fn part1(input: &Input) -> u64 {
    let public1 = input[0];
    let public2 = input[1];

    let private1 = find_loop_number(public1, BASE);
    transform(public2, private1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loops() {
        assert_eq!(find_loop_number(5764801, BASE), 8);
        assert_eq!(find_loop_number(17807724, BASE), 11);
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform(BASE, 8), 5764801);
        assert_eq!(transform(BASE, 11), 17807724);
    }

    #[test]
    fn test_part1() {
        let input = vec![5764801, 17807724];
        assert_eq!(part1(&input), 14897079);
    }
}

