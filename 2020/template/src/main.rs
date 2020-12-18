use std::path::Path;
use std::fs;

type Input = Vec<String>;

fn main() {
    let input = read_input("input.txt");
    println!("> {:?}", input);

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

fn part1(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 0);
    }
}

