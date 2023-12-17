//! Advent of Code 2023: Day 17 "Clumsy Crucible"
//! https://adventofcode.com/2023/day/17

use std::collections::{BTreeMap, HashMap};
use std::{fs, io};
use std::path::Path;

const MAX_CONSEC: usize = 3;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u32 {
    let start = Pos(0, 0);
    let end = Pos(input.width - 1, input.height - 1);

    let mut edge: Vec<(u32, Entry)> = [(0, Entry { pos: start, direction: Direction::Down, consec: 0}), (0, Entry { pos: start, direction: Direction::Right, consec: 0})].into_iter().collect();
    let mut best_so_far: HashMap<Entry, (u32, usize)> = edge.iter().map(|(_, entry)| (*entry, (0, 0))).collect();

    while let Some((_, cur)) = edge.pop() {
        let (heat_loss, _) = best_so_far[&cur];
        if cur.pos == end {
            return heat_loss;
        }

        for dir in [cur.direction, cur.direction.left(), cur.direction.right()] {
            let adj = cur.pos.to(dir);
            if !input.valid(adj) {
                continue;
            }

            let new_consec = if dir == cur.direction { cur.consec + 1 } else { 1 };
            if new_consec > MAX_CONSEC {
                continue;
            }

            let new = Entry { pos: adj, direction: dir, consec: new_consec };
            let new_loss = heat_loss + input.map[&adj];

            if let Some(&best) = best_so_far.get(&new) {
                if new_loss > best.0 || new_loss == best.0 && new_consec >= best.1 {
                    continue;
                }
            }

            let estimate = ((end.0 - adj.0) + (end.1 - adj.1)) as u32;

            best_so_far.insert(new, (new_loss, new_consec));
            edge.push((new_loss + estimate, new));
        }

        // Sort so lowest loss is at the end (where we will next pop)
        edge.sort_by_key(|(est, _)| u32::MAX - *est);
    }

    panic!("No solution found!")
}

fn part2(input: &Input) -> u32 {
    let start = Pos(0, 0);
    let end = Pos(input.width - 1, input.height - 1);

    let mut edge: Vec<(u32, Entry)> = [(0, Entry { pos: start, direction: Direction::Right, consec: 0}), (0, Entry { pos: start, direction: Direction::Down, consec: 0})].into_iter().collect();
    let mut best_so_far: HashMap<Entry, u32> = edge.iter().map(|(_, entry)| (*entry, 0)).collect();

    while let Some((_, cur)) = edge.pop() {
        let heat_loss = best_so_far[&cur];
        if cur.pos == end {
            return heat_loss;
        }

        for dir in [cur.direction.left(), cur.direction.right()] {
            let mut new_loss = heat_loss;
            for n_blocks in 1..=10 {

                let adj = match dir {
                    Direction::Up => Pos(cur.pos.0, cur.pos.1 - n_blocks),
                    Direction::Down => Pos(cur.pos.0, cur.pos.1 + n_blocks),
                    Direction::Left => Pos(cur.pos.0 - n_blocks, cur.pos.1),
                    Direction::Right => Pos(cur.pos.0 + n_blocks, cur.pos.1),
                };

                new_loss += match input.map.get(&adj) {
                    None => continue,
                    Some(n) => *n,
                };


                if n_blocks < 4 {
                    // Can't stop or turn yet
                    continue;
                }

                let new = Entry { pos: adj, direction: dir, consec: 0 };

                if best_so_far.contains_key(&new) && best_so_far[&new] <= new_loss {
                    continue;
                }

                let estimate = ((end.0 - adj.0) + (end.1 - adj.1)) as u32;

                best_so_far.insert(new, new_loss);
                edge.push((new_loss + estimate, new));
            }
        }

        // Sort so lowest loss is at the end (where we will next pop)
        edge.sort_by_key(|(est, _)| u32::MAX - *est);
    }

    panic!("No solution found!")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    pos: Pos,
    direction: Direction,
    consec: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn to(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Pos(self.0, self.1 - 1),
            Direction::Down => Pos(self.0, self.1 + 1),
            Direction::Left => Pos(self.0 - 1, self.1),
            Direction::Right => Pos(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    map: BTreeMap<Pos, u32>,
    width: i32,
    height: i32,
}

impl Input {
    fn valid(&self, pos: Pos) -> bool {
        (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1)
    }

    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut map = BTreeMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let val = c.to_digit(10).unwrap();

                map.insert(Pos(x as i32, y as i32), val);
                width = width.max(x as i32 + 1);
            }
            height = height.max(y as i32 + 1);
        }

        Ok(Self { map, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 102);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 845);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 94);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 993);
    }
}
