use std::path::Path;
use std::fs;

const FRONT: char = 'F';
const BACK: char = 'B';
const LEFT: char = 'L';
const RIGHT: char = 'R';

fn main() {
    let input = read_input("input.txt");
    let max_seatid = input.iter().map(|s| seatid(&s)).max().unwrap();

    println!("Part 1: Max Seat ID is {}", max_seatid);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<String> {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn seatid(seq: &str) -> u32 {
    let row = partition(&seq[..7]);
    let column = partition(&seq[7..]);

    row * 8 + column
}

fn partition(seq: &str) -> u32 {
    let mut min = 0;
    let mut max = 2_u32.pow(seq.len() as u32) - 1;

    for c in seq.chars() {
        let mid = min + (max - min) / 2;
        match c {
            FRONT | LEFT => max = mid,
            BACK | RIGHT => min = mid + 1,
            _ => panic!("Unknown character"),
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let seq = "FBFBBFFRLR";
        assert_eq!(partition(&seq[..7]), 44);
        assert_eq!(partition(&seq[7..]), 5);
    }
}
