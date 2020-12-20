use std::path::Path;
use std::fs;
use std::collections::{HashSet, HashMap};

type Input = Vec<Tile>;
type Pos = (usize, usize);

const SIZE: usize = 10;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");

    input.trim().split("\n\n").map(|s| Tile::from_str(s)).collect()
}

fn part1(input: &Input) -> u64 {
    let tile_edges: HashMap<u64, HashSet<String>> = input.iter().map(|t| (t.id, t.edges())).collect();

    let mut corners = Vec::new();

    for (&id, edges) in &tile_edges {
        let common: HashSet<String> = tile_edges.iter()
            .filter(|(&tile_id, _)| tile_id != id)
            .flat_map(|(_, edges)| edges)
            .cloned()
            .collect();

        let n = edges.intersection(&common).count();
        if n == 4 {
            corners.push(id);
        }
    }

    corners.iter().product()
}

#[derive(Debug)]
struct Tile {
    id: u64,
    tiles: Vec<char>,
}

impl Tile {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..9].parse().expect("Failed to parse number");
        let tiles = lines.flat_map(|line| line.chars()).collect();

        Tile { id, tiles }
    }

    fn get(&self, (x, y): Pos) -> Option<char> {
        let i = y * SIZE + x;
        if i > SIZE*SIZE || x > SIZE {
            None
        } else {
            Some(self.tiles[i])
        }
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for y in 0..SIZE {
            for x in 0..SIZE {
                print!("{}", self.get((x, y)).unwrap());
            }
            println!();
        }
        println!();
    }

    fn edges(&self) -> HashSet<String> {
        let mut edges = HashSet::new();

        edges.insert((0..SIZE).map(|x| self.get((x, 0)).unwrap()).collect());
        edges.insert((0..SIZE).map(|x| self.get((x, SIZE-1)).unwrap()).collect());
        edges.insert((0..SIZE).map(|y| self.get((0, y)).unwrap()).collect());
        edges.insert((0..SIZE).map(|y| self.get((SIZE-1, y)).unwrap()).collect());

        // Flipped versions
        edges.insert((0..SIZE).rev().map(|x| self.get((x, 0)).unwrap()).collect());
        edges.insert((0..SIZE).rev().map(|x| self.get((x, SIZE-1)).unwrap()).collect());
        edges.insert((0..SIZE).rev().map(|y| self.get((0, y)).unwrap()).collect());
        edges.insert((0..SIZE).rev().map(|y| self.get((SIZE-1, y)).unwrap()).collect());

        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 0);
    }
}

