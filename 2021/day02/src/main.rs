/// Advent of Code 2021: Day 2
/// https://adventofcode.com/2021/day/2

use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_input_from_file("day02/input.txt")?;

    // Part 1
    let (position, depth) = part1(&input);
    println!("Part 1: {}", position * depth);

    // Part 2
    let (position, depth) = part2(&input);
    println!("Part 2: {}", position * depth);

    Ok(())
}

fn read_input_from_file(path: impl AsRef<Path>) -> anyhow::Result<Vec<Movement>> {
    let input = fs::read_to_string(path)?;

    Ok(input.lines().map(Movement::from_str).collect())
}

fn part1(input: &[Movement]) -> (i32, i32) {
    let mut position = 0;
    let mut depth = 0;

    for movement in input {
        match movement {
            &Movement::Forward(x) => position += x,
            &Movement::Up(x) => depth -= x,
            &Movement::Down(x) => depth += x,
        }
    }

    (position, depth)
}

fn part2(input: &[Movement]) -> (i32, i32) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for movement in input {
        match movement {
            &Movement::Forward(x) => {
                position += x;
                depth += aim * x;
            },
            &Movement::Up(x) => aim -= x,
            &Movement::Down(x) => aim += x,
        }
    }

    (position, depth)
}

#[derive(Debug)]
enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Movement {
    fn from_str(s: &str) -> Self {
        let (direction, x) = s.split_once(" ").expect("failed to split value");
        let n: i32 = x.parse().expect("failed to parse magnitude");

        match direction {
            "forward" => Movement::Forward(n),
            "down" => Movement::Down(n),
            "up" => Movement::Up(n),
            d => panic!("Unknown direction {:?}", d),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [Movement; 6] = [Movement::Forward(5), Movement::Down(5), Movement::Forward(8), Movement::Up(3), Movement::Down(8), Movement::Forward(2)];

    #[test]
    fn test_part1() {
        let (position, depth) = part1(&INPUT);
        assert_eq!(position * depth, 150);
    }

    #[test]
    fn test_part2() {
        let (position, depth) = part2(&INPUT);
        assert_eq!(position * depth, 900);
    }
}
