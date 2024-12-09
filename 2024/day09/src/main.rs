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

fn part1(input: &Input) -> usize {
    let mut free = BTreeMap::default();
    let mut files = BTreeMap::default();
    let mut offset = 0;

    for (n, len) in input.values.iter().copied().enumerate() {
        if len == 0 {
            continue;
        }

        if n % 2 == 0 {
            files.insert(offset, Extent {
                id: Some(n / 2),
                length: len,
            });
        } else {
            free.insert(offset, Extent {
                id: None,
                length: len,
            });
        }
        offset += len;
    }

    while let Some((offset, mut hole)) = free.pop_first() {
        let extent = if let Some(mut entry) = files.last_entry() {
            if *entry.key() < offset {
                break;
            }

            let file = entry.get_mut();
            let id = file.id;
            let length = usize::min(hole.length, file.length);

            hole.length -= length;
            file.length -= length;

            if file.length == 0 {
                entry.remove();
            }

            Extent { id, length }
        } else {
            panic!();
        };

        files.insert(offset, extent);

        if hole.length > 0 {
            free.insert(offset + extent.length, hole);
        }
    }


    let mut checksum = 0;
    for (offset, extent) in files {
        for n in offset..(offset + extent.length) {
            checksum += n * extent.id.unwrap();
        }
    }

    checksum
}

fn part2(input: &Input) -> usize {
    let mut free = BTreeMap::default();
    let mut files = BTreeMap::default();
    let mut offset = 0;

    for (n, len) in input.values.iter().copied().enumerate() {
        if len == 0 {
            continue;
        }

        if n % 2 == 0 {
            files.insert(offset, Extent {
                id: Some(n / 2),
                length: len,
            });
        } else {
            free.insert(offset, Extent {
                id: None,
                length: len,
            });
        }
        offset += len;
    }

    let mut defragmented = BTreeMap::new();
    while let Some((cur_pos, file)) = files.pop_last() {
        let mut selected_hole = None;
        for (&hole_pos, hole) in free.iter() {
            if hole_pos >= cur_pos {
                break;
            }

            if file.length <= hole.length {
                selected_hole = Some(hole_pos);
                break;
            }
        }

        if let Some(hole_pos) = selected_hole {
            let len = file.length;
            let mut hole = free.remove(&hole_pos).unwrap();

            defragmented.insert(hole_pos, file);
            hole.length -= len;

            if hole.length > 0 {
                free.insert(hole_pos + len, hole);
            }

        } else {
            defragmented.insert(cur_pos, file);
        }
    }

    let mut checksum = 0;
    for (offset, extent) in defragmented {
        for n in offset..(offset + extent.length) {
            checksum += n * extent.id.unwrap();
        }
    }

    checksum
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<usize>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
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
