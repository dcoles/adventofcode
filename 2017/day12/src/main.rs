use std::path::Path;
use std::io::Error;
use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};

fn main() {
    let edges = read_input("input.txt").expect("Failed to read input");

    // Part 1
    let mut visited = HashSet::new();
    visited.insert(0);
    let mut edge = Vec::new();
    edge.push(0);

    // Do DFS
    while let Some(cur) = edge.pop() {
        for &adj in edges.get(&cur).unwrap_or(&HashSet::new()) {
            if !visited.contains(&adj) {
                edge.push(adj);
                visited.insert(adj);
            }
        }
    }

    println!("Number of programs reachable from PID 0: {}", visited.len());

    // Part 2
    let mut ngroups = 0;
    let mut visited = HashSet::new();
    for &pid in edges.keys() {
        if visited.contains(&pid) {
            // This pid is already in a group
            continue;
        }

        ngroups += 1;
        visited.insert(pid);
        let mut edge = Vec::new();
        edge.push(pid);

        // Do DFS
        while let Some(cur) = edge.pop() {
            for &adj in edges.get(&cur).unwrap_or(&HashSet::new()) {
                if !visited.contains(&adj) {
                    edge.push(adj);
                    visited.insert(adj);
                }
            }
        }
    }

    println!("Number of groups: {}", ngroups);
}

fn read_input<P: AsRef<Path>>(path: P) -> Result<HashMap<usize, HashSet<usize>>, Error> {
    let mut edges = HashMap::new();
    for line in read_to_string(path)?.lines() {
        let parts: Vec<_> = line.split(" <-> ").collect();
        let a: usize = parts[0].parse().unwrap();
        let bs: HashSet<usize> = parts[1].split(", ").map(|v| v.parse().unwrap()).collect();
        edges.insert(a, bs);
    }
    Ok(edges)
}
