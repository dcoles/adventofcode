use std::path::Path;
use std::fs;
use std::vec::Vec;

const TARGET: i32 = 2020;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<i32> {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse::<i32>().expect("Failed to parse line"))
        .collect()
}

fn part1(input: &[i32]) -> i32 {
    // We don't need to compare items to themselves
    // or other combinations we've already tried
    for (i, entry1) in input.iter().enumerate() {
        for entry2 in input.iter().skip(i + 1) {
            if entry1 + entry2 == TARGET {
                return entry1 * entry2;
            }
        }
    }

    panic!("No result found!");
}

fn part2(input: &[i32]) -> i32 {
    // We don't need to compare items to themselves
    // or other combinations we've already tried
    for (i, entry1) in input.iter().enumerate() {
        for (j, entry2) in input.iter().skip(i + 1).enumerate() {
            for entry3 in input.iter().skip(i + j + 2) {
                if entry1 + entry2 + entry3 == TARGET {
                    return entry1 * entry2 * entry3;
                }
            }
        }
    }

    panic!("No result found!");
}
