//! Advent of Code 2022: Day 17
//! https://adventofcode.com/2022/day/17

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

const WIDTH: usize = 7;
const SCAN_ROWS: usize = 32;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    simulate(input, 2022)
}

fn part2(input: &Input) -> usize {
    simulate(input, 1000000000000)
}

fn simulate(input: &Input, n_rocks: usize) -> usize {
    let mut chamber: HashSet<(usize, usize)> = HashSet::new();
    let mut jets = input.values.iter().copied().cycle().enumerate();
    let mut seen: HashMap<_, usize> = HashMap::new();
    let mut heights: HashMap<usize, usize> = HashMap::new();

    for (n, shape) in shapes().into_iter().cycle().enumerate().take(n_rocks) {
        // A new shape appears 2-units from the left wall and 3-units above the highest rock
        //println!("Rock {}", n + 1);
        let mut x: usize = 2;
        let mut y: usize = chamber.iter().map(|&(_, y)| y + 1).max().unwrap_or(0) + 3;

        let mut jet_index: usize = 0;

        // Simulate the block falling as affected by the jet bursts
        loop {
            let (j, d) = jets.next().unwrap();

            // Index of the jet in the stream
            jet_index = j % input.values.len();

            match d {
                '<' => {
                    // Push left
                    //println!("<");
                    x = x.saturating_sub(1);

                    if collision(&chamber, &shape, (x, y)) {
                        // Hit object
                        x = x + 1;
                    }
                },
                '>' => {
                    // Push right
                    //println!(">");
                    x = (x + 1).min(WIDTH - shape.width);

                    if collision(&chamber, &shape, (x, y)) {
                        // Hit object
                        x = x - 1;
                    }
                },
                c => panic!("Unknown pattern: {}", c),
            }

            if y == 0 {
                // Hit the floor
                for &(x1, y1) in &shape.block {
                    chamber.insert((x + x1, y + y1));
                }

                break;
            }
            
            y = y - 1;

            if collision(&chamber, &shape, (x, y)) {
                // Hit another block
                y = y + 1;

                for &(x1, y1) in &shape.block {
                    chamber.insert((x + x1, y + y1));
                }

                break;
            }
        }

        // Whats the height of the tower?
        heights.insert(n, chamber.iter().map(|&(_, y)| y + 1).max().unwrap_or_default());

        // Now check if we've seen this state before
        let shape_index = n % shapes().len();
        let s = scan(&chamber);
        let state = (jet_index, shape_index, s);

        if let Some(&nn) = seen.get(&state) {
            // We just saw a state we've already been in!
            let rocks_ago = n - nn;
            let height_delta = heights[&n] - heights[&nn];

            let m = n_rocks - (nn + 1);
            let r = (m.div_euclid(rocks_ago), m.rem_euclid(rocks_ago));

            // Calculated height
            return r.0 * height_delta + heights[&(nn + r.1)];
        }

        seen.insert(state, n);
    }

    heights[&(n_rocks - 1)]
}

fn collision(chamber: &HashSet<(usize, usize)>, shape: &Shape, (x, y): (usize, usize)) -> bool {
    for &(x1, y1) in shape.block.iter() {
        if chamber.contains(&(x + x1, y + y1)) {
            return true;
        }
    }

    false
}

fn display(chamber: &HashSet<(usize, usize)>, rows: usize) {
    let y_max = chamber.iter().map(|&(_, y)| y).max().unwrap_or(0) + 3;
    for y in (y_max - rows..=y_max).rev() {
        for x in 0..WIDTH {
            print!("{}", if chamber.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}

fn scan(chamber: &HashSet<(usize, usize)>) -> [u8; SCAN_ROWS] {
    let height = chamber.iter().map(|&(_, y)| y + 1).max().unwrap_or(0);

    let mut value = [0; SCAN_ROWS];
    for (n, y) in (height-height.min(SCAN_ROWS)..height).rev().enumerate() {
        let mut v = 0;
        for x in 0..WIDTH {
            v <<= 1;
            if chamber.contains(&(x, y)) {
                v |= 1;
            }
        }

        value[n] = v;
    }

    value
}

#[derive(Debug, Clone)]
struct Shape {
    width: usize,
    height: usize,
    block: HashSet<(usize, usize)>,
}

fn shapes() -> Vec<Shape> {
    vec![
        // ####
        Shape {
            width: 4,
            height: 1,
            block: [(0, 0), (1, 0), (2, 0), (3, 0)].into_iter().collect(),
        },

        //  #
        // ###
        //  #
        Shape {
            width: 3,
            height: 3,
            block: [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].into_iter().collect(),
        },

        //   #
        //   #
        // ###
        Shape {
            width: 3,
            height: 3,
            block: [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].into_iter().collect(),
        },

        // #
        // #
        // #
        // #
        Shape {
            width: 1,
            height: 4,
            block: [(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
        },

        // ##
        // ##
        Shape {
            width: 2,
            height: 2,
            block: [(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().collect(),
        },
    ]
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let values = input.trim().chars().collect();

        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 3068);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 1514285714288);
    }
}
