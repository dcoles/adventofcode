//! Advent of Code 2023: Day 24 "Never Tell Me The Odds"
//! https://adventofcode.com/2023/day/24

use std::ops::RangeInclusive;
use std::{fs, io};
use std::path::Path;

const T_MIN: f64 = 0.0;
const RANGE: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;

fn main() {
    //let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    intersections_within_area(input, RANGE)
}

fn part2(input: &Input) -> usize {
    0
}

fn intersections_within_area(input: &Input, range: RangeInclusive<f64>) -> usize {
    let mut n = 0;

    for i in 0..input.values.len() {
        for j in (i + 1)..input.values.len() {
            let a = input.values[i];
            let b = input.values[j];
            let (intersect, (t_a, t_b)) = solve_xy_intersection(a, b);

            println!("{a:?} {b:?}: {intersect:?} @ {t_a}/{t_b} ns");
            if range.contains(&intersect.x) && range.contains(&intersect.y) && t_a > T_MIN && t_b > T_MIN {
                n += 1;
            }
        }
    }

    n
}

fn solve_xy_intersection((p_a, v_a): (Vec3, Vec3), (p_b, v_b): (Vec3, Vec3)) -> (Vec3, (f64, f64)) {
    let dydx_a = v_a.y / v_a.x;
    let dzdx_a = v_a.z / v_a.x;
    let dydx_b = v_b.y / v_b.x;

    let x = ((p_a.x * dydx_a - p_a.y) - (p_b.x * dydx_b - p_b.y)) / (dydx_a - dydx_b);
    let y = dydx_a * (x - p_a.x) + p_a.y;
    let z = dzdx_a * (x - p_a.x) + p_a.z;

    let t_a = (x - p_a.x) / v_a.x;
    let t_b = (x - p_b.x) / v_b.x;

    (Vec3 { x, y, z }, (t_a, t_b))
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split(',');

        let x = iter.next().and_then(|s| s.trim().parse().ok()).unwrap();
        let y = iter.next().and_then(|s| s.trim().parse().ok()).unwrap();
        let z = iter.next().and_then(|s| s.trim().parse().ok()).unwrap();

        Vec3 { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(Vec3, Vec3)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = vec![];
        for line in input.lines() {
            let (position, velocity) = line.split_once('@').unwrap();
            let position = Vec3::from_str(position);
            let velocity = Vec3::from_str(velocity);

            values.push((position, velocity));
        }

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(intersections_within_area(&input, 7.0..=27.0), 2);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 16502);
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
