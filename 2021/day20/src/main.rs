//! Advent of Code 2021: Day 20
//! https://adventofcode.com/2021/day/20

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

type Pos = [i32; 2];

fn main() {
    let input = Input::from_file("day20/input.txt").expect("failed to read input");
    assert_eq!(input.algorithm.len(), 512);

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    enhance(input, 2)
}

fn part2(input: &Input) -> usize {
    enhance(input, 50)
}

fn enhance(input: &Input, n: usize) -> usize {
    let mut image = input.image.clone();
    let mut background = '.';
    //print_image(&image, background);
    for _ in 0..n {
        let mut new_image = image.clone();
        let min = image_min(&image);
        let max = image_max(&image);
        for y in min[1]..=max[1] {
            for x in min[0]..=max[0] {
                let n = kernel(&image, background, [x, y]);
                new_image.insert([x, y], input.algorithm[n]);
            }
        }
        image = new_image;
        background = if background == '.' { input.algorithm[0] } else { *input.algorithm.last().unwrap() };
        //print_image(&image, background);
    }

    assert_eq!(background, '.');
    image.values().filter(|&&c| c == '#').count()
}

fn image_min(image: &HashMap<Pos, char>) -> [i32; 2] {
    let min_x = image.keys().map(|&[x, _]| x).min().unwrap();
    let min_y = image.keys().map(|&[_, y]| y).min().unwrap();

    [min_x - 1, min_y - 1]
}

fn image_max(image: &HashMap<Pos, char>) -> [i32; 2] {
    let max_x = image.keys().map(|&[x, _]| x).max().unwrap();
    let max_y = image.keys().map(|&[_, y]| y).max().unwrap();

    [max_x + 1, max_y + 1]
}

fn kernel(image: &HashMap<Pos, char>, background: char, center: Pos) -> usize {
    let mut n = 0;
    for y in center[1]-1..=center[1]+1 {
        for x in center[0]-1..=center[0]+1 {
            n <<= 1;
            if *image.get(&[x, y]).unwrap_or(&background) == '#' {
                n += 1;
            }
        }
    }

    n
}

fn print_image(image: &HashMap<Pos, char>, background: char) {
    let min = image_min(&image);
    let max = image_max(&image);
    for y in min[1]..=10 {//max[1] {
        for x in min[0]..=10 {//max[0] {
            let &c = image.get(&[x, y]).unwrap_or(&background);
            print!("{}", c);
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone)]
struct Input {
    algorithm: Vec<char>,
    image: HashMap<Pos, char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let (chunk1, chunk2) = input.split_once("\n\n").unwrap();

        let algorithm = chunk1.lines().flat_map(|line| line.chars()).collect();

        let mut image = HashMap::new();
        for (y, line) in chunk2.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                image.insert([x as i32, y as i32], c);
            }
        }


        Ok(Input { algorithm, image })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 3351);
    }
}
