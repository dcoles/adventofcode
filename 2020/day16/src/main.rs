use std::path::Path;
use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");
    let mut sections = input.split("\n\n");

    let fields_re = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let fields: HashMap<String, HashSet<u32>> = sections.next().expect("Failed to read section")
        .lines().map(|line| {
            let m = fields_re.captures(line).expect("Failed to parse field");
            let key = m[1].to_owned();
            let a = m[2].parse().unwrap();
            let b = m[3].parse().unwrap();
            let c = m[4].parse().unwrap();
            let d = m[5].parse().unwrap();
            let values: HashSet<u32> = (a..=b).chain(c..=d).collect();

            (key, values)
        }).collect();

    let ticket = sections.next().expect("Failed to read section")
        .lines().skip(1).next().expect("Failed to read line")
        .split(',').map(|val| val.parse().expect("Failed to parse number"))
        .collect();

    let nearby = sections.next().expect("Failed to read section")
        .lines().skip(1).map(|line| {
            line.split(',').map(|val| val.parse().expect("Failed to parse number")).collect()
        }).collect();

    Input { fields, ticket, nearby }
}

fn part1(input: &Input) -> u32 {
    input.nearby.iter()
        .flat_map(|ticket| ticket.iter().filter(|&val| invalid(&input.fields, *val)))
        .sum()
}

fn invalid(fields: &HashMap<String, HashSet<u32>>, value: u32) -> bool {
    !fields.values().any(|set| set.contains(&value))
}

#[derive(Debug)]
struct Input {
    fields: HashMap<String, HashSet<u32>>,
    ticket: Vec<u32>,
    nearby: Vec<Vec<u32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 71);
    }
}

