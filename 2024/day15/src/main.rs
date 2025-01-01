//! Advent of Code 2024: Day 15
//! https://adventofcode.com/2024/day/15

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use lib::vector::Vector;

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
    let mut robot = input.map.iter().find_map(|(&pos, &c)| (c == '@').then_some(pos)).unwrap();
    let walls: BTreeSet<Vec2> = input.map.iter().filter_map(|(&pos, &c)| (c == '#').then_some(pos)).collect();
    let mut boxes: BTreeSet<Vec2> = input.map.iter().filter_map(|(&pos, &c)| (c == 'O').then_some(pos)).collect();

    for movement in input.input.iter().copied() {
        let d = Vec2::from(movement);
        let new_pos = robot + d;

        if walls.contains(&new_pos) {
            // Bonk!
            continue;
        }

        if boxes.contains(&new_pos) {
            // Can we push the box?
            let mut p = new_pos + d;
            let can_move = loop {
                if walls.contains(&p) {
                    break false;
                }
                if !boxes.contains(&p) {
                    break true;
                }
                p += d;
            };

            if !can_move {
                continue;
            }

            boxes.remove(&new_pos);
            boxes.insert(p);
        }

        robot = new_pos;
    }

    boxes.into_iter().map(|b| (100 * b[1] + b[0]) as usize).sum()
}

fn double_width(pos: Vec2) -> Vec2 {
    Vec2::new([2 * pos[0], pos[1]])
}

fn part2(input: &Input) -> usize {
    let mut robot = input.map.iter().find_map(|(&pos, &c)| (c == '@').then_some(double_width(pos))).unwrap();
    let walls: BTreeSet<Vec2> = input.map.iter().filter_map(|(&pos, &c)| (c == '#').then_some(double_width(pos))).collect();
    let mut boxes: BTreeSet<Vec2> = input.map.iter().filter_map(|(&pos, &c)| (c == 'O').then_some(double_width(pos))).collect();

    'outer: for movement in input.input.iter().copied() {
        //draw(&walls, &boxes, robot);
        //println!("Move {movement:?}");

        let d = Vec2::from(movement);
        let new_pos = robot + d;

        if walls.contains(&new_pos) || walls.contains(&(new_pos - Vec2::new([1, 0]))) {
            // Bonk!
            continue 'outer;
        }

        let mut moving_boxes: BTreeSet<Vec2> = has_box(&boxes, new_pos).into_iter().collect();
        let mut visited = vec![];
        while let Some(a_box) = moving_boxes.pop_first() {
            visited.push(a_box);
            for offset in [Vec2::new([0, 0]), Vec2::new([1, 0])] {
                let new_box_pos = a_box + offset + d;
                if walls.contains(&new_box_pos) || walls.contains(&(new_box_pos - Vec2::new([1, 0]))) {
                    // Bonk!
                    continue 'outer;
                }

                if let Some(another_box) = has_box(&boxes, new_box_pos) {
                    if a_box != another_box {
                        moving_boxes.insert(another_box);
                    }
                }
            }
        }

        for &a_box in visited.iter() {
            boxes.remove(&a_box);
        }

        for &a_box in visited.iter() {
            boxes.insert(a_box + d);
        }

        robot = new_pos;
    }

    draw(&walls, &boxes, robot);

    boxes.into_iter().map(|b| (100 * b[1] + b[0]) as usize).sum()
}

fn has_box(boxes: &BTreeSet<Vec2>, pos: Vec2) -> Option<Vec2> {
    for offset in [Vec2::new([0, 0]), Vec2::new([-1, 0])] {
        if boxes.contains(&(pos + offset)) {
            return Some(pos + offset);
        }
    }

    None
}

fn draw(walls: &BTreeSet<Vec2>, boxes: &BTreeSet<Vec2>, robot: Vec2) {
    let mut max_x = 0;
    let mut max_y = 0;
    for pos in walls {
        max_x = max_x.max(pos[0] + 1);
        max_y = max_y.max(pos[1]);
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Vec2::new([x, y]);
            if pos == robot {
                print!("@");
            } else if walls.contains(&pos) || walls.contains(&(pos - Vec2::new([1, 0]))) {
                print!("#");
            } else if boxes.contains(&pos) {
                print!("[");
            } else if boxes.contains(&(pos - Vec2::new([1, 0]))) {
                print!("]");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

type Vec2 = Vector<i32, 2>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<Move> for Vec2 {
    fn from(value: Move) -> Self {
        match value {
            Move::Up => Vec2::new([0, -1]),
            Move::Down => Vec2::new([0, 1]),
            Move::Left => Vec2::new([-1, 0]),
            Move::Right => Vec2::new([1, 0]),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            c => Err(c),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    map: BTreeMap<Vec2, char>,
    input: Vec<Move>
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let (chunk1, chunk2) = input.split_once("\n\n").unwrap();

        let mut width = 0;
        let mut height = 0;
        let mut map = BTreeMap::new();
        for (y, line) in chunk1.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let pos = Vec2::new([x as i32, y as i32]);

                map.insert(pos, c);
                width = width.max(x as i32 + 1);
            }
            height = height.max(y as i32 + 1);
        }

        let mut input = Vec::new();
        for line in chunk2.lines() {
            for c in line.trim().chars() {
                let movement = Move::try_from(c).unwrap();

                input.push(movement);
            }
        }

        Ok(Self { map, input })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 10092);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1495147);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 9021);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 1524905);
    }
}
