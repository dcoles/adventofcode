use std::path::Path;
use std::fs;
use std::collections::HashMap;

type Input = Vec<usize>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", recite(&input, 2020));
    println!("Part 2: {}", recite(&input, 30000000));
    recite(&[0, 3, 6], 2020);
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .trim()
        .split(",")
        .map(|val| val.parse().expect("Failed to parse number"))
        .collect()
}

fn recite(input: &[usize], max_n: usize) -> usize {
    // Map of numbers to the last time that number was used
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    // Insert the starting numbers
    let mut last = 0;
    for (n, &x) in input.into_iter().enumerate() {
        map.entry(x).or_default().push(n + 1);
        last = x;
    }

    // Index of previous number
    for n in input.len()..max_n {
        // Find the index of the second last time this number was used
        let &m = map.get(&last)
            .and_then(|xs| xs.iter().rev().skip(1).next())
            .unwrap_or(&n);

        let difference = n - m;

        map.entry(difference).or_default().push(n + 1);
        last = difference;
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(recite(&[0, 3, 6], 2020), 436);
    }
}

