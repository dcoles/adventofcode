/// Advent of Code 2021: Day 5
/// https://adventofcode.com/2021/day/5

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

type Position = (i32, i32);

fn main() -> io::Result<()> {
    let input = read_input_from_file("day05/input.txt")?;

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[(Position, Position)]) -> usize {
    let mut map: HashMap<Position, usize> = HashMap::new();

    for &((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            // Vertical line
            // Need to ensure ranges are in order
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in y1..=y2 {
                *map.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            // Horizontal line
            // Need to ensure ranges are in order
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in x1..=x2 {
                *map.entry((x, y1)).or_default() += 1;
            }
        }
    }

    map.values().filter(|&&count| count >= 2).count()
}

fn part2(input: &[(Position, Position)]) -> usize {
    let mut map: HashMap<Position, usize> = HashMap::new();
    
    for &((x1, y1), (x2, y2)) in input {
        let xstep = step(x1, x2);
        let ystep = step(y1, y2);

        let (mut x, mut y) = (x1, y1);
        loop {
            *map.entry((x, y)).or_default() += 1;

            if x == x2 && y == y2 {
                // End of the line 🚂
                break;
            }

            x += xstep;
            y += ystep;
        }
    }

    map.values().filter(|&&count| count >= 2).count()
}

/// Determine step size.
fn step(x1: i32, x2: i32) -> i32 {
    match (x1, x2) {
        (x1, x2) if x1 < x2 => 1,
        (x1, x2) if x1 > x2 => -1,
        _ => 0,
    }
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Vec<(Position, Position)>> {
    let input = fs::read_to_string(path)?;

    let mut result = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();

        let (x1, y1) = a.split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .expect("Failed to parse input");

        let (x2, y2) = b.split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .expect("Failed to parse input");

        result.push(((x1, y1), (x2, y2)));
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(12, part2(&input));
    }
}
