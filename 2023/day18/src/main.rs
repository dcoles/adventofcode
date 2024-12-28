//! Advent of Code 2023: Day 18 "Lavaduct Lagoon"
//! https://adventofcode.com/2023/day/18

use std::collections::HashSet;
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut edge_tiles: HashSet<(i64, i64)> = [(x, y)].into_iter().collect();

    for (direction, distance, _) in &input.values {
        for _ in 0..*distance {
            match direction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }

            edge_tiles.insert((x, y));
        }
    }

    let mut flood_tiles = HashSet::new();
    let mut edge = vec![(1, 1)];
    while let Some(pos) = edge.pop() {
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let adj = (pos.0 + dx, pos.1 + dy);
            if edge_tiles.contains(&adj) || flood_tiles.contains(&adj) {
                continue;
            }

            edge.push(adj);
            flood_tiles.insert(adj);
        }
    }

    edge_tiles.len() + flood_tiles.len()
}

fn part2(input: &Input) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut coords = vec![(x, y)];


    /*
    for (_, _, color) in &input.values {
        let distance = i64::from_str_radix(&color[1..6], 16).unwrap();
        match &color[6..7] {
            "0" => x += distance, // Right,
            "1" => y += distance, // Down
            "2" => x -= distance, // Left
            "3" => y -= distance, // Up
            s => panic!("unknown directional ordinal: {s:?}"),
        }

        coords.push((x, y));
    }
    */
    for (direction, distance, _) in &input.values {
        match direction {
            Direction::Up => y -= distance,
            Direction::Down => y += distance,
            Direction::Left => x -= distance,
            Direction::Right => x += distance,
        }

        coords.push((x, y));
    }

    assert_eq!(coords.first().unwrap(), coords.last().unwrap());

    let forward: Vec<(i64, i64)> = coords.iter().copied().collect();
    let backwards: Vec<(i64, i64)> = coords.iter().rev().copied().collect();

    let mut i = 0;
    let mut j = 0;

    let mut begin: Pos = (0, 0);
    let mut a: Pos = (0, 0);
    let mut b: Pos = (0, 0);
    let mut total_area = 0;


    loop {
        while a.0 - begin.0 == 0 || a.1 - begin.1 == 0 {
            i += 1;
            a = forward[i];
        }

        while b.0 - begin.0 == 0 || b.1 - begin.1 == 0 {
            j += 1;
            b = backwards[j];
        }

        let dx_a = a.0 - begin.0;
        let dx_b = b.0 - begin.0;
        let dy_a = a.1 - begin.1;
        let dy_b = b.1 - begin.1;

        let dx = dx_a;
        let dy = if dy_a.abs() < dy_b.abs() { dy_a } else { dy_b };

        println!("dx {dx}, dy {dy}");

        total_area += (dx.abs() + 1) * (dy.abs() + 1);

        begin = (begin.0 + dx_b + dx_b.signum(), begin.1 + dy + dy.signum());

        println!("{}", total_area);

        if total_area > 10 {
            break;
        }
    }

    0
}

type Pos = (i64, i64);

fn delta(x1: (i64, i64), x0: (i64, i64)) -> (i64, i64) {
    (x1.0 - x0.0, x1.1 - x0.1)
}

fn area(x0: Pos, x1: Pos) -> i64 {
    (x1.0 - x0.0).abs() * (x1.1 - x0.1).abs()
}


#[derive(Debug, Clone)]
struct Input {
    /// `(direction, distance, color)`
    values: Vec<(Direction, i64, String)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = vec![];
        for line in input.lines() {
            let mut iter = line.split(' ');
            let direction = Direction::from_str(iter.next().unwrap());
            let distance = iter.next().unwrap().parse::<i64>().unwrap();
            let color = iter.next().unwrap()[1..8].to_string();

            values.push((direction, distance, color));
        }

        Ok(Self { values })
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
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Unknown direction: {s:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 62573);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 0);
    }
}
