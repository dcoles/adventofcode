use std::path::Path;
use std::fs;
use std::cmp::Ordering;

type Input = Vec<u64>;

fn main() {
    let input = read_input("input.txt");

    let n = part1(&input, 25);
    println!("Part 1: {}", n);
    println!("Part 2: {}", part2(&input, n));

}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse().expect("Failed to parse input"))
        .collect()
}

fn part1(input: &Input, preamble: usize) -> u64 {
    for i in preamble..input.len() {
        let n = input[i];

        if !is_valid(&input[i-preamble..i], n) {
            return n;
        }
    }

    panic!("No solution found!");
}

fn is_valid(preamble: &[u64], number: u64) -> bool {
    for j in 0..preamble.len() {
        for k in j+1..preamble.len() {
            if preamble[j] + preamble[k] == number {
                return true;
            }
        }
    }

    false
}

fn part2(input: &Input, target: u64) -> u64 {
    for i in 0..input.len() {
        if let Some(range) = find_contiguous_range(&input[i..], target) {
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();

            return min + max;
        }
    }

    panic!("No solution found!");
}

fn find_contiguous_range(window: &[u64], target: u64) -> Option<&[u64]> {
    let mut sum = 0;

    for i in 0..window.len() {
        sum += window[i];

        match sum.cmp(&target) {
            Ordering::Greater => break,  // Too big!
            Ordering::Less => continue,  // Not big enough!
            Ordering::Equal => return Some(&window[..i]),  // Just right!
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input, 5), 127);
    }

    #[test]
    fn test_part2() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input, 127), 62);
    }
}

