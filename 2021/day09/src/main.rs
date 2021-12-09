//! Advent of Code 2021: Day 9
//! https://adventofcode.com/2021/day/9

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::ops::Index;
use std::path::Path;

const MAX_HEIGHT: u32 = 9;

type Pos = (i32, i32);

fn main() {
    let map = Map::from_file("day09/input.txt").expect("failed to read input");
    
    // Part 1
    println!("Part 1: {}", part1(&map));

    // Part 2
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &Map) -> u32 {
    let mut result = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {
            let pos = (x as i32, y as i32);
            if let Some(height) = map.get(pos) {
                if map.adjacent(pos).into_iter().all(|p| map[p] > height) {
                    result += 1 + height;
                }
            }
        }
    }

    result
}

fn part2(map: &Map) -> u32 {
    let mut basins = Vec::new();
    let mut seen = HashSet::new();

    for y in 0..map.height() {
        for x in 0..map.width() {
            let pos = (x as i32, y as i32);
            if map.contains(pos) && map[pos] < MAX_HEIGHT && !seen.contains(&pos) {
                // New basin
                seen.insert(pos);
                let mut size = 1;

                // Depth first search
                let mut edge = vec![pos];
                while let Some(pos) = edge.pop() {
                    let adjacent: Vec<_> = map.adjacent(pos).into_iter().filter(|&p| !seen.contains(&p) && map[p] < MAX_HEIGHT).collect();
                    for adj in adjacent {
                        seen.insert(adj);
                        size += 1;

                        edge.push(adj);
                    };
                }

                basins.push(size);
            }
        }
    }

    basins.sort();

    // Multiply three largest basins
    basins[basins.len() - 3..].into_iter().product()
}


struct Map {
    heights: HashMap<Pos, u32>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut heights = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let height = c.to_digit(10).unwrap();

                heights.insert((x as i32, y as i32), height);
            }

            height += 1;
            width = line.len();
        }

        Ok(Map { heights, width, height })
    }
    
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn contains(&self, pos: Pos) -> bool {
        self.heights.contains_key(&pos)
    }

    fn get(&self, pos: Pos) -> Option<u32> {
        self.heights.get(&pos).copied()
    }

    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        let mut adjacent = Vec::new();
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let adj = (pos.0 + dx, pos.1 + dy);
            if self.heights.contains_key(&adj) {
                adjacent.push(adj);
            }
        }

        adjacent
    }
}

impl Index<Pos> for Map {
    type Output = u32;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.heights[&pos]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let map = Map::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&map), 15);
    }

    #[test]
    fn test_part2() {
        let map = Map::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&map), 1134);
    }
}
