//! Advent of Code 2021: Day 19
//! https://adventofcode.com/2021/day/19

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::ops::Add;
use std::ops::Index;
use std::ops::Sub;
use std::path::Path;

fn main() {
    let input = Input::from_file("day19/input.txt").expect("failed to read input");
    let (rotations, offsets) = solve(&input);

    // Part 1
    println!("Part 1: {}", part1(&input.scans, &rotations, &offsets));

    // Part 2
    println!("Part 2: {}", part2(&offsets));
}

fn part1(scans: &[Vec<Vector>], rotations: &HashMap<usize, Rotation>, offsets: &HashMap<usize, Vector>) -> usize {
    // Beacons in the frame of scanner 0
    let mut beacons = HashSet::new();
    for (n, coords) in scans.iter().enumerate() {
        for &coord in coords {
            beacons.insert(rotate(coord, rotations[&n]) + offsets[&n]);
        }
    }

    beacons.len()
}

fn part2(offsets: &HashMap<usize, Vector>) -> i32 {
    let mut max = 0;
    for i in 0..offsets.len() {
        for j in i+1..offsets.len() {
            let d = offsets[&i].manhattan_distance(&offsets[&j]);
            if d > max {
                max = d;
            }
        }
    }

    max
}

fn solve(input: &Input) -> (HashMap<usize, Rotation>, HashMap<usize, Vector>) {
    // Build up relative distances to all other beacons
    let mut edges: HashMap<((usize, usize), i32), usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), HashSet<i32>> = HashMap::new();
    for (n, coords) in input.scans.iter().enumerate() {
        for i in 0..coords.len() {
            for j in i+1..coords.len() {
                let d = coords[i].distance(&coords[j]);
                distances.entry((n, i)).or_default().insert(d);
                distances.entry((n, j)).or_default().insert(d);
                edges.insert(((n, i), d), j);
                edges.insert(((n, j), d), i);
            }
        }
    }

    // Find common sets of distances
    let distances: Vec<_> = distances.into_iter().collect();
    let mut common = Vec::new();
    for i in 0..distances.len() {
        for j in i+1..distances.len() {
            let a = distances[i].0;
            let b = distances[j].0;
            if a.0 == b.0 {
                continue;
            }

            let s = &distances[i].1 & &distances[j].1;
            if s.len() + 1 >= 12 {
                common.push(((a, b), s));
            }
        }
    }

    common.sort_by_key(|&((a, b), _)| (a, b));

    // Resolve to indexes
    let mut resolved = Vec::new();
    for ((a, b), s) in common {
        let mut m = HashSet::new();
        for d in s {
            let a_ = edges[&(a, d)];
            let b_ = edges[&(b, d)];
            m.insert((a_, b_));
        }
        resolved.push(((a, b), m));
    }

    // Solve rotations and offsets
    let mut rotations: HashMap<usize, Rotation> = [(0, Rotation([0, 0, 0]))].into_iter().collect();
    let mut offsets: HashMap<usize, Vector> = [(0, Vector([0, 0, 0]))].into_iter().collect();
    while rotations.len() != input.scans.len() {
        for ((a, b), m) in resolved.iter() {
            if !rotations.contains_key(&a.0) || rotations.contains_key(&b.0) {
                continue;
            }

            // We only need one adjacent point
            let &(an, bn) = m.iter().next().unwrap();
            let v1 = rotate(input.scans[a.0][an] - input.scans[a.0][a.1], rotations[&a.0]);
            let v2 = input.scans[b.0][bn] - input.scans[b.0][b.1];

            let rotation = find_rotation(v1, v2).unwrap();
            rotations.insert(b.0, rotation);

            // Solve offset
            let a0 = rotate(input.scans[a.0][a.1], rotations[&a.0]);
            let b0 = rotate(input.scans[b.0][b.1], rotations[&b.0]);
            let offset = offsets[&a.0] + a0 - b0;

            offsets.insert(b.0, offset);
        }
    }

    (rotations, offsets)
}

fn find_rotation(v1: Vector, v2: Vector) -> Option<Rotation> {
    for rz in 0..4 {
        for ry in 0..4 {
            for rx in 0..4 {
                let vx = rotate(v2, Rotation([rx, ry, rz]));
                if v1 == vx {
                    return Some(Rotation([rx, ry, rz]));
                }
            }
        }
    }

    None
}

fn rotate(mut coord: Vector, rotation: Rotation) -> Vector {
    for _ in 0..rotation[0] {
        coord = coord.rotate_x();
    }

    for _ in 0..rotation[1] {
        coord = coord.rotate_y();
    }

    for _ in 0..rotation[2] {
        coord = coord.rotate_z();
    }

    coord
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector([i32; 3]);

impl Vector {
    fn distance(&self, c: &Vector) -> i32 {
        //(self.0[0] - c[0]).pow(2) + (self.0[1] - c[1]).pow(2) + (self.0[2] - c[2]).pow(2)
        (0..3).into_iter().map(|i| (self[i] - c[i]).pow(2)).sum()
    }

    fn manhattan_distance(&self, c: &Vector) -> i32 {
        (0..3).into_iter().map(|i| (self[i] - c[i]).abs()).sum()
    }

    fn rotate_x(self) -> Vector {
        Vector([self.0[0], -self.0[2], self.0[1]])
    }

    fn rotate_y(self) -> Vector {
        Vector([self.0[2], self.0[1], -self.0[0]])
    }

    fn rotate_z(self) -> Vector {
        Vector([-self.0[1], self.0[0], self.0[2]])
    }
}

impl Index<usize> for Vector {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1], self.0[2] - rhs.0[2]])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rotation([usize; 3]);

impl Index<usize> for Rotation {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug, Clone)]
struct Input {
    scans: Vec<Vec<Vector>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut scans = Vec::new();
        for chunk in input.split("\n\n") {
            let mut positions = Vec::new();
            for line in chunk.lines().skip(1) {
                let mut split = line.split(",");
                let a = split.next().unwrap().parse().unwrap();
                let b = split.next().unwrap().parse().unwrap();
                let c = split.next().unwrap().parse().unwrap();

                positions.push(Vector([a, b, c]));
            }
            scans.push(positions);
        }


        Ok(Input { scans })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").expect("failed to read input");
        let (rotations, offsets) = solve(&input);

        assert_eq!(part1(&input.scans, &rotations, &offsets), 79);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").expect("failed to read input");
        let (_, offsets) = solve(&input);

        assert_eq!(part2(&offsets), 3621);
    }
}
