//! Advent of Code 2022: Day 19
//! https://adventofcode.com/2022/day/19

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

const T1_MAX: u64 = 24;
const T2_MAX: u64 = 32;
const N_RESOURCES: usize = 4;
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    t: u64,
    robots: [u64; 4],
    resources: [u64; 4],
}

fn part1(input: &Input) -> u64 {
    let mut quality = Vec::new();
    for blueprint in &input.blueprints {
        let best = find_best(blueprint, T1_MAX);
        quality.push(blueprint.n * best.resources[GEODE]);
    }

    quality.into_iter().sum()
}

fn part2(input: &Input) -> u64 {
    let mut geodes = Vec::new();
    for blueprint in &input.blueprints[..3] {
        let best = find_best(blueprint, T2_MAX);
        geodes.push(best.resources[GEODE]);
    }

    geodes.into_iter().product()
}

/// Find the solution that maximizes the number of Geodes.
fn find_best(blueprint: &Blueprint, t_max: u64) -> State {
    let mut results = Vec::new();

    let mut seen = HashSet::new();
    let mut edge: Vec<State> = vec![State { t: 0, robots: [1, 0, 0, 0], resources: [0, 0, 0, 0] }];
    while let Some(state) = edge.pop() {
        if state.t == t_max {
            results.push(state);
            continue;
        }

        for adj in adjacent(blueprint, &state) {
            let key = cache_key(blueprint, adj);
            if seen.contains(&key) {
                continue;
            }

            edge.push(adj);
            seen.insert(key);
        }

        edge.sort_by_key(|s| {
            s.resources[GEODE] // current
            + (t_max - s.t) * s.robots[GEODE] // future mined
        });
    }

    results.sort_by_key(|s| s.resources[GEODE]);

    *results.last().unwrap()
}

fn cache_key(blueprint: &Blueprint, state: State) -> State {
    let mut state = state;
    for r in 0..N_RESOURCES {
        // Ignore any resources above what we could possibly need
        if (0..N_RESOURCES).into_iter().all(|n| state.resources[r] > blueprint.requirements[&n].get(&r).copied().unwrap_or(0) + 1) {
            state.resources[r] = u64::MAX;
        }
    }

    state
}

fn adjacent(blueprint: &Blueprint, state: &State) -> Vec<State> {
    let mut states = Vec::new();

    // We can always just mine
    let mut resources = state.resources;
    for r in 0..resources.len() {
        resources[r] += state.robots[r];
    }

    states.push(State { t: state.t + 1, robots: state.robots, resources });

    // We can try to build a robot
    for kind in 0..state.robots.len() {
        let mut resources = state.resources;

        // Do we have the resources?
        if !(0..state.resources.len()).all(|r| resources[r] >= blueprint.requirements[&kind].get(&r).copied().unwrap_or(0)) {
            continue;
        }

        // Build a robot
        for r in 0..resources.len() {
            resources[r] -= blueprint.requirements[&kind].get(&r).copied().unwrap_or(0);
            resources[r] += state.robots[r];
        }

        let mut robots = state.robots;
        robots[kind] += 1;

        states.push(State { t: state.t + 1, robots, resources });
    }


    states
}

#[derive(Debug, Clone)]
struct Input {
    blueprints: Vec<Blueprint>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let re = regex::Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

        let mut blueprints = Vec::new();
        for line in input.lines() {
            let mut requirements = HashMap::new();

            let captures = re.captures(line.trim()).unwrap();
            let values: Vec<u64> = (1..captures.len()).map(|n| captures[n].parse().unwrap()).collect();

            requirements.insert(
                ORE, [
                    (ORE, values[1]),
                ].into_iter().collect()
            );

            requirements.insert(
                CLAY, [
                    (ORE, values[2]),
                ].into_iter().collect()
            );

            requirements.insert(
                OBSIDIAN, [
                    (ORE, values[3]),
                    (CLAY, values[4]),
                ].into_iter().collect()
            );

            requirements.insert(
                GEODE, [
                    (ORE, values[5]),
                    (OBSIDIAN, values[6]),
                ].into_iter().collect()
            );

            blueprints.push(Blueprint {
                n: values[0],
                requirements,
            });
        }

        Ok(Input { blueprints })
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    n: u64,
    requirements: HashMap<usize, HashMap<usize, u64>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(find_best(&input.blueprints[0], T2_MAX).resources[GEODE], 56);
        assert_eq!(find_best(&input.blueprints[1], T2_MAX).resources[GEODE], 62);
    }
}
