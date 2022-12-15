//! Advent of Code 2022: Day 15
//! https://adventofcode.com/2022/day/15

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

type Point = (i64, i64);

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input, 2000000));

    // Part 2
    const SIZE: i64 = 4000000;
    let becon = part2(&input, SIZE);
    println!("Part 2: {} Hz", tuning_frequency(becon, SIZE));
}

fn part1(input: &Input, y: i64) -> i64 {
    let becons: HashSet<_> = input.values.iter().filter_map(|s| (s.closest.1 == y).then_some(s.closest)).collect();
    
    let spans = scan(input, y);
    let num_positions: i64 = spans.into_iter().map(|(x1, x2)| x2 - x1 + 1).sum();

    // Becons can't be valid positions
    num_positions - becons.len() as i64
}

fn part2(input: &Input, size: i64) -> Point {
    let becons: HashSet<_> = input.values.iter().map(|sensor| sensor.closest).collect();

    for y in 0..=size {
        for span in scan(input, y).windows(2) {
            let (_, s1_end) = span[0];
            let (s2_start, _) = span[1];
 
            // Look for a 1-unit gap
            if (s2_start - 1) - s1_end == 1 {
                let x = s1_end + 1;

                // Make sure it's a valid position
                if x >= 0 && x <= size && !becons.contains(&(x, y)) {
                    return (x, y);
                }
            } 

        }
    }

    panic!("Becon not found!");
}

/// Calculate the tuning frequency.
fn tuning_frequency(pos: Point, size: i64) -> i64 {
    pos.0 * size + pos.1
}

/// Determine the ranges visible in `row`.
fn scan(input: &Input, row: i64) -> Vec<Point> {
    let mut ranges = Vec::new();
    for sensor in &input.values {
        let d = distance(sensor.position, sensor.closest);
        let d2 = distance(sensor.position, (sensor.position.0, row));
        if d2 < d {
            ranges.push((sensor.position.0 - (d - d2), sensor.position.0 + (d - d2)));
        }
    }

    ranges.sort();

    let mut spans: Vec<Point> = Vec::new();

    for i in 0..ranges.len() {
        if spans.last().is_some() && spans.last().unwrap().1 >= ranges[i].0 {
            continue;
        }

        let mut span = (ranges[i].0, ranges[i].1);
        for j in i..ranges.len() {
            // Extend this span
            if ranges[j].0 <= span.1 && ranges[j].1 > span.1 {
                span = (span.0, ranges[j].1)
            }
        }

        spans.push(span);
    }

    spans
}

/// Distance between points.
fn distance(p1: Point, p2: Point) -> i64 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Sensor>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let re = regex::Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

        let mut values = Vec::new();
        for line in input.lines() {
            let m = re.captures(line).unwrap();

            let x1: i64 = m[1].parse().unwrap();
            let y1: i64 = m[2].parse().unwrap();
            let x2: i64 = m[3].parse().unwrap();
            let y2: i64 = m[4].parse().unwrap();

            values.push(Sensor { position: (x1, y1), closest: (x2, y2) });
        }

        Ok(Input { values })
    }
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Point,
    closest: Point,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        let pos = part2(&input, 20);
        assert_eq!(pos, (14, 11));
        assert_eq!(tuning_frequency(pos, 4000000), 56000011);
    }
}
