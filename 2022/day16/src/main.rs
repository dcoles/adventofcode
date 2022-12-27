//! Advent of Code 2022: Day 16
//! https://adventofcode.com/2022/day/16

use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::io;
use std::ops::Index;
use std::path::Path;

const START: &str = "AA";
const TIME: usize = 30; // min
const TIME_2: usize = 26; // min

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let start = input[START].id;

    // Use a branch and bound algorithm to find the best possible result.
    let mut edge: Vec<_> = vec![vec![start]];
    let mut best = (edge[0].clone(), 0);
    while let Some(path) = edge.pop() {
        for adj in filter_visited::<Vec<Id>>(input.to_open.iter().copied(), &path) {
            let mut path = path.clone();
            path.push(adj);

            let (score, t_remaining) = match score_path(input, &path, TIME) {
                None => continue,
                Some(s) => s,
            };

            if score > best.1 {
                best = (path.clone(), score);
            } else {
                // How much could we possibly still release from this point?
                let possible = estimate_possible(input, &path, t_remaining);

                if score + possible < best.1 {
                    // It's not possible for this path to ever beat the best score
                    continue;
                }
            }

            edge.push(path);
        }
    }

    best.1
}

fn part2(input: &Input) -> usize {
    let start = input[START].id;

    // Use a branch and bound algorithm to find the best possible result.
    let mut seen = HashSet::new();
    let mut edge: Vec<_> = vec![[vec![start], vec![start]]];
    let mut scores: HashMap<Vec<Id>, usize> = [(vec![start], 0)].into_iter().collect();

    let mut best = (edge[0].clone(), 0);
    while let Some(paths) = edge.pop() {
        for adj in input.to_open.iter().copied().filter(|&v| !paths.iter().any(|p| p.contains(&v))) {
            for n in 0..2 {
                let mut paths = paths.clone();
                paths[n].push(adj);

                if seen.contains(&paths) {
                    continue;
                }

                seen.insert(paths.clone());

                let (score, t_remaining) = match score_path(input, &paths[n], TIME_2) {
                    None => continue,
                    Some(s) => s,
                };

                scores.insert(paths[n].clone(), score);

                let total_score = score + paths.iter().enumerate().filter(|&(m, _)| m != n).map(|(_, p)| scores[p]).sum::<usize>();

                if total_score > best.1 {
                    best = (paths.clone(), total_score);
                } else {
                    // How much could we possibly still release from this point?
                    let possible: usize = paths.iter().map(|p| estimate_possible(input, &p, t_remaining)).sum();

                    if total_score + possible < best.1 {
                        // It's not possible for this path to ever beat the best score
                        continue;
                    }
                }

                edge.push(paths);
            }
        }
    }

    best.1
}

/// An estimate of pressure that could still be released in `t_remaining` minutes after following `path`.
/// Guarenteed to never underestimate.
fn estimate_possible(input: &Input, path: &[Id], t_remaining: usize) -> usize {
    let mut unvisited: Vec<_> = filter_visited(input.to_open.iter().copied(), path);
    unvisited.sort_by_key(|&v| input[v].rate);

    let mut t_remaining = t_remaining;
    let mut possible = 0;
    while let Some(v) = unvisited.pop() {
        if t_remaining < 2 {
            break;
        }

        t_remaining -= 2;
        possible += t_remaining * input[v].rate;
    }

    possible
}

/// Calculate the pressure that can be released in `t_remaining` minutes by following `path`.
/// Returns `(pressure, t_remaining)`.
fn score_path(input: &Input, path: &[Id], t_remaining: usize) -> Option<(usize, usize)> {
    let mut t_remaining = t_remaining;
    let mut score = 0;

    for i in 0..path.len() {
        if input[path[i]].rate > 0 && t_remaining > 0 {
            // Open this valve
            t_remaining -= 1;
            // Only starts working the following minute
            score += t_remaining * input[path[i]].rate;
        }

        if i < path.len() - 1 {
            // move to next valve
            t_remaining = t_remaining.checked_sub(input.distances[&(path[i], path[i + 1])])?
        }
    }

    Some((score, t_remaining))
}

