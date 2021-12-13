//! Advent of Code 2021: Day 13
//! https://adventofcode.com/2021/day/13

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

type Pos = (u32, u32);

fn main() {
    let input = read_input_from_file("day13/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2");
    part2(&input);
}

fn part1(input: &Input) -> usize {
    let mut paper = Paper::new(input.dots.clone());
    paper.fold(input.folds[0]);

    paper.dots.len()
}

fn part2(input: &Input) {
    let mut paper = Paper::new(input.dots.clone());
    for &fold in &input.folds {
        paper.fold(fold);
    }

    paper.print(40, 6);
}

struct Paper {
    dots: HashSet<Pos>,
}

impl Paper {
    fn new(dots: HashSet<Pos>) -> Self {
        Paper {
            dots
        }
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::X(fold) => {
                let dots: Vec<_> = self.dots.iter().copied().filter(|&(x, _)| x > fold).collect();
                for pos in &dots {
                    self.dots.remove(pos);
                }

                for (x, y) in dots {
                    self.dots.insert((2 * fold - x, y));
                }
            },
            Fold::Y(fold) => {
                let dots: Vec<_> = self.dots.iter().copied().filter(|&(_, y)| y > fold).collect();
                for pos in &dots {
                    self.dots.remove(pos);
                }

                for (x, y) in dots {
                    self.dots.insert((x, 2* fold - y));
                }
            },
        }
    }

    fn print(&self, width: usize, height: usize) {
        for y in 0..height as u32 {
            for x in 0..width as u32 {
                print!("{}", if self.dots.contains(&(x, y)) { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Input> {
    let input = fs::read_to_string(path)?;

    let (coord_chunk, fold_chunk) = input.split_once("\n\n").unwrap();

    let mut dots = HashSet::new();
    for line in coord_chunk.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        dots.insert((x, y));
    }

    let mut folds = Vec::new();
    for line in fold_chunk.lines() {
        let (prefix, value) = line.split_once("=").unwrap();
        let fold = match prefix {
            "fold along x" => Fold::X(value.parse().unwrap()),
            "fold along y" => Fold::Y(value.parse().unwrap()),
            _ => panic!("Unknown prefix: {}", prefix),
        };

        folds.push(fold);
    }

    Ok(Input { dots, folds })
}

#[derive(Debug)]
struct Input {
    dots: HashSet<Pos>,
    folds: Vec<Fold>,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 17);
    }
}
