use std::path::Path;
use std::fs;
use regex::Regex;

type Input = Vec<String>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

fn part1(input: &Input) -> u32 {
    let re = Regex::new(r"^(\d+)$").unwrap();

    let mut sum = 0;
    for line in input {
        if let Some(m) = re.captures(&line) {
            sum += m[1].parse::<u32>().expect("Failed to parse number");
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("input.txt");
        assert_eq!(part1(&input), 45);
    }
}

