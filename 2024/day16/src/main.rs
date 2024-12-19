//! Advent of Code 2024: Day 16
//! https://adventofcode.com/2024/day/16

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::path::Path;
use std::str::FromStr;
use lib::grid::{Grid, Pos};

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{}", input.grid);

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn find_start_end(grid: &Grid) -> (Pos, Pos) {
    let mut start = Pos::default();
    let mut end = Pos::default();

    for pos in grid.positions() {
        match grid[pos] {
            START => start = pos,
            END => end = pos,
            _ => (),
        }
    }

    (start, end)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn clockwise(self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    pub fn anticlockwise(self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }
}

impl From<Direction> for Pos {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::East => Pos::new([1, 0]),
            Direction::South => Pos::new([0, 1]),
            Direction::West => Pos::new([-1, 0]),
            Direction::North => Pos::new([0, -1]),
        }
    }
}

fn part1(input: &Input) -> usize {
    let (start, end) = find_start_end(&input.grid);

    let mut best: BTreeMap<(Pos, Direction), usize> = [((start, Direction::East), 0)].into_iter().collect();
    let mut edge: BinaryHeap<(usize, (Pos, Direction), BTreeSet<Pos>)> = [(0, (start, Direction::East), BTreeSet::new())].into_iter().collect();

    while let Some((_, cur, path)) = edge.pop() {
        if cur.0 == end {
            return best[&cur];
        }

        for next in [(cur.0 + cur.1.into(), cur.1), (cur.0, cur.1.clockwise()), (cur.0, cur.1.anticlockwise())] {
            if input.grid[next.0] == WALL {
                continue;
            }

            let new_score = best[&cur] + (next.1 != cur.1).then_some(1000).unwrap_or(1);
            if best.contains_key(&next) && best[&next] < new_score {
                continue;
            }

            let mut new_path: BTreeSet<Pos> = path.clone();
            new_path.insert(next.0);

            best.insert(next, new_score);
            edge.push((usize::MAX - new_score, next, new_path));
        }
    }

    0
}

fn part2(input: &Input) -> usize {
    let (start, end) = find_start_end(&input.grid);

    let mut best: BTreeMap<(Pos, Direction), usize> = [((start, Direction::East), 0)].into_iter().collect();
    let start_path: BTreeSet<Pos> = [start].into_iter().collect();
    let mut edge: BinaryHeap<(usize, (Pos, Direction), BTreeSet<Pos>)> = [(0, (start, Direction::East), start_path)].into_iter().collect();

    let mut best_score = None;
    let mut all_path = BTreeSet::new();

    while let Some((_, cur, path)) = edge.pop() {
        if cur.0 == end {
            all_path.extend(path.iter().copied());

            if let Some(best_score) = best_score {
                if best[&cur] > best_score {
                    break;
                }
            }
            best_score = Some(best[&cur]);
        }

        for next in [(cur.0 + cur.1.into(), cur.1), (cur.0, cur.1.clockwise()), (cur.0, cur.1.anticlockwise())] {
            if input.grid[next.0] == WALL {
                continue;
            }

            let new_score = best[&cur] + (next.1 != cur.1).then_some(1000).unwrap_or(1);
            if best.contains_key(&next) && best[&next] < new_score {
                continue;
            }

            let mut new_path: BTreeSet<Pos> = path.clone();
            new_path.insert(next.0);

            best.insert(next, new_score);
            edge.push((usize::MAX - new_score, next, new_path));
        }
    }

    all_path.len()
}

#[derive(Debug, Clone)]
struct Input {
    pub grid: Grid,
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
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 7036);
    }

    #[test]
    fn test_part1_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 11048);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 102460);
    }

    #[test]
    fn test_part2_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 45);
    }

    #[test]
    fn test_part2_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 64);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 527);
    }
}
