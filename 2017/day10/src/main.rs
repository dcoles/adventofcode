use std::fs::read_to_string;

const SIZE: usize = 256;
const SUFFIX: [usize; 5] = [17, 31, 73, 47, 23];
const N_ROUNDS: usize = 64;

fn main() {
    let input = read_to_string("input.txt")
        .expect("Failed reading input");

    // Part 1
    let lengths: Vec<usize> = input
        .trim()
        .split(",")
        .map(|s| s.parse().expect("Failed parsing int"))
        .collect();

    let mut list: Vec<_> = (0..SIZE).collect();
    let mut cur_pos = 0;
    let mut skip_size = 0;

    for &len in &lengths {
        reverse_range(&mut list, cur_pos, len);
        cur_pos += len + skip_size;
        skip_size += 1;
    }

    println!("Part 1: {}", list[0] * list[1]);

    // Part 2
    println!("Part 2: {}", hash(input.trim()));
}

fn hash(string: &str) -> String {
    let mut lengths = str_to_ascii(&string);
    lengths.extend_from_slice(&SUFFIX);
    let lengths = lengths;

    let mut list: Vec<_> = (0..SIZE).collect();
    let mut cur_pos = 0;
    let mut skip_size = 0;

    for _round in 0..N_ROUNDS {
        for &len in &lengths {
            reverse_range(&mut list, cur_pos, len);
            cur_pos += len + skip_size;
            skip_size += 1;
        }
    }

    list.chunks(16)
        .map(|c| c.iter().fold(0usize, |a, &b| a ^ b))
        .map(|x| format!("{:02x}", x))
        .collect()
}

fn reverse_range(list: &mut Vec<usize>, start: usize, len: usize) {
    for n in 0..len/2 {
        list.swap((start + n) % SIZE, (start + len - 1 - n) % SIZE);
    }
}

fn str_to_ascii(string: &str) -> Vec<usize> {
    string.chars().map(|c| c as usize).collect()
}

#[test]
fn test_empty() {
    assert_eq!(hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
}

#[test]
fn test_aoc() {
    assert_eq!(hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
}

#[test]
fn test_123() {
    assert_eq!(hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
}

#[test]
fn test_124() {
    assert_eq!(hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
