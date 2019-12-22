use std::{fs, iter};
use std::path::Path;

const PATTERN: [i32; 4] = [0, 1, 0, -1];

fn main() {
    let (signal, offset) = read_input("input.txt");

    // Part 1
    let mut signal1 = signal.clone();
    for _ in 0..100 {
        signal1 = phase(&signal1);
    }
    println!("Part 1: After 100 phases, the first 8 digits are: {:?}", &signal1[..8]);

    // Part 2
    let real_signal: Vec<i32> = signal.iter().copied().cycle().take(10_000 * signal.len()).collect();
    println!("Part 2: The first 8 digits of the final output list are: {:?}", &fft(&real_signal, offset, 100)[..8]);
}

fn fft(input: &[i32], offset: usize, rounds: usize) -> Vec<i32> {
    let remainder = input.len() - offset;

    // This only works if all elements of the pattern at or after offset are 1
    assert!(offset >= remainder);

    let mut result: Vec<_> = input.iter().copied().skip(offset).collect();
    for _ in 0..rounds {
        // As each new element is the sum of current and following elements of the
        // previous round, we can construct the result from the right.
        // e.g.
        // n0: [..., 5, 6, 7, 8]
        // n1: [..., 5 + (6 + 7 + 8), 6 + (7 + 8), 7 + (8), 8] mod 10
        //     [..., 6, 1, 7, 8]
        for i in (0..result.len() - 1).rev() {
            result[i] = (result[i] + result[i + 1]) % 10;
        }
    }

    result
}

fn read_input<T: AsRef<Path>>(path: T) -> (Vec<i32>, usize) {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    let offset = contents[..7].parse().expect("Failed to parse message offset");

    (digits(&contents), offset)
}

/// Convert a list of digits to a vector of integers.
fn digits(s: &str) -> Vec<i32> {
    s.trim().chars().map(|c| c.to_digit(10).expect("Failed to parse digit") as i32).collect()
}

/// Calculate a single phase of the FFT.
fn phase(input: &[i32]) -> Vec<i32> {
    (0..input.len()).map(|n| apply_pattern(input.iter().copied(), pattern(n + 1).skip(1))).collect()
}

/// Apply a pattern to an input list and return the resulting value.
fn apply_pattern(input: impl IntoIterator<Item=i32>, pattern: impl IntoIterator<Item=i32>) -> i32 {
    let n: i32 = input.into_iter().zip(pattern.into_iter()).map(|(a, b)| a * b).sum();

    (n % 10).abs()
}

/// Generate a pattern with each element repeated `n`-times.
/// e.g. If `n` = 2, `[0, 0, 1, 1, 0, 0, -1, -1]`
fn pattern(n: usize) -> impl Iterator<Item=i32> {
    iter::repeat(PATTERN[0]).take(n)
        .chain(iter::repeat(PATTERN[1]).take(n))
        .chain(iter::repeat(PATTERN[2]).take(n))
        .chain(iter::repeat(PATTERN[3]).take(n))
        .cycle()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let signal = phase(&digits("12345678"));
        assert_eq!(signal, digits("48226158"));

        let signal = phase(&signal);
        assert_eq!(signal, digits("34040438"));

        let signal = phase(&signal);
        assert_eq!(signal, digits("03415518"));

        let signal = phase(&signal);
        assert_eq!(signal, digits("01029498"));
    }
}
