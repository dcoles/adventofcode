//! Advent of Code 2023: Day 3
//! https://adventofcode.com/2023/day/3

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{:?}", input);

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u32 {
    let mut sum = 0;

    for (pos, len, number) in input.numbers.iter().copied() {
        for (spos, _) in input.symbols.iter().copied() {
            let xmin = pos.0.saturating_sub(1);
            let xmax = pos.0 + len;
            let ymin = pos.1.saturating_sub(1);
            let ymax = pos.1 + 1;

            if (xmin..=xmax).contains(&spos.0) && (ymin..=ymax).contains(&spos.1) {
                // We're adjacent to a symbol
                sum += number;
                break;
            }
        }
    }

    sum
}

fn part2(input: &Input) -> u32 {
    let mut adjacent_numbers: HashMap<Pos, Vec<u32>> = HashMap::new();

    let gears: Vec<_> = input.symbols.iter().filter_map(|(p, s)| (*s == '*').then_some(*p)).collect();
    for (num_pos, len, number) in input.numbers.iter().copied() {
        for gear_pos in gears.iter().copied() {
            let xmin = num_pos.0.saturating_sub(1);
            let xmax = num_pos.0 + len;
            let ymin = num_pos.1.saturating_sub(1);
            let ymax = num_pos.1 + 1;

            if (xmin..=xmax).contains(&gear_pos.0) && (ymin..=ymax).contains(&gear_pos.1) {
                // We're adjacent to a gear
                adjacent_numbers.entry(gear_pos).or_default().push(number);
            }
        }
    }

    adjacent_numbers.into_values()
    .filter(|numbers| numbers.len() == 2)
    .map(|numbers| numbers.into_iter().product::<u32>())
    .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(pub usize, pub usize);

#[derive(Debug, Clone)]
struct Input {
    symbols: Vec<(Pos, char)>,
    numbers: Vec<(Pos, usize, u32)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut symbols = Vec::new();
        let mut numbers = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut x = 0;
            while x < line.len() {
                // Skip to the next valid symbol/number
                x += match line[x..].find(|c| c != '.') {
                    None => break,
                    Some(x) => x,
                };

                let first_char = line[x..].chars().next().unwrap();
                if first_char.is_ascii_punctuation() {
                    // A symbol
                    symbols.push((Pos(x, y), first_char));
                    x += 1;
                } else {
                    // A number
                    let x_end = line[x..].find(|c: char| !c.is_numeric()).map(|n| x + n).unwrap_or(line.len());
                    let number = line[x..x_end].parse().unwrap();
                    numbers.push((Pos(x, y), x_end - x, number));
                    x = x_end;
                }
            }
        }

        Ok(Input { symbols, numbers })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 517021);
    }

    #[test]
    fn test_part2_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 467835);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 81296995);
    }
}
