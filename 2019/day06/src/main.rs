use std::fs;
use std::path::Path;
use std::collections::HashMap;

type Obj = String;
type ObjRef<'a> = &'a str;
type Distance = u32;

const COM: ObjRef = "COM";
const YOU: ObjRef = "YOU";
const SANTA: ObjRef = "SAN";

fn main() {
    // Vector of (obj, orbiting_obj)
    let input = read_input("input.txt");

    // Part 1
    // Maps object to the object it's orbiting (so we need to flip the ordering)
    let obj_orbits: HashMap<_, _> = input.into_iter()
        .map(|(obj, orbiting_obj)| (orbiting_obj, obj))
        .collect();

    let mut count = 0;
    for obj in obj_orbits.keys() {
        count += orbital_distances(&obj_orbits, obj).len();
    }

    println!("Part 1: Count of direct and indirect orbits {}", count);

    // Part 2
    let santa_orbital_dist = orbital_distances(&obj_orbits, SANTA);
    let you_orbital_dist = orbital_distances(&obj_orbits, YOU);

    let mut santa_orbits_by_dist: Vec<_> = santa_orbital_dist.keys().cloned().collect();
    santa_orbits_by_dist.sort_by_key(|obj| santa_orbital_dist[obj]);

    // Find the first common object
    let mut common_obj = COM;
    for obj in &santa_orbits_by_dist {
        if you_orbital_dist.contains_key(obj) {
            common_obj = obj;
            break;
        }
    }

    // How many transfers are required to get to the common object
    let transfers = santa_orbital_dist[common_obj] + you_orbital_dist[common_obj];
    println!("Part 2: Orbital transfers required {}", transfers);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<(Obj, Obj)> {
    let mut orbits = Vec::new();
    let contents = fs::read_to_string(path).expect("Failed to read input");
    for line in contents.lines() {
        let mut vals = line.split(')');
        let obj = Obj::from(vals.next().expect("Missing value"));
        let orbiting_obj = Obj::from(vals.next().expect("Missing value"));
        orbits.push((obj, orbiting_obj))
    }

    orbits
}

fn orbital_distances(obj_orbits: &HashMap<Obj, Obj>, start: ObjRef) -> HashMap<Obj, Distance> {
    let mut orbits = HashMap::new();
    let mut obj = start;
    let mut dist = 0;
    while let Some(parent) = obj_orbits.get(obj) {
        orbits.insert(parent.clone(), dist);
        obj = parent;
        dist += 1;
    }

    orbits
}
