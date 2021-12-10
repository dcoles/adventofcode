//! Advent of Code 2021: Day 10
//! https://adventofcode.com/2021/day/10

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = read_input_from_file("day10/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[String]) -> u64 {
    let mut score = 0;

    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let opening = stack.pop().unwrap();
                    if closing(opening) != c {
                        score += syntax_points(c);
                        break
                    }
                },
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    score
}

fn part2(input: &[String]) -> u64 {
    let mut scores = Vec::new();

    for line in input {
        let mut stack = Vec::new();
        let mut valid_line = true;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let opening = stack.pop().unwrap();
                    if closing(opening) != c {
                        valid_line = false;
                        break
                    }
                },
                _ => panic!("Unexpected character: {}", c),
            }
        }

        if valid_line {
            let mut score = 0;
            while let Some(c) = stack.pop() {
                score = score * 5 + autocomplete_points(closing(c));
            }

            scores.push(score);
        }
    }

    scores.sort();

    scores[scores.len() / 2]
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown character: {}", c),
    }
}

fn syntax_points(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown character: {}", c),
    }
}

fn autocomplete_points(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unknown character: {}", c),
    }
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = fs::read_to_string(path)?;

    Ok(input.lines().map(str::to_string).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 288957);
    }
}
