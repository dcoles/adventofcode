//! Advent of Code 2022: Day 24
//! https://adventofcode.com/2022/day/24

use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::ops::Add;
use std::path::Path;
use std::rc::Rc;

const DEBUG: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn distance(self, other: Pos) -> u32 {
        other.0.abs_diff(self.0) + other.1.abs_diff(self.1)
    }
}

impl Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {
    let input = Valley::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));
    println!();

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Valley) -> usize {
    let input = input.clone();
    let mut edge = vec![(0, input.start)];
    let mut seen: HashMap<(usize, Pos), i32> = [(edge[0], 0)].into_iter().collect();

    while let Some((t, pos)) = edge.pop() {
        if pos == input.end {
            println!("Reached end at t={} {:?}", t, pos);
            input.print_map(t, pos);
            return t;
        }

        if DEBUG {
            println!("@{} {:?}", t, pos);
            input.print_map(t, pos);
        }

        let new_t = t + 1;
        let blizards = input.at(new_t);
        for dir in [Direction::None, Direction::North, Direction::West, Direction::East, Direction::South] {
            let new_pos = pos + dir.vector();
            if !input.is_valid_pos(new_pos) {
                // This isn't a valid position
                continue
            }

            if seen.contains_key(&(new_t, new_pos)) {
                // We've seen this state
                continue;
            }
            seen.insert((new_t, new_pos), 0);

            if blizards.iter().find(|(p, _)| *p == new_pos).is_some() {
                // There would be a blizard here
                continue;
            }

            edge.push((new_t, new_pos));
        }

        // Order by time taken + estimate remaining steps
        edge.sort_by_key(|&(t, pos)| -(t as i32 + pos.distance(input.end) as i32));
    }

    panic!("No solution found!");
}

fn part2(input: &Valley) -> usize {
    // Cache of blizard positions
    let input = input.clone();
    let mut current_trip = 3;
    let mut edge = vec![(0, current_trip, input.start)];
    let mut seen: HashSet<(usize, u32, Pos)> = [edge[0]].into_iter().collect();

    while let Some((t, trip, pos)) = edge.pop() {
        if trip > current_trip {
            // We already know the best time for that trip
            continue;
        } else if trip == 1 && pos == input.end {
            // We finished all the trips
            println!("Reached end at t={} {:?}", t, pos);
            input.print_map(t, pos);
            return t;
        }
        current_trip = trip;

        if DEBUG {
            println!("@{} {:?}", t, pos);
            input.print_map(t, pos);
        }

        let new_t = t + 1;
        let blizards = input.at(new_t);
        for dir in [Direction::None, Direction::North, Direction::West, Direction::East, Direction::South] {
            let new_pos = pos + dir.vector();
            if !input.is_valid_pos(new_pos) {
                // This isn't a valid position
                continue
            }

            let mut new_trip = trip;
            if (new_trip == 3 && new_pos == input.end) || (new_trip == 2 && new_pos == input.start) {
                // We completed a trip!
                new_trip -= 1;
            }

            let new_state = (new_t, new_trip, new_pos);
            if seen.contains(&new_state) {
                // We've seen this state
                continue;
            }
            seen.insert(new_state);

            if blizards.iter().find(|(p, _)| *p == new_pos).is_some() {
                // There would be a blizard here
                continue;
            }

            edge.push(new_state);
        }

        // Order by time taken + estimate remaining steps
        let min_trip_time = input.start.distance(input.end);
        edge.sort_by_key(|&(t, trip, pos)| {
            // Change the waypoint based on the current trip
            let waypoint = if trip % 2 == 1 {
                input.end
            } else {
                input.start
            };

            // Cur time spent + estimated distance remaining
            -((t as u32 + (trip - 1) * min_trip_time + pos.distance(waypoint)) as i32)
        });
    }

    panic!("No solution found!");
}

type Blizard = (Pos, Direction);

#[derive(Debug, Clone)]
struct Valley {
    walls: HashSet<Pos>,
    start: Pos,
    end: Pos,
    min: Pos,
    max: Pos,

    // Cache of blizard positions
    cache: RefCell<Vec<Rc<[Blizard]>>>,
}

impl Valley {
    fn is_valid_pos(&self, pos: Pos) -> bool {
        (pos.0 >= self.min.0 && pos.1 >= self.min.1 && pos.0 <= self.max.0 && pos.1 <= self.max.1)
        && !self.walls.contains(&pos)
    }

    fn at(&self, t: usize) -> Rc<[Blizard]> {
        let mut cache = self.cache.borrow_mut();
        let n = cache.len();
        for t in n..=t {
            let mut state = Vec::new();

            for &(bliz_pos, bliz_dir) in &*cache[t - 1] {
                let mut new_bliz_pos = bliz_pos + bliz_dir.vector();

                if self.walls.contains(&new_bliz_pos) {
                    new_bliz_pos = match bliz_dir {
                        Direction::North => Pos(bliz_pos.0, self.max.1 - 1),
                        Direction::South => Pos(bliz_pos.0, self.min.1 + 1),
                        Direction::East => Pos(self.min.0 + 1, bliz_pos.1),
                        Direction::West => Pos(self.max.0 - 1, bliz_pos.1),
                        Direction::None => unreachable!("Blizards always move"),
                    };
                }

                state.push((new_bliz_pos, bliz_dir));
            }

            cache.push(Rc::from(state));
        }

        Rc::clone(&cache[t])
    }

    fn print_map(&self, t: usize, you_are_here: Pos) {
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                let pos = Pos(x, y);

                let blds: Vec<_> = self.at(t).iter().copied().filter(|(p, _)| *p == pos).collect();
                if pos == you_are_here {
                    // Yes, you are!
                    print!("\x1b[31m@\x1b[0m");
                } else if !blds.is_empty() {
                    if blds.len() > 1 {
                        // Show count of overlapping blizards
                        print!("\x1b[33m{}\x1b[0m", blds.len());
                    } else {
                        // Show blizard detail
                        let c = match blds[0].1 {
                            Direction::None => '@',
                            Direction::North => '^',
                            Direction::South => 'v',
                            Direction::East => '>',
                            Direction::West => '<',
                        };

                        print!("\x1b[36m{}\x1b[0m", c);
                    }
                } else if self.walls.contains(&pos) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut walls = HashSet::new();
        let mut blizards = Vec::new();
        let mut start = None;
        let mut end = None;
        let lines: Vec<_> = input.lines().collect();
        for (y, &line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos(x as i32, y as i32);

                match c {
                    '#' => { walls.insert(pos); },
                    '^' => { blizards.push((pos, Direction::North)); },
                    'v' => { blizards.push((pos, Direction::South)); },
                    '>' => { blizards.push((pos, Direction::East)); },
                    '<' => { blizards.push((pos, Direction::West)); },
                    '.' if y == 0 => { start = Some(pos); },
                    '.' if y == lines.len() - 1 => { end = Some(pos) },
                    _ => (),
                }
            }
        }

        let min_x = walls.iter().copied().map(|p| p.0).min().unwrap();
        let max_x = walls.iter().copied().map(|p| p.0).max().unwrap();
        let min_y = walls.iter().copied().map(|p| p.1).min().unwrap();
        let max_y = walls.iter().copied().map(|p| p.1).max().unwrap();

        let cache = vec![Rc::from(blizards)];
        Ok(Valley { walls, start: start.unwrap(), end: end.unwrap(), min: Pos(min_x, min_y), max: Pos(max_x, max_y), cache: RefCell::new(cache) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    fn vector(self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Valley::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = Valley::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 54);
    }
}
