use std::path::Path;
use std::fs;
use std::collections::HashSet;

fn main() {
    let groups = read_input("input.txt");

    println!("Part 1: Sum of count of questions ANY person answered yes is {}", count_any(&groups));
    println!("Part 2: Sum of count of questions ALL people answered yes is {}", count_all(&groups));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Vec<String>> {
    fs::read_to_string(path).expect("Failed to read input")
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.to_owned()).collect())
        .collect()
}

fn count_any(groups: &[Vec<String>]) -> usize {
    groups.iter().map(|group| {
        let set: HashSet<_> = group.iter().flat_map(|p| p.chars()).collect();
        set.len()
    }).sum()
}

fn count_all(groups: &[Vec<String>]) -> usize {
    groups.iter().map(|group| {
        let mut set: HashSet<_> = ('a'..='z').collect();
        for person in group {
            let questions: HashSet<_> = person.chars().collect();
            set = set.intersection(&questions).copied().collect();
        }
        set.len()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let groups = read_input("input1.txt");
        assert_eq!(count_any(&groups), 11);
    }

    #[test]
    fn test_part2() {
        let groups = read_input("input1.txt");
        println!("{}", count_all(&groups));
        assert_eq!(count_all(&groups), 6);
    }
}