/// Create a new collection which doesn't contain any values from `path`.
fn filter_visited<B>(collection: impl IntoIterator<Item=Id>, path: &[Id]) -> B
    where B: FromIterator<Id>
{
    collection.into_iter().filter(|v| !path.contains(v)).collect::<B>()
}

/// Calculate distances from all valves to all other valves.
fn distances(valves: &[Valve], ids: &HashMap<Name, Id>) -> HashMap<(Id, Id), usize> {
    let adjacent: HashMap<Id, Vec<Id>> = valves.iter().map(|v| (ids[&v.name], v.adjacent.iter().map(|a| ids[a]).collect())).collect();

    let mut distances = HashMap::new();
    for n1 in valves.iter().map(|v| ids[&v.name]) {
        for n2 in valves.iter().map(|v| ids[&v.name]) {
            if n1 == n2 {
                distances.insert((n1, n2), 0);
            }

            let mut best: HashMap<Id, usize> = [(n1, 0)].into_iter().collect();
            let mut edge = vec![n1];
            while let Some(n) = edge.pop() {
                if n == n2 {
                    distances.insert((n1, n2), best[&n2]);
                    break;
                }

                for &adj in &adjacent[&n] {
                    let cost = best[&n] + 1;
                    if !best.contains_key(&adj) || cost < best[&adj] {
                        edge.push(adj);
                        best.insert(adj, cost);
                    }
                }
                edge.sort_by(|a, b| (best[b]).cmp(&best[a]));
            }
        }
    }

    distances
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name(Box<str>);

impl Name {
    fn new(name: &str) -> Self {
        Self(Box::from(name))
    }
}

impl Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &*self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id(usize);

#[derive(Debug, Clone)]
struct Input {
    valves: Vec<Valve>,
    ids: HashMap<Name, Id>,
    to_open: HashSet<Id>,
    distances: HashMap<(Id, Id), usize>,
}

impl Index<Id> for Input {
    type Output = Valve;

    fn index(&self, index: Id) -> &Self::Output {
        &self.valves[index.0]
    }
}

impl Index<Name> for Input {
    type Output = Valve;

    fn index(&self, index: Name) -> &Self::Output {
        &self.valves[self.ids[&index].0]
    }
}

impl Index<&str> for Input {
    type Output = Valve;

    fn index(&self, index: &str) -> &Self::Output {
        &self.valves[self.ids[index].0]
    }
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let re = regex::Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

        let mut ids = HashMap::new();
        let mut valves = Vec::new();
        let mut to_open = HashSet::new();
        for (n, line) in input.lines().enumerate() {
            let captures = re.captures(line).unwrap();

            let id = Id(n);
            let name = Name::new(&captures[1]);
            let rate: usize = captures[2].parse().unwrap();
            let adjacent: Vec<_> = captures[3].split(", ").map(|s| Name::new(s)).collect();

            if rate > 0 {
                to_open.insert(id);
            }

            ids.insert(name.clone(), id);
            valves.push(Valve { id, name, rate, adjacent });
        }

        let distances = distances(&valves, &ids);

        Ok(Input { valves, ids, to_open, distances })
    }
}

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    id: Id,
    rate: usize,
    adjacent: Vec<Name>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        let input = Input::from_file("example1.txt").unwrap();

        let aa = input.ids["AA"];
        let dd = input.ids["DD"];
        let bb = input.ids["BB"];
        let jj = input.ids["JJ"];
        let hh = input.ids["HH"];
        let ee = input.ids["EE"];
        let cc = input.ids["CC"];

        assert_eq!(score_path(&input, &[aa], 30), Some((0, 30)));
        assert_eq!(score_path(&input, &[aa, dd], 30), Some((560, 28)));
        assert_eq!(score_path(&input, &[aa, dd, bb], 30), Some((885, 25)));
        assert_eq!(score_path(&input, &[aa, dd, bb, jj, hh, ee, cc], 30), Some((1651, 6)));
    }

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 1651);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 1707);
    }
}
