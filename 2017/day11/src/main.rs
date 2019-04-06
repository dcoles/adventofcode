use std::fs::read_to_string;
use std::path::Path;
use std::io::Error;
use std::ops::AddAssign;

const ORIGIN: Cube = Cube(0, 0, 0);

fn main() {
    let input = parse_input("input.txt").expect("Failed to read input");

    let mut pos = ORIGIN;
    let mut max = pos;
    for direction in &input {
        pos += direction.cube_direction();
        if pos.distance(ORIGIN) > max.distance(ORIGIN) {
            max = pos;
        }
    }

    // Part 1
    println!("Distance: {:?}", pos.distance(ORIGIN) / 2);

    // Part 2
    println!("Max Distance: {:?}", max.distance(ORIGIN) / 2);
}

fn parse_input<P: AsRef<Path>>(path: P) -> Result<Vec<Direction>, Error> {
    Ok(read_to_string(path)?
        .trim()
        .split(",")
        .map(|d| Direction::from_str(d))
        .collect())
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Direction {
    fn from_str(string: &str) -> Direction {
        match string {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => panic!("Unknown direction: {}", string),
        }
    }

    fn cube_direction(&self) -> Cube {
        match self {
            Direction::N => Cube(0, 1 , -1),
            Direction::NE => Cube(1, 0, -1),
            Direction::SE => Cube(1, -1, 0),
            Direction::S => Cube(0, -1, 1),
            Direction::SW => Cube(-1, 0, 1),
            Direction::NW => Cube(-1, 1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cube(i32, i32, i32);

impl Cube {
    fn distance(&self, other: Cube) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

impl AddAssign for Cube {
    fn add_assign(&mut self, other: Cube) {
        *self = Cube(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
