use std::path::Path;
use std::{fs, fmt};
use std::collections::{HashSet, HashMap};

const ASTEROID: char = '#';

fn main() {
    // Part 1
    assert_eq!((Pos::new(3, 4), 8), best_position(&Map::from_file("sample0.txt").find_asteroids()));
    assert_eq!((Pos::new(5, 8), 33), best_position(&Map::from_file("sample1.txt").find_asteroids()));
    assert_eq!((Pos::new(1, 2), 35), best_position(&Map::from_file("sample2.txt").find_asteroids()));
    assert_eq!((Pos::new(6, 3), 41), best_position(&Map::from_file("sample3.txt").find_asteroids()));
    assert_eq!((Pos::new(11, 13), 210), best_position(&Map::from_file("sample4.txt").find_asteroids()));

    let map = Map::from_file("input.txt");
    let asteroids = map.find_asteroids();
    let (pos, count) = best_position(&asteroids);

    println!("Part 1: Most visible {} from {}", count, pos);

    // Part 2
    assert_eq!(Pos::new(8, 2),
               asteroids_zapped(Pos::new(11, 13), &Map::from_file("sample4.txt").find_asteroids())[199]);

    let zapped = asteroids_zapped(pos, &asteroids);
    let asteroid_200 = zapped[199];

    println!("Part 2: Index 200 is {} (answer: {})", asteroid_200, 100 * asteroid_200.x + asteroid_200.y);
}

/// Find the best position (asteroid with the most visible other asteroids)
fn best_position(asteroids: &HashSet<Pos>) -> (Pos, usize) {
    asteroids.iter().map(|&a| (a, num_visible_from_origin(a, &asteroids)))
        .max_by_key(|&(_, n)| n)
        .expect("No asteroids found")
}

/// How many positions are visible from this position
fn num_visible_from_origin(origin: Pos, positions: &HashSet<Pos>) -> usize {
    let mut gradients = HashSet::new();
    for &pos in positions.iter().filter(|&&p| p != origin) {
        gradients.insert(origin.gradient(pos));
    }

    gradients.len()
}

/// Find the order that asteroids would be zapped by the laser
fn asteroids_zapped(origin: Pos, asteroids: &HashSet<Pos>) -> Vec<Pos> {
    let mut zapped = Vec::new();
    let mut asteroids_by_angle = asteroid_angles(origin, asteroids);

    let mut i = 0;
    for _ in 1..asteroids.len() {  // excluding the origin
        // Skip angles with no asteroids left
        while asteroids_by_angle[i].1.is_empty() {
            i = (i + 1) % asteroids_by_angle.len();
        }

        // Zap!
        let pos = asteroids_by_angle[i].1.pop().unwrap();
        zapped.push(pos);
        i = (i + 1) % asteroids_by_angle.len();
    }

    zapped
}

/// Group asteroids by their angle from an origin
fn asteroid_angles(origin: Pos, positions: &HashSet<Pos>) -> Vec<(f32, Vec<Pos>)> {
    // Group positions by their gradient
    let mut gradients: HashMap<(i32, i32), Vec<Pos>> = HashMap::new();
    for &pos in positions.iter().filter(|&&p| p != origin) {
        gradients.entry(origin.gradient(pos)).or_default().push(pos);
    }

    // Sort positions by distance
    for posns in gradients.values_mut() {
        // Sort by distance
        posns.sort_by_key(|&pos| -origin.distance(pos));
    }

    // Map gradient to vector sorted by angles
    let mut angles: Vec<(f32, Vec<Pos>)> = gradients.into_iter().map(|((dx, dy), posns)| {
        let angle = match (dx as f32).atan2(-dy as f32) {
            // Map from (-π, π] to [0, 2π)
            angle if dx < 0 => angle + 2.0 * std::f32::consts::PI,
            angle => angle,
        };

        (angle, posns)
    }).collect();
    angles.sort_by(|&(a1, _), &(a2, _)| a1.partial_cmp(&a2).unwrap());

    angles
}

/// Calculate the greatest common divisor of two numbers
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

    fn gradient(self, target: Pos) -> (i32, i32) {
        let dx = target.x - self.x;
        let dy = target.y - self.y;

        let q = gcd(dx.abs(), dy.abs());

        (dx / q, dy / q)
    }

    fn distance(self, b: Pos) -> i32 {
        (b.x - self.x).pow(2) + (b.y - self.y).pow(2)
    }

}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
