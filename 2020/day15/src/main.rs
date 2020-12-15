use std::path::Path;
use std::fs;

type Input = Vec<usize>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input, 2020));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .trim()
        .split(",")
        .map(|val| val.parse().expect("Failed to parse number"))
        .collect()
}

fn part1(input: &[usize], max_n: usize) -> usize {
    let mut numbers: Vec<_> = input.to_owned();
    for _ in input.len()+1..=max_n {
        let &prev = numbers.last().unwrap();

        if let Some(difference) = numbers.iter().rev().skip(1).position(|&x| x == prev) {
            numbers.push(difference + 1);
        } else {
            numbers.push(0);
        }
    }

    *numbers.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[0, 3, 6], 2020), 436);
    }
}

