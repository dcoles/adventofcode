//! Advent of Code 2024: Day 9
//! https://adventofcode.com/2024/day/9

use std::{fs, io};
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Extent {
    id: Option<usize>,
    length: usize,
}

fn extents_map(extents: &[Extent]) -> BTreeMap<usize, Extent> {
    let mut map = BTreeMap::new();

    let mut offset = 0;

    for extent in extents {
        if extent.length == 0 {
            continue;
        }

        map.insert(offset, *extent);

        offset += extent.length;
    }

    map
}

fn checksum(extents_map: &BTreeMap<usize, Extent>) -> usize {
    let mut checksum = 0;

    for (&offset, extent) in extents_map {
        if let Some(id) = extent.id {
            for n in offset..(offset + extent.length) {
                checksum += n * id;
            }
        }
    }

    checksum
}

fn part1(input: &Input) -> usize {
    let mut extents_map = extents_map(&input.values);

    loop {
        let (&hole_pos, _) = extents_map.iter().find(|(_, e)| e.id.is_none()).unwrap();
        let (&file_pos, _) = extents_map.iter().rfind(|(_, e)| e.id.is_some()).unwrap();

        if file_pos <= hole_pos {
            // No more suitable holes left
            break;
        }

        let mut hole = extents_map.remove(&hole_pos).unwrap();
        let mut file = extents_map.remove(&file_pos).unwrap();

        let size = usize::min(hole.length, file.length);
        hole.length -= size;
        file.length -= size;

        extents_map.insert(hole_pos, Extent { id: file.id, length: size });

        if hole.length > 0 {
            extents_map.insert(hole_pos + size, hole);
        }

        if file.length > 0 {
            extents_map.insert(file_pos, file);
        }
    }

    checksum(&extents_map)
}

fn part2(input: &Input) -> usize {
    let mut extents_map = extents_map(&input.values);
    let file_positions: Vec<_> = extents_map.iter().filter_map(|(&offset, extent)| {
        extent.id.is_some().then_some(offset)
    }).collect();

    for &file_pos in file_positions.iter().rev() {
        let len = extents_map[&file_pos].length;

        if let Some((&hole_pos, _)) = extents_map.iter().find(|(_, e)| e.id.is_none() && e.length >= len) {
            if hole_pos >= file_pos {
                // No better position found
                continue;
            }

            let file = extents_map.remove(&file_pos).unwrap();
            let mut hole = extents_map.remove(&hole_pos).unwrap();

            hole.length -= file.length;

            if hole.length > 0 {
                extents_map.insert(hole_pos + file.length, hole);
            }

            extents_map.insert(hole_pos, file);
        }
    }

    checksum(&extents_map)
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Extent>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input
            .trim()
            .chars()
            .enumerate()
            .map(|(n, c)| {
                let is_file = n % 2 == 0;
                let length = c.to_digit(10).unwrap() as usize;

                Extent {
                    id: is_file.then_some(n / 2),
                    length,
                }
            })
            .collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 1928);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 6446899523367);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 2858);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 6478232739671);
    }
}
