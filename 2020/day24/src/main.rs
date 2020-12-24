use std::path::Path;
use std::fs;
use std::collections::HashSet;
use std::slice::Iter;
use Direction::*;

const NUM_DAYS: usize = 100;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Vec<Direction>> {
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

fn part1(input: &[Vec<Direction>]) -> usize {
    let mut map = Map::new();

    for directions in input {
        map.flip(hexpos(directions));
    }

    map.count()
}

fn part2(input: &[Vec<Direction>]) -> usize {
    let mut map = Map::new();

    // Apply the initial layout
    for directions in input {
        map.flip(hexpos(directions));
    }

    for _ in 0..NUM_DAYS {
        let cur = map.clone();
        for &pos in &cur.black_tiles {
            // Current black tiles
            let count = cur.count_adjacent(pos);
            if count == 0 || count > 2 {
                // Flip to white
                map.black_tiles.remove(&pos);
            }

            // All adjacent white tiles
            let white_tiles = Direction::directions()
                .map(|d| d.offset(pos))
                .filter(|p| !cur.black_tiles.contains(p));
            for pos in white_tiles {
                let count = cur.count_adjacent(pos);
                if count == 2 {
                    // Flip to black
                    map.black_tiles.insert(pos);
                }
            }
        }
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
        match self {
            East => (cur.0 + 1, cur.1),
            SouthEast => (cur.0, cur.1 - 1),
            SouthWest => (cur.0 - 1, cur.1 - 1),
            West => (cur.0 - 1, cur.1),
            NorthWest => (cur.0, cur.1 + 1),
            NorthEast => (cur.0 + 1, cur.1 + 1),
        }
    }

    /// Iterator over all possible directions
    fn directions() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [East, SouthEast, SouthWest, West, NorthWest, NorthEast];
        DIRECTIONS.iter()
    }
}

type Pos = (i64, i64);

#[derive(Debug, Clone)]
struct Map {
    black_tiles: HashSet<Pos>,
}

impl Map {
    fn new() -> Self {
        Map { black_tiles: HashSet::new() }
    }

    /// Flip a tile
    fn flip(&mut self, pos: Pos) {
        if self.black_tiles.contains(&pos) {
            self.black_tiles.remove(&pos);
        } else {
            self.black_tiles.insert(pos);
        }
    }

    /// Count number of black tiles
    fn count(&self) -> usize {
        self.black_tiles.len()
    }

    /// Count number of adjacent black tiles
    fn count_adjacent(&self, pos: Pos) -> usize {
        Direction::directions()
            .filter(|d| self.black_tiles.contains(&d.offset(pos)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexpos1() {
        use Direction::*;
        assert_eq!(hexpos(&[East, SouthEast, West]), (0, -1))
    }

    #[test]
    fn test_hexpos2() {
        use Direction::*;
        assert_eq!(hexpos(&[NorthWest, West, SouthWest, East, East]), (0, 0))
    }

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn test_part2() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input), 2208);
    }
}

