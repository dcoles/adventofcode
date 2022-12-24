//! Advent of Code 2022: Day 22
//! https://adventofcode.com/2022/day/22

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

type Pos = (i32, i32);

const SIZE: i32 = 50; // Face size

// This is the internal cube representation used
//    -1  0  2
// -1    [K]
//  0 [L][T][R]
//  1    [F]
//  2    [B]
const BACK: Pos = (0, -1);
const LEFT: Pos = (-1, 0);
const TOP: Pos = (0, 0);
const RIGHT: Pos = (1, 0);
const FRONT: Pos = (0, 1);
const BOTTOM: Pos = (0, 2);

fn print_map(map: &HashMap<Pos, char>, pos: Pos, direction: Direction) {
    for y in 0..200 {
        for x in 0..200 {
            let c = match direction {
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Up => '^',
            };
            if pos == (x, y) {
                print!("\x1b[91m{}\x1b[0m", c);
            } else {
                print!("{}", map.get(&(x, y)).copied().unwrap_or(' '));
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> i32 {
    let mut pos = input.start;
    let mut direction = Direction::Right;

    for &step in &input.path {
        match step {
            Step::Forward(n) => {
                for _ in 0..n {
                    let (dx, dy) = direction.to_pos();
                    let new_pos = match direction {
                        Direction::Left | Direction::Right => {
                            let x_min = input.map.keys().filter(|&p| p.1 == pos.1).map(|&p| p.0).min().unwrap();
                            let x_max = input.map.keys().filter(|&p| p.1 == pos.1).map(|&p| p.0).max().unwrap();
                            let xlen = x_max - x_min + 1;

                            ((pos.0 + dx - x_min).rem_euclid(xlen) + x_min, pos.1)
                        },
                        Direction::Up | Direction::Down => {
                            let y_min = input.map.keys().filter(|&p| p.0 == pos.0).map(|&p| p.1).min().unwrap();
                            let y_max = input.map.keys().filter(|&p| p.0 == pos.0).map(|&p| p.1).max().unwrap();
                            let ylen = y_max - y_min + 1;

                            (pos.0, (pos.1 + dy - y_min).rem_euclid(ylen) + y_min)
                        },
                    };

                    if input.map[&new_pos] == '#' {
                        break;
                    }

                    pos = new_pos;
                }
            },
            Step::Left => direction = direction.left(),
            Step::Right => direction = direction.right(),
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + direction as i32
}

fn part2(input: &Input) -> i32 {
    let mut pos = (0, 0);
    let mut direction = Direction::Right;
    let valid: HashSet<_> = [(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1), (0, 2)].into_iter().collect();

    for (t, &step) in input.path.iter().enumerate() {
        //println!("{:3}: {:?}", t, step);
        match step {
            Step::Forward(n) => {
                for _ in 0..n {
                    let (dx, dy) = direction.to_pos();
                    let mut new_direction = direction;
                    let mut new_pos = (pos.0 + dx, pos.1 + dy);

                    let old_cell = cell(pos);
                    let new_cell = cell(new_pos);
                    if !valid.contains(&new_cell) {
                        // Remapping required
                        let (xoff, yoff) = norm_pos(new_pos);
                        assert!(xoff == 0 || xoff == 49 || yoff == 0 || yoff == 49, "{:?} xoff: {}, yoff: {}", new_pos, xoff, yoff);
                        assert_eq!(new_cell.0 * SIZE + xoff, new_pos.0);
                        assert_eq!(new_cell.1 * SIZE + yoff, new_pos.1);

                        (new_pos, new_direction) = match (old_cell, new_cell) {
                            (_, (0, -2)) => ((BOTTOM.0 * SIZE + xoff, BOTTOM.1 * SIZE + yoff), Direction::Up), // wrap back to bottom (no rotation)

                            (_, (2, 0)) => ((BOTTOM.0 * SIZE + invert(xoff), BOTTOM.1 * SIZE + invert(yoff)), Direction::Left), // wrap right to bottom (180 deg)
                            (_, (-1, 2)) => ((LEFT.0 * SIZE + invert(xoff), LEFT.1 * SIZE + invert(yoff)), Direction::Right), // wrap bottom to left (180 deg)
                            (_, (-2, 0)) => ((BOTTOM.0 * SIZE + invert(xoff), BOTTOM.1 * SIZE + invert(yoff)), Direction::Right), // wrap left to bottom (180 deg)
                            (_, (1, 2)) => ((RIGHT.0 * SIZE + invert(xoff), RIGHT.1 * SIZE + invert(yoff)), Direction::Left), // wrap bottom to right (180 deg)

                            ((0, -1), (1, -1)) => ((RIGHT.0 * SIZE + invert(yoff), RIGHT.1 * SIZE + xoff), Direction::Down), // wrap back to right (90 deg)
                            ((1, 0), (1, 1)) => ((FRONT.0 * SIZE + invert(yoff), FRONT.1 * SIZE + xoff), Direction::Left), // wrap right to front (90 deg)
                            ((0, 1), (-1, 1)) => ((LEFT.0 * SIZE + invert(yoff), LEFT.1 * SIZE + xoff), Direction::Up), // wrap front to left (90 deg)
                            ((-1, 0), (-1, -1)) => ((BACK.0 * SIZE + invert(yoff), BACK.1 * SIZE + xoff), Direction::Right), // wrap left to back (90 deg)

                            ((0, -1), (-1, -1)) => ((LEFT.0 * SIZE + yoff, LEFT.1 * SIZE + invert(xoff)), Direction::Down), // wrap back to left (-90 deg)
                            ((-1, 0), (-1, 1)) => ((FRONT.0 * SIZE + yoff, FRONT.1 * SIZE + invert(xoff)), Direction::Right), // wrap left to front (-90 deg)
                            ((0, 1), (1, 1)) => ((RIGHT.0 * SIZE + yoff, RIGHT.1 * SIZE + invert(xoff)), Direction::Up), // wrap front to right (-90 deg)
                            ((1, 0), (1, -1)) => ((BACK.0 * SIZE + yoff, BACK.1 * SIZE + invert(xoff)), Direction::Left), // wrap right to back (-90 deg)

                            (_, (0, 3)) => ((BACK.0 * SIZE + xoff, BACK.1 * SIZE + yoff), Direction::Down), // wrap bottom to back (no rotation)

                            _ => panic!("No mapping from {:?} to {:?}", old_cell, new_cell),
                        };
                    }

                    let (map_pos, _) = to_map(new_pos, new_direction);
                    if input.map[&map_pos] == '#' {
                        // Hit a wall
                        break;
                    }

                    pos = new_pos;
                    direction = new_direction;
                }
            },
            Step::Left => direction = direction.left(),
            Step::Right => direction = direction.right(),
        }
    }

    let (map_pos, map_direction) = to_map(pos, direction);
    //print_map(&input.map, map_pos, map_direction);

    1000 * (map_pos.1 + 1) + 4 * (map_pos.0 + 1) + map_direction as i32
}

fn to_map(pos: Pos, direction: Direction) -> (Pos, Direction) {
    // Mapping of map cells
    let cell = cell(pos);
    let map_cell = match cell {
        TOP => (1, 0),
        BOTTOM => (1, 2),
        FRONT => (1, 1),
        BACK => (0, 3),
        LEFT => (0, 2),
        RIGHT => (2, 0),
        _ => panic!("Unmapped cell: {:?}", cell),
    };

    // How are faces oriented
    let (x, y) = norm_pos(pos);
    let new_pos = match cell {
        TOP | BOTTOM | FRONT | RIGHT => (x, y),
        BACK => (invert(y), x),
        LEFT => (invert(x), invert(y)),
        _ => panic!("Unmapped cell: {:?}", cell),
    };

    // How is the direction remapped?
    let map_direction = match cell {
        TOP | BOTTOM | FRONT | RIGHT => direction,
        BACK => match direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        },
        LEFT => match direction {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        },
        _ => panic!("Unmapped cell: {:?}", cell),
    };

    ((map_cell.0 * SIZE + new_pos.0, map_cell.1 * SIZE + new_pos.1), map_direction)
}

fn cell(pos: Pos) -> Pos {
    (pos.0.div_euclid(SIZE), pos.1.div_euclid(SIZE))
}

fn norm_pos(pos: Pos) -> Pos {
    (norm(pos.0), norm(pos.1))
}

fn norm(n: i32) -> i32 {
    n.rem_euclid(SIZE)
}

fn invert(n: i32) -> i32 {
    (SIZE - 1) - norm(n)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn to_pos(self) -> Pos {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn left(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    map: HashMap<(i32, i32), char>,
    path: Vec<Step>,
    start: Pos,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let (m, p) = input.split_once("\n\n").unwrap();

        let mut map = HashMap::new();
        let mut start_pos = None;
        for (y, line) in m.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == ' ' {
                    continue;
                }

                let pos = (x as i32, y as i32);

                if start_pos.is_none() {
                    start_pos = Some(pos);
                }

                map.insert(pos, c);
            }
        }

        let mut path = Vec::new();

        // I hope there is a nicer way than this...
        let p = p.trim();
        let mut start = 0;
        let mut end = 0;
        while start < p.len() {
            // Find numeral string
            while end + 1 <= p.len() && p[start..end + 1].chars().all(|c| c.is_numeric()) {
                end += 1;
            }

            let s = &p[start..end];
            if s.is_empty() {
                break;
            }

            path.push(Step::Forward(s.parse().unwrap()));
            start = end;

            // Find alphabetic string
            while end + 1 < p.len() && p[start..end + 1].chars().all(|c| c.is_alphabetic()) && end + 1 <= p.len() {
                end += 1;
            }

            let s = &p[start..end];
            if s.is_empty() {
                break;
            }

            match s {
                "L" => path.push(Step::Left),
                "R" => path.push(Step::Right),
                x => panic!("Unrecognized turn: {}", x),
            }

            start = end;
        }

        Ok(Input { map, path, start: start_pos.unwrap() })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Step {
    Forward(i32),
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 6032);
    }

    // Unfortunately my code does not handle the general case q.q
    //#[test]
    //fn test_part2() {
    //    let input = Input::from_file("example1.txt").unwrap();
    //
    //    assert_eq!(part2(&input), 0);
    //}
}
