use std::path::Path;
use std::fs;
use crate::Direction::{SouthEast, SouthWest, NorthEast, NorthWest, East, West};
use std::collections::HashMap;

type Input = Vec<Vec<Direction>>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");

    input.lines()
        .map(|line| parse_directions(&line))
        .collect()
}

fn parse_directions(s: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    let mut i = 0;
    while i < s.len() {
        directions.push(
            match &s[i..i+1] {
                "e" => East,
                "w" => West,
                "s" => {
                    i += 1;
                    match &s[i..i + 1] {
                        "e" => SouthEast,
                        "w" => SouthWest,
                        x => panic!("Unknown direction: s{}", x),
                    }
                }
                "n" => {
                    i += 1;
                    match &s[i..i + 1] {
                        "e" => NorthEast,
                        "w" => NorthWest,
                        x => panic!("Unknown direction: n{}", x),
                    }
                }
                x => panic!("Unknown direction: {}", x),
            }
        );
        i += 1;
    }

    directions
}

fn part1(input: &Input) -> usize {
    let mut map = Map::new();

    for directions in input {
        map.flip(hexpos(directions));
    }

    map.count()
}

/// Calculate the position from the origin.
fn hexpos(directions: &[Direction]) -> Pos {
    let mut pos = (0, 0);
    for d in directions {
        pos = d.offset(pos);
    }

    pos
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn offset(&self, cur: Pos) -> Pos {
        use Direction::*;
        match self {
            East => (cur.0 + 1, cur.1),
            SouthEast => (cur.0, cur.1 - 1),
            SouthWest => (cur.0 - 1, cur.1 - 1),
            West => (cur.0 - 1, cur.1),
            NorthWest => (cur.0, cur.1 + 1),
            NorthEast => (cur.0 + 1, cur.1 + 1),
        }
    }
}

type Pos = (i64, i64);

struct Map {
    tiles: HashMap<Pos, bool>,
}

impl Map {
    fn new() -> Self {
        Map { tiles: HashMap::new() }
    }

    /// Flip a tile
    fn flip(&mut self, pos: Pos) {
        self.tiles.entry(pos).and_modify(|t| *t = !*t).or_insert(true);
    }

    /// Count number of black tiles
    fn count(&self) -> usize {
        self.tiles.values().filter(|&&t| t).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexpos1() {
        assert_eq!(hexpos(&[East, SouthEast, West]), (0, -1))
    }

    #[test]
    fn test_hexpos2() {
        assert_eq!(hexpos(&[NorthWest, West, SouthWest, East, East]), (0, 0))
    }

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 10);
    }
}

