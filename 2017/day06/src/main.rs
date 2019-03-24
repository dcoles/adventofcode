use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;
use std::io;

fn main() {
    let input = read_input("input.txt").expect("Failed to read input");

    // Part 1
    let mut banks = input.clone();
    let mut seen = HashSet::new();
    seen.insert(banks.clone());
    for n in 1.. {
        redistribute(&mut banks);
        if seen.contains(&banks) {
            println!("[Part 1] Redistribution cycles {}", n);
            break;
        }
        seen.insert(banks.clone());
    }

    // Part 2
    let seen_state = banks.clone();
    for n in 1.. {
        redistribute(&mut banks);
        if banks == seen_state {
            println!("[Part 2] Redistribution cycles {}", n);
            break;
        }
    }
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<i32>> {
    Ok(read_to_string(path)?.split_whitespace().filter_map(|v| v.parse().ok()).collect())
}

fn redistribute(banks: &mut Vec<i32>) {
    let mut max_idx = 0;
    let mut max = banks[max_idx];
    for idx in 0..banks.len() {
        if banks[idx] > max {
            max_idx = idx;
            max = banks[idx];
        }
    }

    let mut blocks = banks[max_idx];
    banks[max_idx] = 0;

    let mut idx = (max_idx + 1) % banks.len();
    while blocks > 0 {
        banks[idx] += 1;
        blocks -= 1;
        idx = (idx + 1) % banks.len()
    }
}
