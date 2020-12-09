use std::path::Path;
use std::fs;

type Input = Vec<u64>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input, 25));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse().expect("Failed to parse input"))
        .collect()
}

fn part1(input: &Input, preamble: usize) -> u64 {
    'test: for i in preamble..input.len() {
        let n = input[i];
        for j in i-preamble..i {
            for k in j+1..i {
                if input[j] + input[k] == n {
                    continue 'test
                }
            }
        }
        return n;
    }

    panic!("No solution found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input, 5), 127);
    }
}

