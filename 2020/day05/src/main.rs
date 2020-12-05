use std::path::Path;
use std::fs;
use std::collections::HashSet;

const ROWS_POW: usize = 7;  // 128 rows
const COLS_POW: usize = 3;  // 8 columns

const FRONT: char = 'F';
const BACK: char = 'B';
const LEFT: char = 'L';
const RIGHT: char = 'R';

fn main() {
    let input = read_input("input.txt");
    let seatids: HashSet<_> = input.iter().map(|s| seatid(s)).collect();
    let max_seatid = seatids.iter().max().unwrap().clone();
    let your_seatid = find_seat(&seatids, max_seatid);

    println!("Part 1: Max Seat ID is {}", max_seatid);
    println!("Part 2: Your Seat ID is {}", your_seatid);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<String> {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn seatid(seq: &str) -> u32 {
    assert_eq!(seq.len(), ROWS_POW + COLS_POW);

    let row = partition(&seq[..ROWS_POW]);
    let column = partition(&seq[ROWS_POW..]);

    row * 2_u32.pow(COLS_POW as u32) + column
}

fn partition(seq: &str) -> u32 {
    let mut min = 0;
    let mut max = 2_u32.pow(seq.len() as u32) - 1;

    for c in seq.chars() {
        let mid = min + (max - min) / 2;
        match c {
            FRONT | LEFT => max = mid,
            BACK | RIGHT => min = mid + 1,
            _ => panic!("Unknown character {}", c),
        }
    }

    min
}

fn find_seat(seatids: &HashSet<u32>, max_seatid: u32) -> u32 {
    for i in 1..(max_seatid-1) {
        // Must be an empty seat...
        if !seatids.contains(&i) {
            // ...with seats on either side
            if seatids.contains(&(i-1)) && seatids.contains(&(i+1)) {
                return i;
            }
        }
    }

    panic!("Could not find seat!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let seq = "FBFBBFFRLR";
        assert_eq!(partition(&seq[..ROWS_POW]), 44);
        assert_eq!(partition(&seq[ROWS_POW..]), 5);
    }
}
