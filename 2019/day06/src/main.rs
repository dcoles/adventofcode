use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};

type Obj = [char; 3];

fn main() {
    let input = read_input("input.txt");

    // Part 1
    // Maps object to the object it's orbiting
    let mut orbits: HashMap<Obj, Obj> = HashMap::new();
    for &(a, b) in &input {
        orbits.insert(b, a);
    }

    let mut count = 0;
    for &obj in orbits.keys() {
        let mut obj = obj;
        while let Some(&parent) = orbits.get(&obj) {
            obj = parent;
            count += 1;
        }
    }

    println!("Part 1: Count of direct and indirect orbits {}", count);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<(Obj, Obj)> {
    let mut orbits = Vec::new();
    let contents = fs::read_to_string(path).expect("Failed to read input");
    for line in contents.lines() {
        let mut vals = line.split(')');
        let a = vals.next().unwrap().as_bytes();
        let b = vals.next().unwrap().as_bytes();
        let obja = [char::from(a[0]), char::from(a[1]), char::from(a[2])];
        let objb = [char::from(b[0]), char::from(b[1]), char::from(b[2])];
        orbits.push((obja , objb))
    }

    orbits
}
