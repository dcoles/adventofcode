//! Advent of Code 2025: Day 6
//! <https://adventofcode.com/2025/day/6>

use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn sum(it: impl Iterator<Item=i64>) -> i64 {
    it.sum()
}

fn product(it: impl Iterator<Item=i64>) -> i64 {
    it.product()
}

fn part1(input: &Input) -> i64 {
    let mut total = 0;

    let mut rows: Vec<_> = input.values.iter().map(|s| s.split_whitespace()).collect();

    let operations: Vec<_> = rows.pop().unwrap()
        .map(|s| match s {
            "+" => sum,
            "*" => product,
            _ => panic!(),
        })
        .collect();

    let values: Vec<Vec<_>> = rows.into_iter().map(|split| split.map(|s| s.parse::<i64>().unwrap()).collect()).collect();

    for (i, op) in operations.into_iter().enumerate() {
        total += op(values.iter().map(move |v| v[i]));
    }

    total
}

fn transpose(input: &[String]) -> Vec<String> {
    let mut transposed = vec![];

    for line in input {
        for (x, c) in line.trim_end().chars().enumerate() {
            if transposed.len() < (x + 1) {
                transposed.push(String::new());
            }

            if c != ' ' {
                transposed[x].push(c);
            }
        }
    }

    transposed
}

fn part2(input: &Input) -> i64 {
    let mut total = 0;

    let transposed = transpose(&input.values);
    let equations = transposed.join("\n");

    for equation in equations.split("\n\n") {
        let mut op = ' ';
        let mut values: Vec<i64> = vec![];
        for (i, line) in equation.lines().enumerate() {
            let value;
            if i == 0 {
                op = line.chars().last().unwrap();
                value = &line[0..(line.len() - 1)];
            } else {
                value = line;
            }

            values.push(value.trim().parse().unwrap());
        }

        let subtotal = match op {
            '+' => values.iter().copied().sum::<i64>(),
            '*' => values.iter().copied().product::<i64>(),
            _ => panic!("unknown op {op}"),
        };

        total += subtotal;
    }

    total
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<String>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(str::to_string).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 6172481852142);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 3263827);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 10188206723429);
    }
}
