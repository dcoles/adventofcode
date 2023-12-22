//! Advent of Code 2023: Day 22 "Sand Slabs"
//! https://adventofcode.com/2023/day/22

use std::collections::{HashMap, HashSet};
use std::{fs, io};
use std::path::Path;

const Z_GROUND: i32 = 0;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let (bricks, supports, supported_by) = simulate(input);

    (0..bricks.len()).filter(|&i| {
        // Bricks where all the bricks supported by this brick are supported by at least one other brick
        supports[&i].iter().all(|&j| supported_by[&j].iter().filter(|&&k| k != i).count() > 0)
    }).count()
}

/// Returns a tuple of `(bricks, supports, supported_by)`
fn simulate(input: &Input) -> (Vec<Brick>, HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let mut bricks = input.values.clone();
    bricks.sort_by_key(|b| b.start.z);

    let mut supports: HashMap<usize, HashSet<usize>> = (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut supported_by: HashMap<usize, HashSet<usize>> = (0..bricks.len()).map(|i| (i, HashSet::new())).collect();

    for i in 0..bricks.len() {
        let mut brick = bricks[i];
        let mut z_gaps = vec![];
        for j in 0..i {
            let other_brick = bricks[j];

            if brick.overlap_xy(&other_brick) {
                let z_gap = brick.start.z - (other_brick.end.z + 1);

                z_gaps.push((j, z_gap));
            }
        }

        let z_gap = z_gaps.iter().map(|&(_, z)| z).min().unwrap_or(brick.start.z - (Z_GROUND + 1));
        for j in z_gaps.iter().filter(|&&(_, z)| z == z_gap).map(|&(j, _)| j) {
            // Block `j`` supports this block (Block `i`)
            supports.get_mut(&j).unwrap().insert(i);
            supported_by.get_mut(&i).unwrap().insert(j);
        }

        brick.start.z -= z_gap;
        brick.end.z -= z_gap;

        bricks[i] = brick;
    }

    (bricks, supports, supported_by)
}

fn part2(input: &Input) -> usize {
    let (bricks, _supports, supported_by) = simulate(input);

    let mut would_fall: HashMap<usize, usize> = HashMap::new();
    for i in 0..bricks.len() {
        // Assume this brick is removed
        let mut falling: HashSet<_> = [i].into_iter().collect();

        let mut num_falling = 0;
        while num_falling != falling.len() {
            num_falling = falling.len();

            // Are there any new unsupported bricks that would now be falling?
            for (&i, supported_by) in &supported_by {
                // A brick can only be falling if it's off the ground and all the bricks that supported it are also falling
                if !falling.contains(&i) && !bricks[i].is_on_ground() && supported_by.iter().all(|&j| falling.contains(&j)) {
                    falling.insert(i);
                }
            }
        }

        // Exclude the original brick from the number of bricks falling
        would_fall.insert(i, num_falling - 1);
    }

    would_fall.values().sum()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick{
    start: Vec3,
    end: Vec3,
}

impl Brick {
    /// Is this brick sitting on the ground?
    fn is_on_ground(&self) -> bool {
        self.start.z == (Z_GROUND + 1)
    }

    /// Does this brick overlap with another brick in the X/Y plane?
    fn overlap_xy(&self, other: &Brick) -> bool {
        self.end.x >= other.start.x && self.start.x <= other.end.x
        && self.end.y >= other.start.y && self.start.y <= other.end.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split(',').map(|s| s.parse::<i32>().unwrap());

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();

        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Brick>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let mut values = vec![];

        for line in input.lines() {
            let (a, b) = line.split_once('~').unwrap();

            values.push(Brick { start: Vec3::from_str(a), end: Vec3::from_str(b) });
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

        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 448);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 7);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 57770);
    }
}
