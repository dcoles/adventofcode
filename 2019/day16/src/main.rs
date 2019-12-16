use std::{fs, iter};
use std::path::Path;

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn main() {
    let input = read_input("input.txt");

    // Part 1
    println!("Part 1: After 100 phases, the first 8 digits are: {:?}", &n_phases(&input, 100)[..8]);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    signal_to_list(&contents)
}

fn signal_to_list(s: &str) -> Vec<i32> {
    s.trim().chars().map(|c| c.to_digit(10).expect("Failed to parse digit") as i32).collect()
}

fn n_phases(input: &[i32], n: usize) -> Vec<i32> {
    let mut signal = input.to_vec();
    for _ in 0..n {
        signal = phase(&signal);
    }

    signal
}

fn phase(input: &[i32]) -> Vec<i32> {
    (0..input.len()).map(|n| apply_pattern(input.iter().copied(), pattern(n + 1).skip(1))).collect()
}

fn apply_pattern(input: impl IntoIterator<Item=i32>, pattern: impl IntoIterator<Item=i32>) -> i32 {
    let n = input.into_iter().zip(pattern.into_iter()).map(|(a, b)| a * b).sum();

    modulo(n, 10)
}

fn pattern(n: usize) -> impl Iterator<Item=i32> {
    iter::repeat(BASE_PATTERN[0]).take(n)
        .chain(iter::repeat(BASE_PATTERN[1]).take(n))
        .chain(iter::repeat(BASE_PATTERN[2]).take(n))
        .chain(iter::repeat(BASE_PATTERN[3]).take(n))
        .cycle()
}

fn modulo(n: i32, m: i32) -> i32 {
    let x = n % m;
    if x < 0 {
        -x
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod() {
        assert_eq!(modulo(38, 10), 8);
    }

    #[test]
    fn test_mod_negative() {
        assert_eq!(modulo(-17, 10), 7);
        assert_eq!(modulo(-34, 10), 4);
    }

    #[test]
    fn test_pattern() {
        assert_eq!(pattern(1).take(4).collect::<Vec<_>>(), [0, 1, 0, -1]);
    }

    #[test]
    fn test_apply_pattern() {
        assert_eq!(apply_pattern([9, 8, 7, 6, 5].iter().cloned(), [1, 2, 3].iter().cloned().cycle()), 2);
    }

    #[test]
    fn test_signal1() {
        let signal = phase(&signal_to_list("12345678"));
        assert_eq!(signal, signal_to_list("48226158"));

        let signal = phase(&signal);
        assert_eq!(signal, signal_to_list("34040438"));

        let signal = phase(&signal);
        assert_eq!(signal, signal_to_list("03415518"));

        let signal = phase(&signal);
        assert_eq!(signal, signal_to_list("01029498"));
    }
}
