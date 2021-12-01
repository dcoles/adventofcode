/// Advent of Code 2021: Day 1
/// https://adventofcode.com/2021/day/1

use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_input_from_file("day01/input.txt")?;

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn read_input_from_file(path: impl AsRef<Path>) -> anyhow::Result<Vec<i32>> {
    let input = fs::read_to_string(path)?;
    let values: Vec<i32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    Ok(values)
}

fn part1(input: &[i32]) -> usize {
    input.windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn part2(input: &[i32]) -> usize {
    input.windows(4)
        .filter(|w| w[1..4].iter().sum::<i32>() > w[0..3].iter().sum::<i32>())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 5);
    }
}
