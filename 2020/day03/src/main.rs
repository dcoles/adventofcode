use std::path::Path;
use std::fs;

const OPEN: char = '.';
const TREE: char = '#';

fn main() {
    let map = read_input("input.txt");

    println!("Part 1: Number of trees encountered is {}", part1(&map));
}

fn part1(map: &Map) -> usize {
    let mut x = 0;
    let mut n_trees = 0;

    for y in 0..map.tiles.len() {
        if map.get(x, y) == TREE {
            n_trees += 1;
        }
        x += 3;
    }

    n_trees
}

fn read_input<T: AsRef<Path>>(path: T) -> Map {
    let tiles: Vec<Vec<char>> = fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = tiles[0].len();
    Map { tiles, width }
}

struct Map {
    tiles: Vec<Vec<char>>,
    width: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> char {
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
}

