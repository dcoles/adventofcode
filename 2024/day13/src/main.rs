//! Advent of Code 2024: Day 13
//! https://adventofcode.com/2024/day/13

use std::{fs, io};
use std::path::Path;
use lib::vector::Vector;

const COST_A: usize = 3;
const COST_B: usize = 1;

type Vec2 = Vector<i64, 2>;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut total_cost = 0;

    for machine in &input.values {
        let da = machine.button_a;
        let db = machine.button_b;
        let p = machine.prize;

        // Derived via linear algebra
        let num = da[0] * p[1] - da[1] * p[0];
        let denom = da[0] * db[1] - da[1] * db[0];

        if num % denom != 0 {
            continue;
        }

        let b = num / denom;
        let a = (p[0] - db[0] * b) / da[0];

        total_cost += 3 * a as usize + b as usize;
    }

    total_cost
}

fn part2(input: &Input) -> usize {
    let mut total_cost = 0;

    for machine in &input.values {
        let da = machine.button_a;
        let db = machine.button_b;
        let p = machine.prize + Vec2::new([10000000000000, 10000000000000]);

        // Derived by linear algebra
        let num = da[0] * p[1] - da[1] * p[0];
        let denom = da[0] * db[1] - da[1] * db[0];

        if num % denom != 0 {
            continue;
        }

        let b = num / denom;
        let a = (p[0] - db[0] * b) / da[0];

        total_cost += COST_A * a as usize + COST_B * b as usize;
    }

    total_cost
}

#[derive(Debug, Clone, Default)]
struct Machine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

fn parse_button(s: &str) -> Vec2 {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x[1..].parse().unwrap();
    let y = y[1..].parse().unwrap();

    Vec2::new([x, y])
}

fn parse_prize(s: &str) -> Vec2 {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x[2..].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Vec2::new([x, y])
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Machine>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let mut values = vec![];

        for chunk in input.split("\n\n") {
            let mut machine = Machine::default();
            for line in chunk.lines() {
                let (field, value) = line.trim().split_once(": ").unwrap();
                match field {
                    "Button A" => machine.button_a = parse_button(value),
                    "Button B" => machine.button_b = parse_button(value),
                    "Prize" => machine.prize = parse_prize(value),
                    _ => panic!("unknown field {field:?}"),
                }
            }

            values.push(machine);
        }


        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 480);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 29522);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 101214869433312);
    }
}
