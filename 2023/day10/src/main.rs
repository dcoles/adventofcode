//! Advent of Code 2023: Day 10
//! https://adventofcode.com/2023/day/10

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::{fs, io};
use std::path::Path;

// By manual inspection
const PIPE: Pipe = Pipe::Horizontal;
const FLOW: Direction = Direction::East;

const CARDINALS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut map = input.map.clone();
    map.insert(input.start, PIPE);

    let path = follow_pipe(&map, input.start);

    path.len() / 2
}

fn follow_pipe(map: &BTreeMap<Pos, Pipe>, start: Pos) -> Vec<Pos> {
    let mut path = vec![start];
    let mut cur = path[0];
    let mut direction = FLOW;

    loop {
        let pipe = map.get(path.last().unwrap()).unwrap();
        direction = pipe.flow(direction);

        let d = direction.vector();

        cur = (cur.0 + d.0, cur.1 + d.1);
        path.push(cur);

        if cur == start {
            break;
        }
    }

    path
}

fn part2(input: &Input) -> usize {
    let mut map = input.map.clone();
    map.insert(input.start, PIPE);

    let pipeline: HashSet<_> = follow_pipe(&map, input.start).into_iter().collect();

    let map = map.into_iter().filter(|(p, _)| pipeline.contains(p)).collect();

    let start = (0, 0);
    let mut seen: BTreeSet<_> = [start].into_iter().collect();
    let mut to_visit = vec![start];
    while let Some(cur) = to_visit.pop() {
        for (d, v) in CARDINALS.map(|d| (d, d.vector())) {
            let adj = (cur.0 + v.0, cur.1 + v.1);

            if seen.contains(&adj) {
                // Already visited
                continue;
            }

            if !(0..input.width).contains(&adj.0) || !(0..input.height).contains(&adj.1) {
                // Out of bounds
                continue;
            }

            if blocked(&map, cur, d) {
                // Not a valid movement
                continue
            }

            seen.insert(adj);
            to_visit.push(adj);
        }
    }

    draw_pipes(&map, input.width, input.height);
    println!();

    let mut n = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            let pos = (x, y);

            let c = if !seen.contains(&pos) {
                if pipeline.contains(&pos) {
                    '#'
                } else {
                    n += 1;

                    'I'
                }
            } else {
                '.'
            };
            print!("{}", c)
        }
        println!();
    }

    n
}

fn blocked(map: &BTreeMap<Pos, Pipe>, pos: Pos, direction: Direction) -> bool {
    let p0_0 = map.get(&pos).copied();
    let p1_0 = map.get(&(pos.0 + 1, pos.1)).copied();
    let p0_1 = map.get(&(pos.0, pos.1 + 1)).copied();
    let p1_1 = map.get(&(pos.0 + 1, pos.1 + 1)).copied();

    match direction {
        Direction::North => points(p0_0, Direction::East) && points(p1_0, Direction::West),
        Direction::South => points(p0_1, Direction::East) && points(p1_1, Direction::West),
        Direction::East => points(p1_0, Direction::South) && points(p1_1, Direction::North),
        Direction::West => points(p0_0, Direction::South) && points(p0_1, Direction::North),
    }
}

fn points(pipe: Option<Pipe>, direction: Direction) -> bool {
    pipe.map(|p| p.is_pointing(direction)).unwrap_or_default()
}

fn draw_pipes(map: &BTreeMap<Pos, Pipe>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            let pipe = map.get(&pos).copied();

            let c = match (points(pipe, Direction::North), points(pipe, Direction::South), points(pipe, Direction::East), points(pipe, Direction::West)) {
                (false, false, false, false) => '.',
                (true, true, true, true) => '+',

                (true, false, false, false) => '↑',
                (false, true, false, false) => '↓',
                (false, false, true, false) => '→',
                (false, false, false, true) => '←',

                (true, true, false, false) => '│',
                (false, false, true, true) => '─',

                (true, false, true, false) => '└',
                (true, false, false, true) => '┘',
                (false, true, true, false) => '┌',
                (false, true, false, true) => '┐',

                _ => '?',
            };
            print!("{c}")
        }
        println!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn vector(self) -> Pos {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }

    const fn complement(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }


}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
}

impl Pipe {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'L' => Some(Self::NEBend),
            'J' => Some(Self::NWBend),
            '7' => Some(Self::SWBend),
            'F' => Some(Self::SEBend),
            _ => None,
        }
    }

    // Is there an opening facing this direction?
    const fn is_pointing(self, direction: Direction) -> bool {
        match direction {
            Direction::West => matches!(self.points(), [Direction::West, _] | [_, Direction::West]),
            Direction::East => matches!(self.points(), [Direction::East, _] | [_, Direction::East]),
            Direction::North => matches!(self.points(), [Direction::North, _] | [_, Direction::North]),
            Direction::South => matches!(self.points(), [Direction::South, _] | [_, Direction::South]),
        }
    }

    // What directions does this pipe faces?
    const fn points(self) -> [Direction; 2] {
        match self {
            Pipe::Vertical => [Direction::North, Direction::South],
            Pipe::Horizontal => [Direction::East, Direction::West],
            Pipe::NEBend => [Direction::North, Direction::East],
            Pipe::NWBend => [Direction::North, Direction::West],
            Pipe::SWBend => [Direction::South, Direction::West],
            Pipe::SEBend => [Direction::South, Direction::East],
        }
    }

    // Flowing into this pipe from a direction
    fn flow(self, d: Direction) -> Direction {
        let directions = self.points();

        if d == directions[0].complement() {
            directions[1]
        } else if d == directions[1].complement() {
            directions[0]
        } else {
            panic!("invalid flow: {d:?} into {self:?} pipe")
        }
    }
}

type Pos = (i64, i64);

#[derive(Debug, Clone)]
struct Input {
    start: Pos,
    map: BTreeMap<Pos, Pipe>,
    width: i64,
    height: i64,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut start = (0, 0);
        let mut width: i64 = 0;
        let mut height: i64 = 0;
        let mut map = BTreeMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as i64, y as i64);
                if c == 'S' {
                    start = pos;
                }
                if let Some(p) = Pipe::from_char(c) {
                    map.insert(pos, p);
                }
                width = (x + 1) as i64;
            }
            height = (y + 1) as i64;
        }

        Ok(Self { start, map, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part1_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 7107);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 281);
    }
}
