use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");
    println!("Checksum: {}", checksum(&input));

    for line1 in input.lines() {
        for line2 in input.lines() {
            let common = common(line1, line2);
            if common.len() == line1.len() - 1 {
                println!("Close match! {}", common)
            }
        }
    }
}

fn checksum(input: &str) -> u32 {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let counts = count_letters(line);
        if counts.values().any(|x| *x == 2) {
            twos += 1;
        }
        if counts.values().any(|x| *x == 3) {
            threes += 1;
        }
    }
    twos * threes
}

fn count_letters(str: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for c in str.chars() {
        *counts.entry(c).or_default() += 1;
    }
    counts
}

fn common(str1: &str, str2: &str) -> String {
    let mut common = String::new();
    for (char1, char2) in str1.chars().zip(str2.chars()) {
        if char1 == char2 {
            common.push(char1);
        }
    }
    return common;
}
