use std::path::Path;
use std::fs;

type Tile = char;
const TREE: Tile = '#';

fn main() {
    let map = read_input("input.txt");

    println!("Part 1: Number of trees encountered is {}", part1(&map));
    println!("Part 2: Product of all slopes is {}", part2(&map));
}

fn part1(map: &Map) -> usize {
    trees_encountered(map, 3, 1)
}

fn part2(map: &Map) -> usize {
    let mut n = 1;
    for (x_offset, y_offset) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        n *= trees_encountered(&map, *x_offset, *y_offset);
    }

    n
}

/// Calculate number of trees that would be intersected following a slope of `y_offset` / `x_offset`.
fn trees_encountered(map: &Map, x_offset: usize, y_offset: usize) -> usize {
    assert_ne!(y_offset, 0);
    let mut x = 0;
    let mut y = 0;
    let mut n_trees = 0;

    while y < map.tiles.len() {
        if map.get(x, y) == TREE {
            n_trees += 1;
        }

        x += x_offset;
        y += y_offset;
    }

    n_trees
}

fn read_input<T: AsRef<Path>>(path: T) -> Map {
    let tiles: Vec<Vec<char>> = fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = tiles[0].len();  // XXX: This will panic if no lines are read
    Map { tiles, width }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
}

impl Map {
    /// Get tile at position `x`, `y`.
    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x % self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let map = read_input("input1.txt");
        assert_eq!(part1(&map), 7);
    }

    #[test]
    fn test_part2_example1() {
        let map = read_input("input1.txt");
        assert_eq!(part2(&map), 336);
    }
}

