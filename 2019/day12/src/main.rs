use std::path::Path;
use std::{fs, ops};
use std::fmt;

fn main() {
    // Tests
    let mut m1 = read_input("sample1.txt");
    simulate(&mut m1, 10);
    assert_eq!(179, total_energy(&m1));

    let mut m2 = read_input("sample2.txt");
    simulate(&mut m2, 100);
    assert_eq!(1940, total_energy(&m2));

    // Part 1
    let mut moons = read_input("input.txt");
    simulate(&mut moons, 1000);
    println!("Part 1: Total energy {}", total_energy(&moons));
}

fn simulate(moons: &mut [Moon], steps: usize) {
    for _ in 1..=steps {
        // Apply gravity
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }

                let other_pos = moons[j].pos;
                moons[i].apply_gravity(other_pos);
            }
        }

        // Update position
        for moon in moons.iter_mut() {
            moon.tick();
        }
    }
}

fn total_energy(moons: &[Moon]) -> i32 {
    let mut total_energy = 0;
    for moon in moons {
        total_energy += moon.energy();
    }

    total_energy
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Moon> {
    let mut coords = Vec::new();
    let contents = fs::read_to_string(path).expect("Failed to read input");
    for line in contents.lines() {
        let line = line.trim_start_matches('<').trim_end_matches('>');
        let mut split = line.split(',');
        let x = split.next().expect("Expected x field")
            .split('=').nth(1).expect("Expected value")
            .parse::<i32>().expect("Failed to parse value");
        let y = split.next().expect("Expected y field")
            .split('=').nth(1).expect("Expected value")
            .parse::<i32>().expect("Failed to parse value");
        let z = split.next().expect("Expected z field")
            .split('=').nth(1).expect("Expected value")
            .parse::<i32>().expect("Failed to parse value");

        coords.push(Moon::new(Triple::new(x, y, z)));
    }

    coords
}

struct Moon {
    pos: Triple,
    vel: Triple,
}

impl Moon {
    fn new(position: Triple) -> Moon {
        Moon { pos: position, vel: Triple::new(0, 0, 0) }
    }

    fn apply_gravity(&mut self, position: Triple) {
        apply_gravity_1d(&mut self.vel.x, self.pos.x, position.x);
        apply_gravity_1d(&mut self.vel.y, self.pos.y, position.y);
        apply_gravity_1d(&mut self.vel.z, self.pos.z, position.z);
    }

    fn tick(&mut self) {
        self.pos += self.vel;
    }

    fn energy(&self) -> i32 {
        let potential = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kinetic = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();

        potential * kinetic
    }
}

fn apply_gravity_1d(velocity: &mut i32, pos: i32, other_pos: i32) {
    if pos > other_pos {
        *velocity -= 1;
    } else if other_pos > pos {
        *velocity += 1;
    }
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct Triple {
    x: i32,
    y: i32,
    z: i32,
}

impl Triple {
    fn new(x: i32, y: i32, z: i32) -> Triple {
        Triple { x, y, z }
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:2}, {:2}, {:2}>", self.x, self.y, self.z)
    }
}

impl ops::Add for Triple {
    type Output = Triple;

    fn add(self, rhs: Self) -> Self::Output {
        Triple { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::AddAssign for Triple {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
