//! Advent of Code 2022: Day 8
//! https://adventofcode.com/2022/day/8

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    // Consider your map; how many trees are visible from outside the grid?
    visible_trees(input).len()
}

fn part2(input: &Input) -> usize {
    // Consider each tree on your map.
    let visible = visible_trees(input);
    
    // What is the highest scenic score possible for any tree?
    visible.into_iter().map(|pos| scenic_score(input, pos)).max().unwrap()
}

/// Find all trees that are visible from the outside
fn visible_trees(input: &Input) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..input.height {
        let mut view = 0;
        for x in 0..input.width {
            // Add 1 to height so we can use 0 as an initial value
            let h = input.tree_heights[&(x, y)] + 1;
            if view < h {
                visible.insert((x, y));
                view = h;
            }
        }
    }

    for y in 0..input.height {
        let mut view = 0;
        for x in (0..input.width).rev() {
            // Add 1 to height so we can use 0 as an initial value
            let h = input.tree_heights[&(x, y)] + 1;
            if view < h {
                visible.insert((x, y));
                view = h;
            }
        }
    }

    for x in 0..input.width {
        let mut view = 0;
        for y in 0..input.height {
            // Add 1 to height so we can use 0 as an initial value
            let h = input.tree_heights[&(x, y)] + 1;
            if view < h {
                visible.insert((x, y));
                view = h;
            }
        }
    }

    for x in 0..input.width {
        let mut view = 0;
        for y in (0..input.height).rev() {
            // Add 1 to height so we can use 0 as an initial value
            let h = input.tree_heights[&(x, y)] + 1;
            if view < h {
                visible.insert((x, y));
                view = h;
            }
        }
    }

    visible
}

/// Calculate scenic score for a tree.
fn scenic_score(input: &Input, (x, y): (usize, usize)) -> usize {
    // What is this tree's height
    let h = input.tree_heights[&(x, y)];

    // Look Down
    let mut s1 = 0;
    for y1 in (0..y).rev() {
        s1 = y - y1;
        if input.tree_heights[&(x, y1)] >= h {
            break;
        }
    }

    // Look Left
    let mut s2 = 0;
    for x1 in (0..x).rev() {
        s2 = x - x1;
        if input.tree_heights[&(x1, y)] >= h {
            break;
        }
    }
    
    // Look Up
    let mut s3 = 0;
    for y1 in (y + 1)..input.height {
        s3 = y1 - y;
        if input.tree_heights[&(x, y1)] >= h {
            break;
        }
    }

    // Look Right
    let mut s4 = 0;
    for x1 in (x + 1)..input.width {
        s4 = x1 - x;
        if input.tree_heights[&(x1, y)] >= h {
            break;
        }
    }

    s1 * s2 * s3 * s4
}

#[derive(Debug, Clone)]
struct Input {
    tree_heights: HashMap<(usize, usize), u32>,
    width: usize,
    height: usize,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut tree_heights = HashMap::new();
        let mut width = 0;
        let mut height = 0;


        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let h = c as u32 - '0' as u32;

                tree_heights.insert((x, y), h);

                width = x + 1;
                height = y + 1;
            }
        }

        Ok(Input { tree_heights, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 8);
    }
}
