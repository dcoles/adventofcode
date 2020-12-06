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
        .map(|line| line.lines().map(|l| l.to_owned()).collect())
        .collect()
}

fn count_any(groups: &Vec<Vec<String>>) -> usize {
    let mut count = 0;
    for group in groups {
        let mut set = HashSet::new();
        for person in group {
            set.extend(person.chars());
        }
        count += set.len();
    }

    count
}

fn count_all(groups: &Vec<Vec<String>>) -> usize {
    let mut count = 0;
    for group in groups {
        let mut set: HashSet<_> = ('a'..='z').collect();
        for person in group {
            let questions: HashSet<_> = person.chars().collect();
            set = set.intersection(&questions).copied().collect();
        }
        count += set.len();
    }

    count
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

