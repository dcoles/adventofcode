//! Advent of Code 2021: Day 15
//! https://adventofcode.com/2021/day/15

use std::collections::HashMap;
use std::fs;
use std::io;
use std::ops::Index;
use std::path::Path;

type Pos = (i32, i32);

fn main() {
    let input = Map::from_file("day15/example1.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Map) -> i32 {
    // See https://www.redblobgames.com/pathfinding/a-star/introduction.html
    // for an excellent introduction to A*
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert((0, 0), 0);
    let mut edge: Vec<_> = [(0, (0, 0))].into_iter().collect();
    while let Some((cost, pos)) = edge.pop() {
        if pos == input.end() {
            return cost;
        }

        for adj in input.adjacent(pos) {
            let new_cost = cost_so_far[&pos] + input.risk[&adj];
            if !cost_so_far.contains_key(&adj) || new_cost < cost_so_far[&adj] {
                cost_so_far.insert(adj, new_cost);
                edge.push((new_cost, adj));
            }
        }
        edge.sort_by_key(|(p, _)| -*p);
    }

    panic!("Unable to find exit!");
}

fn part2(input: &Map) -> i32 {
    let map = input.tiled(5, 5);

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert((0, 0), 0);
    let mut edge: Vec<_> = [(0, (0, 0))].into_iter().collect();
    while let Some((cost, pos)) = edge.pop() {
        if pos == map.end() {
            return cost;
        }

        for adj in map.adjacent(pos) {
            let new_cost = cost_so_far[&pos] + map[&adj];
            if !cost_so_far.contains_key(&adj) || new_cost < cost_so_far[&adj] {
                cost_so_far.insert(adj, new_cost);
                edge.push((new_cost, adj));
            }
        }
        edge.sort_by_key(|(p, _)| -*p);
    }

    panic!("Unable to find exit!");
}

#[derive(Debug, Clone)]
struct Map {
    risk: HashMap<Pos, i32>,
    width: i32,
    height: i32,
}

impl Map {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut risk = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                risk.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
                x_max = x as i32;
            }
            y_max = y as i32;
        }

        Ok(Map { risk, width: x_max + 1, height: y_max + 1 })
    }

    /// End position
    fn end(&self) -> Pos {
        (self.width - 1, self.height - 1)
    }

    /// All adjacent positions to this position
    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        let mut adjacent = Vec::new();
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let pos = (pos.0 + dx, pos.1 + dy);
            if self.risk.contains_key(&pos) {
                adjacent.push(pos);
            }
        }

        adjacent
    }

    /// Generate tiled map
    fn tiled(&self, xtile: usize, ytile: usize) -> Self {
        let mut risk = HashMap::new();
        for y in 0..xtile as i32 {
            for x in 0..ytile as i32 {
                for (&pos, &r) in self.risk.iter() {
                    let new_pos = (x * self.width + pos.0, y * self.height + pos.1);
                    risk.insert(new_pos, ((r - 1 + x + y) % 9) + 1);
                }
            }
        }

        Map { risk, width: xtile as i32 * self.width, height: ytile as i32 * self.height }
    }

    /// Print the current map
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.risk[&(x, y)]);
            }
            println!();
        }
        println!();
    }
}

impl Index<&Pos> for Map {
    type Output = i32;

    fn index(&self, index: &Pos) -> &Self::Output {
        &self.risk[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Map::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test_part2() {
        let input = Map::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 315);
    }
}
