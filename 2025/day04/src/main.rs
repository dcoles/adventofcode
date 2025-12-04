//! Advent of Code 2025: Day 4
//! <https://adventofcode.com/2025/day/4>

use std::{fs, io};
use std::path::Path;
use std::str::FromStr;

use lib::grid::{Grid, Pos};

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
    accessible(&input.grid).len()
}

fn accessible(grid: &Grid) -> Vec<Pos> {
    let mut accessible = Vec::new();

    for y in grid.y_range() {
        for x in grid.x_range() {
            let pos = Pos::new([x, y]);

            if grid[pos] == '@' {
                let mut count = 0;

                for adj in grid.adjacent8(pos) {
                    if grid[adj] == '@' {
                        count += 1;
                    }
                }

                if count < 4 {
                    accessible.push(pos);
                }
            }
        }
    }

    accessible
}

fn part2(input: &Input) -> usize {
    let mut total_removed = 0;
    let mut grid = input.grid.clone();

    loop {
        let access = accessible(&grid);
        let mut removed = 0;
        for pos in access.iter().copied() {
            grid[pos] = 'x';
            removed += 1;
        }

        if removed == 0 {
            break;
        }

        total_removed += removed;
    }

    total_removed
}

#[derive(Debug, Clone)]
struct Input {
    grid: Grid,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let grid = Grid::from_str(&input)?;

        Ok(Self { grid })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1543);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 43);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 9038);
    }
}
