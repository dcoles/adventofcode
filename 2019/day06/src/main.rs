use std::{fs, fmt};
use std::path::Path;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Obj([u8; 3]);

impl Obj {
    const fn from_str(s: &str) -> Obj {
        let bytes = s.as_bytes();
        Obj([bytes[0], bytes[1], bytes[2]])
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

const COM: Obj = Obj::from_str("COM");
const YOU: Obj = Obj::from_str("YOU");
const SANTA: Obj = Obj::from_str("SAN");

fn main() {
    let input = read_input("input.txt");

    // Part 1
    // Maps object to the object it's orbiting
    let mut obj_orbits: HashMap<Obj, Obj> = HashMap::new();
    for &(a, b) in &input {
        obj_orbits.insert(b, a);
    }

    let mut count = 0;
    for &obj in obj_orbits.keys() {
        count += orbital_distances(&obj_orbits, obj).len();
    }

    println!("Part 1: Count of direct and indirect orbits {}", count);

    // Part 2
    let santa_orbital_dist = orbital_distances(&obj_orbits, SANTA);
    let you_orbital_dist = orbital_distances(&obj_orbits, YOU);

    let mut santa_orbits_by_dist: Vec<Obj> = santa_orbital_dist.keys().cloned().collect();
    santa_orbits_by_dist.sort_by_key(|&obj| santa_orbital_dist[&obj]);

    // Find the first common object
    let mut common_obj = COM;
    for &obj in &santa_orbits_by_dist {
        if you_orbital_dist.contains_key(&obj) {
            common_obj = obj;
            break;
        }
    }

    // How many transfers are required to get to the common object
    let transfers = santa_orbital_dist[&common_obj] + you_orbital_dist[&common_obj];
    println!("Part 2: Orbital transfers required {}", transfers);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<(Obj, Obj)> {
    let mut orbits = Vec::new();
    let contents = fs::read_to_string(path).expect("Failed to read input");
    for line in contents.lines() {
        let mut vals = line.split(')');
        let obja = Obj::from_str(vals.next().expect("Missing value"));
        let objb = Obj::from_str(vals.next().expect("Missing value"));
        orbits.push((obja , objb))
    }

    orbits
}

fn orbital_distances(obj_orbits: &HashMap<Obj, Obj>, start: Obj) -> HashMap<Obj, u32> {
    let mut orbits = HashMap::new();
    let mut obj = start;
    let mut dist = 0;
    while let Some(&parent) = obj_orbits.get(&obj) {
        orbits.insert(parent, dist);
        obj = parent;
        dist += 1;
    }

    orbits
}
