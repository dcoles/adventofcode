use std::path::Path;
use std::{fs, ops, fmt};
use std::collections::HashSet;

const ORIGIN: Pos = Pos::new(0, 0);

fn main() {
    let input = read_input("input.txt");
    for wire in &input {
        println!("{}", wire.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(","));
    }

    // Part 1
    let mut grid: HashSet<Pos> = HashSet::new();
    let mut intersections: Vec<Pos> = Vec::new();
    for wire in &input {
        // Walk the wire's path
        let path = walk(&wire);

        // Check for any intersections
        for &pos in &path {
            if grid.contains(&pos) {
                intersections.push(pos);
            }
        }

        // Add the wire to to grid
        for &pos in &path {
            grid.insert(pos);
        }
    }

    let closest_intersection = intersections.iter()
        .map(|&p| (p, p.distance(ORIGIN)))
        .min_by_key(|&(_, distance)| distance)
        .expect("Expected non-zero intersections");
    println!("Part 1: Closest intersection: {} (distance: {})", closest_intersection.0, closest_intersection.1);

    // Part 2
    let mut min_signal_delay = std::u32::MAX;
    for &intersection in &intersections {
        let mut steps = 0;
        for wire in &input {
            // Walk the wire's path until we hit the intersection
            for pos in walk(&wire) {
                steps += 1;
                if pos == intersection {
                    break;
                }
            }
        }
        min_signal_delay = min_signal_delay.min(steps);
    }
    println!("Part 2: Fewest combined steps: {}", min_signal_delay);
}

/// Read a list of move-lists from a file.
fn read_input<T: AsRef<Path>>(path: T) -> Vec<Vec<Move>> {
    let input = fs::read_to_string(path).expect("Failed to read input");
    let mut result = Vec::new();
    for line in input.lines() {
        let moves: Vec<Move> = line.trim().split(',').filter_map(Move::parse).collect();
        result.push(moves);
    }

    result
}

/// Follow a move-list and return the step-by-step path followed.
fn walk(movelist: &[Move]) -> Vec<Pos> {
    let mut path = Vec::new();

    let mut pos = ORIGIN;
    for m in movelist {
        let unit = m.direction().unit();
        for _ in 0..m.distance() {
            pos += unit;
            path.push(pos);
        }
    }

    path
}

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    const fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn distance(self, p: Pos) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

#[derive(Eq,PartialEq,Copy,Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }

    fn unit(self) -> Pos {
        match self {
            Direction::Up => Pos::new(0, 1),
            Direction::Down => Pos::new(0, -1),
            Direction::Left => Pos::new(-1, 0),
            Direction::Right => Pos::new(1, 0),
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        }
    }
}

#[derive(Eq,PartialEq,Copy,Clone)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl Move {
    fn parse(s: &str) -> Option<Move> {
        let direction = Direction::parse(&s[..1])?;
        let distance = s[1..].parse::<u32>().ok()?;

        Some(Move { direction, distance })
    }

    fn direction(self) -> Direction {
        self.direction
    }

    fn distance(self) -> u32 {
        self.distance
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.direction.to_str(), self.distance)
    }
}
