//! Advent of Code 2021: Day 25
//! https://adventofcode.com/2021/day/25

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const EMPTY: char = '.';
const EAST: char = '>';
const SOUTH: char = 'v';

fn main() {
    let input = Input::from_file("day25/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    let mut tiles = input.tiles.clone();
    for n in 1.. {
        let mut moved = false;

        //print_tiles(&tiles);

        // Eastbound herd moves first
        let mut new_tiles = tiles.clone();
        for (&pos, &c) in tiles.iter() {
            match c {
                EAST => {
                    let mut new_pos = [pos[0] + 1, pos[1]];
                    if !tiles.contains_key(&new_pos) {
                        new_pos = [0, pos[1]];
                    }
                    
                    if tiles[&new_pos] == EMPTY {
                        new_tiles.insert(pos, EMPTY);
                        new_tiles.insert(new_pos, c);
                        moved = true;
                    }
                },
                _ => (),
            }
        }

        tiles = new_tiles;

        // Southbound herd moves first
        let mut new_tiles = tiles.clone();
        for (&pos, &c) in tiles.iter() {
            match c {
                SOUTH => {
                    let mut new_pos = [pos[0], pos[1] + 1];
                    if !tiles.contains_key(&new_pos) {
                        new_pos = [pos[0], 0];
                    }
                    
                    if tiles[&new_pos] == EMPTY {
                        new_tiles.insert(pos, EMPTY);
                        new_tiles.insert(new_pos, c);
                        moved = true;
                    }
                },
                _ => (),
            }
        }
        if !moved {
            return n;
        }

        tiles = new_tiles;
    }

    0
}

fn print_tiles(tiles: &HashMap<Pos, char>) {
    let width = tiles.keys().map(|&[x, _]| x).max().unwrap() + 1;
    let height = tiles.keys().map(|&[_, y]| y).max().unwrap() + 1;
    for y in 0..height {
        for x in 0..width {
            print!("{}", tiles.get(&[x, y]).unwrap_or(&' '));
        }
        println!();
    }
    println!();
}

type Pos = [i32; 2];

#[derive(Debug, Clone)]
struct Input {
    tiles: HashMap<Pos, char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut tiles = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                tiles.insert([x as i32, y as i32], c);
            }
        }

        Ok(Input { tiles })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").expect("failed to read input");
    
        assert_eq!(part1(&input), 58);
    }
}
