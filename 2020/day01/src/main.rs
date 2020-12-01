use std::path::Path;
use std::fs;
use std::vec::Vec;

const TARGET: i32 = 2020;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input).expect("No result?"));
    println!("Part 2: {}", part2(&input).expect("No result?"));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<i32> {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse::<i32>().expect("Failed to parse line"))
        .collect()
}

fn part1(input: &[i32]) -> Option<i32> {
    for i in input {
        for j in input {
            if i + j == TARGET {
                return Some(i * j);
            }
        }
    }

    None
}

fn part2(input: &[i32]) -> Option<i32> {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == TARGET {
                    return Some(i * j * k);
                }
            }
        }
    }

    None
}
