//! Advent of Code 2023: Day X
//! https://adventofcode.com/2023/day/X

use std::collections::BTreeMap;
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example3.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

const START: &str = "AAA";
const END: &str = "ZZZ";

fn part1(input: &Input) -> usize {
    let mut current = START;
    for (i, instruction) in input.instructions.iter().cycle().enumerate() {
        current = match instruction {
            Instruction::L => &input.network.get(current).unwrap().0,
            Instruction::R => &input.network.get(current).unwrap().1,
        };

        if current == END {
            return i + 1;
        }
    }

    unreachable!()
}

fn part2(input: &Input) -> usize {
    let start: Vec<&str> = input.network.keys().filter(|k| k.ends_with('A')).map(String::as_str).collect();
    let mut current = start.clone();

    let mut period = vec![0; current.len()];
    for (i, instruction) in input.instructions.iter().cycle().enumerate() {
        for (j, node) in current.iter_mut().enumerate() {
            *node = match instruction {
                Instruction::L => &input.network.get(*node).unwrap().0,
                Instruction::R => &input.network.get(*node).unwrap().1,
            };

            if node.ends_with('Z') {
                if period[j] == 0 {
                    period[j] = i + 1;
                }
            }
        }

        if period.iter().all(|&n| n != 0) {
            // We know all the periods and offsets
            break;
        }
    }

    // A brute-force way to calculate the GCM
    let mut div = 2;
    let mut gcm = 1;
    while div < 10000 {
        if period.iter().all(|&n| n % div == 0) {
            gcm *= div;
            period = period.into_iter().map(|n| n / div).collect();
        } else {
            div += 1;
        }
    }

    period.iter().product::<usize>() * gcm
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    L,
    R,
}

#[derive(Debug, Clone)]
struct Input {
    instructions: Vec<Instruction>,
    network: BTreeMap<String, (String, String)>,
}

impl Instruction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("Unknown instrcution {c:?}"),
        }
    }
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut instructions = vec![];
        let mut network = BTreeMap::new();
        for (n, part) in input.split("\n\n").enumerate() {
            match n {
                0 => {
                    for c in part.chars() {
                        instructions.push(Instruction::from_char(c));
                    }
                },
                1 => {
                    for line in part.lines() {
                        let (node, elements) = line.split_once("=").unwrap();
                        let node = node.trim().to_owned();
                        let (element1, element2) = elements.split_once(',')
                        .map(|(e1, e2)| (
                            e1.trim_matches(|c: char| !c.is_alphanumeric()).to_owned(),
                            e2.trim_matches(|c: char| !c.is_alphanumeric()).to_owned()
                        )).unwrap();

                        network.insert(node, (element1, element2));
                    }
                },
                _ => (),
            }
        }

        Ok(Self { instructions, network })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part1_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 12169);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example3.txt").unwrap();

        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 12030780859469);
    }
}
