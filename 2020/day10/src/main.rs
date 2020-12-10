use std::path::Path;
use std::fs;

type Input = Vec<u64>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

fn part1(input: &Input) -> u64 {
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

fn part2(input: &Input) -> u64 {
    //let mut adapters: HashSet<u64> = input.iter().copied().collect();
    let mut adapters = input.clone();
    adapters.sort();

    let mut combinations = 1;

    let mut n2 = 0;
    let mut n1 = 0;
    let mut n0 = 1;

    let mut prev = 0;
    for &n in &adapters {
        if n == prev + 1 {
            let n3 = n2;
            n2 = n1;
            n1 = n0;
            n0 = n1 + n2 + n3;
        } else {
            combinations *= n0;
            n2 = 0;
            n1 = 0;
            n0 = 1;
        }
        prev = n;
    }

    // Need to account for a possible last run
    combinations *= n0;

    combinations
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

    #[test]
    fn test_part2_sample1() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn test_part2_sample2() {
        let input = read_input("sample2.txt");
        assert_eq!(part2(&input), 19208);
    }
}

