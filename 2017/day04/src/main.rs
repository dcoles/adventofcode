use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;
use std::io::Result;

fn main() {
    let input = read_input("input.txt").expect("Failed to read input");

    println!("[Part 1] Number of valid passphrases: {}", input.iter().filter(|p| valid(&p)).count());
    println!("[Part 2] Number of valid passphrases: {}", input.iter().filter(|p| no_anagrams(&p)).count());
}

fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    Ok(read_to_string(path)?.lines().map(|l| l.to_string()).collect())
}

fn valid(passphrase: &str) -> bool {
    let mut seen = HashSet::new();
    for word in passphrase.split_whitespace() {
        if seen.contains(word) {
            return false;
        }
        seen.insert(word);
    }
    true
}

fn no_anagrams(passphrase: &str) -> bool {
    let mut seen = HashSet::new();
    for word in passphrase.split_whitespace() {
        let mut word: Vec<char> = word.chars().collect();
        word.sort();
        if seen.contains(&word) {
            return false;
        }
        seen.insert(word);
    }
    true
}
