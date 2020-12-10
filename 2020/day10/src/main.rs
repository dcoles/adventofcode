use std::path::Path;
use std::fs;

type Input = Vec<u32>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

fn part1(input: &Input) -> u32 {
    let mut adapters = input.clone();
    adapters.sort();

    let mut ones = 0;
    let mut threes = 1;  // Device has a built-in 3 jolt adapter

    let mut prev = 0;
    for &j in adapters.iter() {
        match j - prev {
            1 => ones += 1,
            3 => threes += 1,
            _ => ()
        }

        prev = j;
    }

    ones * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part1_sample2() {
        let input = read_input("sample2.txt");
        assert_eq!(part1(&input), 220);
    }
}

