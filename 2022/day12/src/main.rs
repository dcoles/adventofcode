//! Advent of Code 2022: Day 12
//! https://adventofcode.com/2022/day/12

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut best: HashMap<(usize, usize), usize> = [(input.start, 0)].into_iter().collect();
    let mut edge = vec![input.start];

    while let Some(pos) = edge.pop() {
        for neighbour in input.neighbour(pos) {
            let steps = best[&pos] + 1;
            if best.get(&neighbour).map(|best| steps < *best).unwrap_or(true) {
                best.insert(neighbour, steps);
                edge.push(neighbour);
            }
        }
    }

    // What is the fewest steps required to move from your current position
    // to the location that should get the best signal?
    best[&input.end]
}

fn part2(input: &Input) -> usize {
    // All locations with elevation 'a'
    let starting_locations: Vec<_> = input.map.iter().filter(|(_, &h)| h == 0).map(|(&p, _)| p).collect();

    let mut best: HashMap<(usize, usize), usize> = starting_locations.iter().cloned().map(|p| (p, 0)).collect();
    let mut edge = starting_locations.clone();

    while let Some(pos) = edge.pop() {
        for neighbour in input.neighbour(pos) {
            let steps = best[&pos] + 1;
            if best.get(&neighbour).map(|best| steps < *best).unwrap_or(true) {
                best.insert(neighbour, steps);
                edge.push(neighbour);
            }
        }
    }

    // What is the fewest steps required to move starting from any square
    // with elevation a to the location that should get the best signal?
    best[&input.end]
}

#[derive(Debug, Clone)]
struct Input {
    map: HashMap<(usize, usize), u32>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut map = HashMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let elevation = match c {
                    'S' => {
                        start = (x, y);

                        0 // 'a'
                    },
                    'E' => {
                        end = (x, y);

                        'z' as u32 - 'a' as u32
                    }
                    _ => {
                        c as u32 - 'a' as u32
                    },
                };

                map.insert((x, y), elevation);
                height = height.max(y + 1);
                width = width.max(x + 1);
            }
        }

        Ok(Input { map, start, end, width, height })
    }

    fn neighbour(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let cur_height = self.map[&pos];

        let mut neighbours = Vec::new();
        for y in pos.1.saturating_sub(1)..(pos.1 + 2).min(self.height) {
            for x in pos.0.saturating_sub(1)..(pos.0 + 2).min(self.width) {

                if (x == pos.0 && y != pos.1) || (y == pos.1 && x != pos.0) {
                    let height = self.map[&(x, y)];
                    if height <= cur_height + 1 {
                        neighbours.push((x, y));
                    }
                }
            }
        }

        neighbours
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 29);
    }
}
