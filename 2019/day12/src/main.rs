use std::path::Path;
use std::{fs, ops};
use std::fmt;
use std::collections::HashSet;

fn main() {
    // Tests
    let mut s1 = Simulation::new(read_input("sample1.txt"));
    s1.simulate_n_steps(10);
    assert_eq!(179, s1.total_energy());

    let mut s2 = Simulation::new(read_input("sample2.txt"));
    s2.simulate_n_steps(100);
    assert_eq!(1940, s2.total_energy());

    let mut s3 = Simulation::new(read_input("sample2.txt"));
    s3.simulate_until_repeat();
    assert_eq!(4686774924, s3.t);

    // Part 1
    let mut sim1 = Simulation::new(read_input("input.txt"));
    sim1.simulate_n_steps(1000);
    println!("Part 1: Total energy of system after 1000 steps: {}", sim1.total_energy());

    // Part 2
    let mut sim2 = Simulation::new(read_input("input.txt"));
    sim2.simulate_until_repeat();
    println!("Part 2: {}", sim2.t);
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

struct Simulation {
    t: usize,
    initial_state: Vec<Moon>,
    current_state: Vec<Moon>,
}

impl Simulation {
    fn new(moons: Vec<Moon>) -> Simulation {
        Simulation { t: 0, initial_state: moons.clone(), current_state: moons }
    }

    fn simulate_n_steps(&mut self, steps: usize) {
        while steps != self.t {
            self.simulate_one_tick();
        }
    }

    fn simulate_until_repeat(&mut self) {
        let mut seen = HashSet::new();

        // Find the time that each of the axis first repeat
        let mut t_x = None;
        let mut t_y = None;
        let mut t_z = None;
        for _ in 0.. {
            let pos_vel_x: Vec<_> = self.current_state.iter().map(|m| ('x', m.pos.x, m.vel.x)).collect();
            if t_x.is_none() && !seen.insert(pos_vel_x.clone()) {
                t_x = Some(self.t);
            }

            let pos_vel_y: Vec<_> = self.current_state.iter().map(|m| ('y', m.pos.y, m.vel.y)).collect();
            if t_y.is_none() && !seen.insert(pos_vel_y.clone()) {
                t_y = Some(self.t);
            }

            let pos_vel_z: Vec<_> = self.current_state.iter().map(|m| ('z', m.pos.z, m.vel.z)).collect();
            if t_z.is_none() && !seen.insert(pos_vel_z.clone()) {
                t_z = Some(self.t);
            }

            if t_x.is_some() && t_y.is_some() && t_z.is_some() {
                break;
            }

            self.simulate_one_tick();
        }

        let t_x = t_x.unwrap();
        let t_y = t_y.unwrap();
        let t_z = t_z.unwrap();

        // Find the first number that's a multiple of each axis's time
        // A simpler method is to repeatedly add to the smallest number until all numbers match,
        // but this is substantially slower than using this way based on the GCD.
        // See: https://en.wikipedia.org/wiki/Least_common_multiple
        self.t = t_x * t_y * t_z  // A number with all factors (but likely not the first)
            / gcd(t_x, t_y) / gcd(t_y, t_z) / gcd(t_z, t_x)  // Remove common factors of each pair (but also removes factors common to all 3)
            * gcd(t_x, gcd(t_y, t_z));  // Add back factors common to all 3
        self.current_state = self.initial_state.clone();
    }

    fn simulate_one_tick(&mut self) {
        // Apply gravity
        for i in 0..self.current_state.len() {
            for j in i+1..self.current_state.len() {
                let force = self.current_state[i].force(&self.current_state[j]);
                self.current_state[i].vel += force;
                self.current_state[j].vel -= force;
            }
        }

        // Update position
        for moon in self.current_state.iter_mut() {
            moon.tick();
        }

        // Tick
        self.t += 1;
    }

    fn total_energy(&self) -> i32 {
        let mut total_energy = 0;
        for moon in &self.current_state {
            total_energy += moon.energy();
        }

        total_energy
    }
}

 fn gcd(a: usize, b: usize) -> usize {
     if b == 0 {
         a
     } else {
         gcd(b, a % b)
     }
 }

#[derive(Clone)]
struct Moon {
    pos: Triple,
    vel: Triple,
}

impl Moon {
    fn new(position: Triple) -> Moon {
        Moon { pos: position, vel: Triple::new(0, 0, 0) }
    }

    fn force(&self, other: &Moon) -> Triple {
        let dx = if self.pos.x > other.pos.x { -1 } else if self.pos.x < other.pos.x { 1 } else { 0 };
        let dy = if self.pos.y > other.pos.y { -1 } else if self.pos.y < other.pos.y { 1 } else { 0 };
        let dz = if self.pos.z > other.pos.z { -1 } else if self.pos.z < other.pos.z { 1 } else { 0 };

        Triple::new(dx, dy, dz)
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

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct Triple {
    x: i32,
    y: i32,
    z: i32,
}

impl Triple {
    const fn new(x: i32, y: i32, z: i32) -> Triple {
        Triple { x, y, z }
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:3}, {:3}, {:3}>", self.x, self.y, self.z)
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

impl ops::SubAssign for Triple {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Neg for Triple {
    type Output = Triple;

    fn neg(self) -> Self::Output {
        Triple { x: -self.x, y: -self.y, z: -self.y }
    }
}
