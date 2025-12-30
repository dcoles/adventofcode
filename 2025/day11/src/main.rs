//! Advent of Code 2025: Day X
//! <https://adventofcode.com/2025/day/X>

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::AddAssign;
use std::{fs, io};
use std::path::Path;

const YOU: &str = "you";
const OUT: &str = "out";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

type Dependencies = HashMap<String, Vec<String>>;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");
    println!("Part 1: {}", part1(&input));

    // Part 2
    //let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");
    println!("Part 2: {}", part2(&input));
}

fn reverse_dependencies(deps: &Dependencies) -> Dependencies {
    let mut reverse_deps: Dependencies = HashMap::new();
    for (from, to) in deps {
        for each in to {
            reverse_deps.entry(each.to_owned()).or_default().push(from.into());
        }
    }

    reverse_deps
}

fn part1(input: &Input) -> usize {
    let reverse_deps = reverse_dependencies(&input.dependencies);

    let mut path_counts: HashMap<String, usize> = [
        (YOU.into(), 1),
    ].into();

    let mut to_eval = vec![OUT.to_owned()];

    while !to_eval.is_empty() {
        let node = to_eval.last().unwrap().to_owned();
        println!("{node}");

        if path_counts.contains_key(&node) {
            // this node has already been evaluated
            continue;
        }

        if !reverse_deps.contains_key(&node) {
            // hit a dead-end
            path_counts.insert(node, 0);
            to_eval.pop();
            continue;
        }

        if !reverse_deps[&node].iter().all(|dep| path_counts.contains_key(dep)) {
            for dep in reverse_deps[&node].iter().filter(|&dep| !path_counts.contains_key(dep)) {
                to_eval.push(dep.to_owned());
            }

            // there were unevaluated dependencies, so have to deal with them first
            continue;
        }

        // we can now evaluate this node
        let count = reverse_deps[&node].iter().map(|dep| path_counts[dep]).sum();

        path_counts.insert(node, count);
        to_eval.pop();
    }

    path_counts[OUT]
}

#[derive(Debug, Clone, Copy, Default)]
struct Counts {
    visited_neither: usize,
    visited_dac: usize,
    visited_fft: usize,
    visited_both: usize,
}

impl AddAssign for Counts {
    fn add_assign(&mut self, rhs: Self) {
        self.visited_neither += rhs.visited_neither;
        self.visited_dac += rhs.visited_dac;
        self.visited_fft += rhs.visited_fft;
        self.visited_both += rhs.visited_both;
    }
}

fn part2(input: &Input) -> usize {
    let reverse_deps = reverse_dependencies(&input.dependencies);

    let mut path_counts: HashMap<String, Counts> = [
        (SVR.into(), Counts { visited_neither: 1, ..Default::default() }),
    ].into();

    let mut to_eval = vec![OUT.to_owned()];

    while !to_eval.is_empty() {
        let node = to_eval.last().unwrap().to_owned();

        if path_counts.contains_key(&node) {
            // this node has already been evaluated
            continue;
        }

        if !reverse_deps.contains_key(&node) {
            // hit a dead-end
            path_counts.insert(node, Counts::default());
            to_eval.pop();
            continue;
        }

        if !reverse_deps[&node].iter().all(|dep| path_counts.contains_key(dep)) {
            for dep in &reverse_deps[&node] {
                if path_counts.contains_key(dep) {
                    continue;
                }

                to_eval.push(dep.to_owned());
            }

            // there were unevaluated dependencies, so have to deal with them first
            continue;
        }

        // we can now evaluate this node
        let mut counts = Counts::default();

        for dep in &reverse_deps[&node] {
            let dep_counts = path_counts[dep];

            match node.as_str() {
                DAC => {
                    counts.visited_dac += dep_counts.visited_neither;
                    counts.visited_both += dep_counts.visited_fft;
                    //println!("dac: {next_counts:?}");
                },
                FFT => {
                    counts.visited_fft += dep_counts.visited_neither;
                    counts.visited_both += dep_counts.visited_dac;
                    //println!("fft: {next_counts:?}");
                },
                _ => counts += dep_counts,
            }
        }

        path_counts.insert(node, counts);
        to_eval.pop();
    }

    path_counts[OUT].visited_both
}

#[derive(Debug, Clone)]
struct Input {
    dependencies: HashMap<String, Vec<String>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
        for line in input.lines() {
            let (from, to) = line.split_once(": ").unwrap();
            for each in to.split_ascii_whitespace() {
                dependencies.entry(from.into()).or_default().push(each.into());
            }
        }

        Ok(Self { dependencies })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 590);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 319473830844560);
    }
}
