//! Advent of Code 2021: Day 12
//! https://adventofcode.com/2021/day/12

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = read_input_from_file("day12/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));
    
    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &HashMap<Node, Vec<Node>>) -> usize {
    let mut paths = Vec::new();

    let mut edge: VecDeque<_> = [vec![Node::Start]].into_iter().collect();

    while let Some(path) = edge.pop_front() {
        let cur = path.last().unwrap();

        for adj in &input[cur] {
            if adj.is_start() || adj.is_small() && path.contains(adj) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(adj.clone());

            if adj.is_end() {
                paths.push(new_path);
            } else {
                edge.push_back(new_path);
            }
        }
    }

    paths.len()
}

fn part2(input: &HashMap<Node, Vec<Node>>) -> usize {
    let mut paths = Vec::new();

    let mut edge: VecDeque<_> = [vec![Node::Start]].into_iter().collect();

    while let Some(path) = edge.pop_front() {
        let cur = path.last().unwrap();

        for adj in &input[cur] {
            if adj.is_start() {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(adj.clone());

            if !is_valid(&new_path) {
                continue;
            }

            if adj.is_end() {
                paths.push(new_path);
            } else {
                edge.push_back(new_path);
            }
        }
    }

    paths.len()
}

fn is_valid(path: &[Node]) -> bool {
    let small: Vec<_> = path.iter().filter(|&n| n.is_small()).collect();
    let small_deduped: HashSet<_> = small.iter().collect();

    // Can have up to one duplicate entry
    small_deduped.len() + 1 >= small.len()
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<HashMap<Node, Vec<Node>>> {
    let input = fs::read_to_string(path)?;

    let mut result: HashMap<Node, Vec<Node>> = HashMap::new();
    for lines in input.lines() {
        let (a, b) = lines.split_once("-").unwrap();
        let a = Node::from_str(a);
        let b = Node::from_str(b);

        result.entry(a.clone()).or_default().push(b.clone());
        result.entry(b).or_default().push(a);
    }

    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Node {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => Node::Start,
            "end" => Node::End,
            x if x.chars().all(|c| c.is_ascii_lowercase()) => Node::Small(x.to_string()),
            x => Node::Big(x.to_string()),
        }
    }

    fn is_start(&self) -> bool {
        matches!(self, Node::Start)
    }

    fn is_end(&self) -> bool {
        matches!(self, Node::End)
    }

    fn is_small(&self) -> bool {
        matches!(self, Node::Small(_))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn test_part1_example2() {
        let input = read_input_from_file("example2.txt").expect("failed to read input");

        assert_eq!(part1(&input), 19);
    }
    
    #[test]
    fn test_part1_example3() {
        let input = read_input_from_file("example3.txt").expect("failed to read input");

        assert_eq!(part1(&input), 226);
    }

    #[test]
    fn test_part2_example1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 36);
    }
    
    #[test]
    fn test_part2_example2() {
        let input = read_input_from_file("example2.txt").expect("failed to read input");

        assert_eq!(part2(&input), 103);
    }

    #[test]
    fn test_part2_example3() {
        let input = read_input_from_file("example3.txt").expect("failed to read input");

        assert_eq!(part2(&input), 3509);
    }
}
