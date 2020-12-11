// Simple 2D Map
use std::path::Path;
use std::fs;

pub type Pos = (usize, usize);
pub type Tile = char;

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize
}

impl Map {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read input");

        let mut tiles = Vec::new();
        let height = contents.lines().count();
        let width = contents.lines().next().unwrap_or("").len();

        for line in contents.lines() {
            tiles.extend(line.chars());
        }

        assert_eq!(tiles.len(), width * height);
        Map { tiles, width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn at(&self, (x, y): Pos) -> Tile {
        assert!(x < self.width);
        assert!(y < self.height);
        self.tiles[y * self.width + x]
    }

    pub fn set(&mut self, (x, y): Pos, tile: Tile) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.tiles[y * self.width + x] = tile;
    }

    pub fn count(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|t| **t == tile).count()
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.at((x, y)));
            }
            println!();
        }
        println!();
    }
}


