use std::path::Path;
use std::fs;
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";
const NO_OTHER: &str = "no other";

fn main() {
    let rules = read_input("input.txt");

    println!("Part 1: {}", part1(&rules));
    println!("Part 2: {}", part2(&rules));
}

fn part1(rules: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut count = 0;
    for starting_bag in rules.keys() {
        if starting_bag == SHINY_GOLD {
            continue;
        }

        let mut stack = vec![starting_bag];
        while let Some(bag) = stack.pop() {
            if bag == SHINY_GOLD {
                count += 1;
                break;
            }

            let children = rules.get(bag).unwrap();
            stack.extend(children.iter().map(|(_, child)| child));
        }
    }

    count
}

fn part2(rules: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut count = 0;

    let shiny_gold = SHINY_GOLD.to_owned();
    let mut stack = vec![(1, &shiny_gold)];
    while let Some((q, bag)) = stack.pop() {
        let children = rules.get(bag).unwrap();
        stack.extend(children.iter().map(|(n, child)| (q * n, child)));
        count += children.iter().map(|(n, _)| q * n).sum::<usize>();
    }

    count
}

fn read_input<T: AsRef<Path>>(path: T) -> HashMap<String, Vec<(usize, String)>> {
    let mut map: HashMap<String, Vec<_>> = HashMap::new();
    for line in fs::read_to_string(path).expect("Failed to read input").lines() {
        let mut it = line.split(" contain ");
        let bag = trim_bag(it.next().unwrap());
        for contents in it.next().unwrap().split(",").map(|s| trim_bag(s)) {
            let entry = map.entry(bag.to_owned()).or_default();
            if contents == NO_OTHER {
                continue;
            }

            let p = contents.find(" ").unwrap();
            let quantity = contents[..p].parse().expect("Failed to parse quantity");
            let child_bag = contents[p+1..].to_owned();

            entry.push((quantity, child_bag));
        }
    }

    map
}

fn trim_bag(s: &str) -> &str {
    s.trim_end_matches(".").trim_end_matches("s").trim_end_matches("bag").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample1() {
        let rules = read_input("sample1.txt");
        assert_eq!(part1(&rules), 4);
    }

    #[test]
    fn test_part2_sample1() {
        let rules = read_input("sample1.txt");
        assert_eq!(part2(&rules), 32);
    }
}

