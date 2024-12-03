//! Advent of Code 2024: Day 3
//! https://adventofcode.com/2024/day/3

use std::{fs, io};
use std::path::Path;
use regex::Regex;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    mul_re.captures_iter(&input.input)
        .map(|m| {
            let a: usize = m.get(1).and_then(|m| m.as_str().parse().ok()).unwrap();
            let b: usize = m.get(2).and_then(|m| m.as_str().parse().ok()).unwrap();

            a * b
        })
        .sum()
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

fn part2(input: &Input) -> usize {
    let instr_re = Regex::new(r"(?x) (?<instr>mul|do(n't)?) \( (?:(?<a>\d{1,3}) , (?<b>\d{1,3}))? \)").unwrap();

    instr_re.captures_iter(&input.input)
        .map(|m| {
            match m.name("instr").unwrap().as_str() {
                "mul" => {
                    let a: usize = m.name("a").and_then(|m| m.as_str().parse().ok()).unwrap();
                    let b: usize = m.name("b").and_then(|m| m.as_str().parse().ok()).unwrap();

                    Instruction::Mul(a, b)
                },
                "do" => Instruction::Do,
                "don't" => Instruction::Dont,
                instr => panic!("unknown instruction {instr:?}"),
            }
        })
        .fold((true, 0), |(do_it, acc), instr| match instr {
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
            Instruction::Mul(a, b) if do_it => (true, acc + a * b),
            _ => (do_it, acc),
        })
        .1
}

#[derive(Debug, Clone)]
struct Input {
    input: String,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        Ok(Self { input })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 175615763);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 48);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 74361272);
    }
}
