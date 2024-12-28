//! Advent of Code 2024: Day 23
//! https://adventofcode.com/2024/day/23

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let edges = get_edges(&input.values);
    let keys: Vec<_> = edges.keys().map(|s| s.to_owned()).collect();

    let mut tris: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    for i in 0..keys.len() {
        for j in (i+1)..keys.len() {
            for k in (j+1)..keys.len() {
                let a = keys[i].as_str();
                let b = keys[j].as_str();
                let c = keys[k].as_str();

                if [b, c].iter().all(|&x| edges[a].contains(x))
                    && [a, c].iter().all(|&x| edges[b].contains(x))
                    && [a, b].iter().all(|&x| edges[c].contains(x)) {
                    tris.insert([a.to_string(), b.to_string(), c.to_string()].into());
                }
            }
        }
    }

    tris.iter().filter(|set| set.iter().any(|s| s.starts_with("t"))).count()
}

fn get_edges(values: &[(String, String)]) -> BTreeMap<String, BTreeSet<String>> {
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for (a, b) in values {
        edges.entry(a.to_string()).or_default().insert(b.to_string());
        edges.entry(b.to_string()).or_default().insert(a.to_string());
    }

    edges
}

fn maximal_cliques(edges: &BTreeMap<String, BTreeSet<String>>) -> BTreeSet<BTreeSet<String>> {
    let keys: Vec<_> = edges.keys().map(|s| s.to_owned()).collect();
    let mut cliques = BTreeSet::new();

    for node in keys.iter() {
        let mut clique: BTreeSet<String> = [node.to_string()].into();

        for other in keys.iter().filter(|n| n.as_str() != node.as_str()) {
            if clique.iter().all(|n| edges[other].contains(n)) {
                clique.insert(other.to_string());
            }
        }

        cliques.insert(clique);
    }

    cliques
}

fn part2(input: &Input) -> String {
    let edges = get_edges(&input.values);
    let cliques = maximal_cliques(&edges);

    let (_, largest) = cliques.into_iter().map(|c| (c.len(), c)).max().unwrap();
    let password: Vec<_> = largest.into_iter().collect();

    password.join(",")
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(String, String)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines()
            .map(|s| {
                let (a, b) = s.split_once('-').unwrap();

                (a.to_string(), b.to_string())
            })
            .collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1108);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), "co,de,ka,ta");
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), "ab,cp,ep,fj,fl,ij,in,ng,pl,qr,rx,va,vf");
    }
}
