// Simple 2D Map
use std::path::Path;
use std::fs;

pub type Pos = (i32, i32);
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
        assert!(self.is_valid_pos(&(x, y)));
        self.tiles[y as usize * self.width + x as usize]
    }

    pub fn get(&self, pos: Pos) -> Option<Tile> {
        if !self.is_valid_pos(&pos) {
            None
        } else {
            Some(self.tiles[pos.1 as usize * self.width + pos.0 as usize])
        }
    }

    pub fn is_valid_pos(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && (pos.0 as usize) < self.width
            && pos.1 >= 0 && (pos.1 as usize) < self.height
    }

    pub fn set(&mut self, (x, y): Pos, tile: Tile) {
        assert!(self.is_valid_pos(&(x, y)));
        self.tiles[y as usize * self.width + x as usize] = tile;
    }

    pub fn count(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|t| **t == tile).count()
    }

    pub fn print(&self) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                print!("{}", self.at((x, y)));
            }
            println!();
        }
        println!();
    }
}


