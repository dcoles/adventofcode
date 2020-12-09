use std::path::Path;
use std::fs;

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

fn part2(input: &Input, target: u64) -> u64 {
    'test: for i in 0..input.len() {
        let mut sum = 0;
        for j in i..input.len() {
            sum += input[j];
            if sum == target {
                let min = input[i..=j].iter().min().unwrap();
                let max = input[i..j].iter().max().unwrap();

                return min + max;
            }

            if sum > target {
                continue 'test;
            }
        }
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

    #[test]
    fn test_part2() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input, 127), 62);
    }
}

