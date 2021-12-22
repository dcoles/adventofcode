//! Advent of Code 2021: Day 22
//! https://adventofcode.com/2021/day/22

use std::collections::HashSet;
use std::fs;
use std::io;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::path::Path;

const INITIALIZATION_RANGE: RangeInclusive<i32> = -50..=50;

type Cuboid = [RangeInclusive<i32>; 3];

fn main() {
    let input = Input::from_file("day22/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut reactor = HashSet::new();

    'outer: for (state, cuboid) in input.cubes.clone() {
        for axis in &cuboid {
            if axis.start() < INITIALIZATION_RANGE.start() || axis.end() > INITIALIZATION_RANGE.end() {
                continue 'outer;
            }
        }

        for z in cuboid[2].clone() {
            for y in cuboid[1].clone() {
                for x in cuboid[0].clone() {
                    if state {
                        reactor.insert([x, y, z]);
                    } else {
                        reactor.remove(&[x, y, z]);
                    }
                }
            }
        }
    }

    reactor.len()
}

fn part2(input: &Input) -> usize {
    // The key idea is to chop up the reactor into 3D sectors and keep track of
    // the state of these sectors rather than individual cubes.

    // Determine sector "cuts"
    let mut cuts = [HashSet::new(), HashSet::new(), HashSet::new()];
    for cube in &input.cubes {
        for (i, axis) in cube.1.iter().enumerate() {
            cuts[i].insert(*axis.start());
            cuts[i].insert(*axis.end() + 1);
        }
    }

    // Sort cuts
    let mut cuts2 = [vec![], vec![], vec![]];
    for (i, cut) in cuts.into_iter().enumerate() {
        cuts2[i].extend(cut);
        cuts2[i].sort_unstable();
    }

    // Group into pairs
    let mut cuts3 = [vec![], vec![], vec![]];
    for (i, cut) in cuts2.into_iter().enumerate() {
        cuts3[i].extend(cut.windows(2).map(|w| [w[0], w[1]]));
    }

    // Run the procedure
    let mut reactor = HashSet::new();
    for (state, cuboid) in &input.cubes {
        println!("CUBE: {:?} ({})", cuboid, if *state { "on" } else { "off" });
        if *state {
            // Turn sectors off
            for z in filter_cuts(&cuts3[2], *cuboid[2].start(), *cuboid[2].end()) {
                for y in filter_cuts(&cuts3[1], *cuboid[1].start(), *cuboid[1].end()) {
                    for x in filter_cuts(&cuts3[0], *cuboid[0].start(), *cuboid[0].end()) {
                        // Found a sector to turn on
                        let sector = [x[0]..x[1], y[0]..y[1], z[0]..z[1]];
                        reactor.insert(sector);
                    }
                }
            }
        } else {
            // Turn sectors off
            let remove: Vec<_> = reactor.iter().filter(|cut| {
                // Sector must be within cuboid
                cut.iter().enumerate().all(|(i, r)| r.start >= *cuboid[i].start() && r.end <= *cuboid[i].end() + 1)
            }).cloned().collect();
            for remove in remove {
                reactor.remove(&remove);
            }
        }
    }

    let count = reactor.iter()
        .map(|c| n_cubes(c))
        .sum();

    count
}

/// A smarter filter
/// NOTE: Cuts MUST be sorted!
fn filter_cuts(cuts: &[[i32; 2]], start: i32, end: i32) -> impl Iterator<Item=&[i32; 2]> {
    let start = cuts.partition_point(|c| c[0] < start);
    let end = cuts.partition_point(|c| c[1] <= end + 1);

    cuts[start..end].into_iter()
}

/// Number of cubes in a 3D sector
fn n_cubes(sector: &[Range<i32>; 3]) -> usize {
    sector.iter().map(|r| (r.end - r.start) as usize).product()
}

#[derive(Debug, Clone)]
struct Input {
    cubes: Vec<(bool, Cuboid)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut cubes = Vec::new();
        for line in input.lines() {
            let (state, coords) = line.split_once(" ").unwrap();
            let mut cuboid = [0..=0, 0..=0, 0..=0];
            for (i, axis) in coords.split(",").enumerate() {
                let (_, range) = axis.split_once("=").unwrap();
                let (low, high) = range.split_once("..").unwrap();
                let low = low.parse().unwrap();
                let high = high.parse().unwrap();

                cuboid[i] = low..=high;
            }

            cubes.push((state == "on", cuboid));
        }

        Ok(Input { cubes })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 39)
    }

    #[test]
    fn test_part1_example2() {
        let input = Input::from_file("example2.txt").expect("failed to read input");

        assert_eq!(part1(&input), 590784)
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example3.txt").expect("failed to read input");

        assert_eq!(part2(&input), 2758514936282235);
    }
}
