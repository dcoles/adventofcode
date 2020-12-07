// Simple 2D Map
use std::path::Path;
use std::fs;

type Pos = (usize, usize);
type Tile = char;

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize
}

impl Map {
    fn from_file<T: AsRef<Path>>(path: T) -> Self {
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

    fn at(&self, (x, y): Pos) -> Tile {
        assert!(x < self.width);
        assert!(y < self.height);
        self.tiles[y * self.width + x]
    }
}


