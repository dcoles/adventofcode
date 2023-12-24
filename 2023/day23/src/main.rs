//! Advent of Code 2023: Day 23 "A Long Walk"
//! https://adventofcode.com/2023/day/23

use std::collections::{BTreeMap, HashMap, HashSet};
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let start = *input.map.keys().filter(|p| p.1 == 0 && matches!(input.map[&p], Tile::Path)).next().unwrap();
    let end = *input.map.keys().filter(|p| p.1 == (input.height - 1) && matches!(input.map[&p], Tile::Path)).next().unwrap();

    let mut all_paths = vec![];

    let mut edge = vec![vec![start]];
    while let Some(path) = edge.pop() {
        let cur = *path.last().unwrap();

        if cur == end {
            all_paths.push(path);
            continue;
        }

        for adj in input.adjacent(cur).filter(|&p| !path.contains(&p)) {
            let mut path = path.clone();
            path.push(adj);

            // Check if we stepped onto a slippery slope
            match input.map[&adj] {
                Tile::Slope(direction) => {
                    let downhill = match direction {
                        Direction::North => Pos(adj.0, adj.1 - 1),
                        Direction::South => Pos(adj.0, adj.1 + 1),
                        Direction::East => Pos(adj.0 + 1, adj.1),
                        Direction::West => Pos(adj.0 - 1, adj.1),
                    };

                    if !input.is_valid(downhill) || path.contains(&downhill) {
                        continue;
                    }

                    path.push(downhill);
                }
                _ => (),
            };

            edge.push(path);
        }
    }

    all_paths.into_iter().map(|path| path.len() - 1).max().unwrap()
}

fn part2(input: &Input) -> usize {
    let start = *input.map.keys().filter(|p| p.1 == 0 && matches!(input.map[&p], Tile::Path)).next().unwrap();
    let end = *input.map.keys().filter(|p| p.1 == (input.height - 1) && matches!(input.map[&p], Tile::Path)).next().unwrap();


    let mut nodes = vec![start, end];
    for &pos in input.map.keys().filter(|&pos| !input.map[pos].is_forrest()) {
        if input.adjacent(pos).count() > 2 {
            nodes.push(pos);
        }
    }

    let mut adjacency: HashMap<Pos, HashMap<Pos, usize>> = nodes.iter().copied().map(|p| (p, Default::default())).collect();
    for &pos in nodes.iter() {
        let mut seen: HashSet<_> = [pos].into_iter().collect();
        let mut edge = vec![(pos, 0)];
        while let Some((cur, n)) = edge.pop() {
            for adj in input.adjacent(cur) {
                if seen.contains(&adj) {
                    continue;
                }

                if adjacency.contains_key(&adj) {
                    adjacency.entry(pos).or_default().insert(adj, n + 1);
                    adjacency.entry(adj).or_default().insert(pos, n + 1);
                } else {
                    edge.push((adj, n + 1));
                }

                seen.insert(adj);
            }
        }
    }

    let mut longest_path = 0;
    let mut edge = vec![vec![start]];
    while let Some(path) = edge.pop() {
        let cur = *path.last().unwrap();

        if cur == end {
            longest_path = longest_path.max(path.windows(2).map(|pos| adjacency[&pos[0]][&pos[1]]).sum());
            continue;
        }

        for adj in adjacency[&cur].keys().copied().filter(|&p| !path.contains(&p)) {
            edge.push(path.iter().copied().chain([adj]).collect());
        }
    }

    longest_path
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Direction)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Direction::North),
            'v' => Self::Slope(Direction::South),
            '>' => Self::Slope(Direction::East),
            '<' => Self::Slope(Direction::West),
            _ => panic!("unknown tile {c:?}"),
        }
    }

    fn is_forrest(&self) -> bool {
        matches!(self, Self::Forest)
    }
}

#[derive(Debug, Clone)]
struct Input {
    map: BTreeMap<Pos, Tile>,
    width: i64,
    height: i64,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut map: BTreeMap<Pos, Tile> = BTreeMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.insert(Pos(x as i64, y as i64), Tile::from_char(c));
                width = width.max(x as i64 + 1);
            }
            height = height.max(y as i64 + 1);
        }

        Ok(Self { map, width, height })
    }

    fn adjacent(&self, pos: Pos) -> impl Iterator<Item=Pos> + '_ {
        [Pos(pos.0, pos.1 + 1), Pos(pos.0, pos.1 - 1), Pos(pos.0 + 1, pos.1), Pos(pos.0 - 1, pos.1)].into_iter().filter(|&p| self.is_valid(p))
    }

    fn is_valid(&self, pos: Pos) -> bool {
        (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) && !self.map[&pos].is_forrest()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 94);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 2250);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 154);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 6470);
    }
}
