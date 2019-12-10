use std::path::Path;
use std::{fs, fmt};
use std::collections::HashSet;

const ASTEROID: char = '#';

fn main() {
    assert_eq!((Pos::new(3, 4), 8), best_position(&Map::from_file("sample0.txt").find_asteroids()));
    assert_eq!((Pos::new(5, 8), 33), best_position(&Map::from_file("sample1.txt").find_asteroids()));
    assert_eq!((Pos::new(1, 2), 35), best_position(&Map::from_file("sample2.txt").find_asteroids()));
    assert_eq!((Pos::new(6, 3), 41), best_position(&Map::from_file("sample3.txt").find_asteroids()));
    assert_eq!((Pos::new(11, 13), 210), best_position(&Map::from_file("sample4.txt").find_asteroids()));

    // Part 1
    let map = Map::from_file("input.txt");
    let asteroids = map.find_asteroids();
    let (pos, count) = best_position(&asteroids);

    println!("Part 1: Most visible {} from {:?}", count, pos);
}

fn best_position(asteroids: &HashSet<Pos>) -> (Pos, usize) {
    asteroids.iter().map(|&a| (a, num_visible_from_origin(a, &asteroids)))
        .max_by_key(|&(_, n)| n)
        .expect("No asteroids found")
}

fn num_visible_from_origin(origin: Pos, positions: &HashSet<Pos>) -> usize {
    let mut gradients = HashSet::new();
    for &pos in positions.iter().filter(|&&p| p != origin) {
        let dx = pos.x - origin.x;
        let dy = pos.y - origin.y;

        let q = gcd(dx.abs(), dy.abs());
        let gradient = (dx / q, dy / q);

        gradients.insert(gradient);
    }

    gradients.len()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Map {
    width: usize,
    data: Vec<char>,
}

impl Map {
    fn from_file<T: AsRef<Path>>(path: T) -> Map {
        let contents = fs::read_to_string(path).expect("Failed to read input");
        let mut width = None;
        let mut data = Vec::new();
        for line in contents.lines() {
            if width.is_none() {
                width = Some(line.len());
            }

            data.extend(line.chars());
        }

        Map { width: width.unwrap(), data }
    }

    fn find_asteroids(&self) -> HashSet<Pos> {
        self.data.iter().enumerate()
            .filter(|(_, &c)| c == ASTEROID)
            .map(|(n, _)| Pos { x: (n % self.width) as i32, y: (n / self.width) as i32 })
            .collect()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.width {
            let offset = y * self.width;
            writeln!(f, "{}", self.data[offset..offset+self.width].iter().copied().collect::<String>())?;
        }
        Ok(())
    }
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}
