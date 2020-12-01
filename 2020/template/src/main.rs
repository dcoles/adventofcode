use std::path::Path;
use std::fs;

fn main() {
    let input = read_input("input.txt");
    println!("> {:?}", input);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<i32> {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| line.parse::<i32>().expect("Failed to parse line"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = read_input("input.txt");
        assert_eq!(input[0], 1);
    }

    #[test]
    fn test_part2_example1() {
        let input = read_input("input.txt");
        assert_eq!(input[1], 2);
    }
}

