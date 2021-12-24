//! Advent of Code 2021: Day 23
//! https://adventofcode.com/2021/day/23

use std::collections::HashMap;
use std::fs;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;

const EMPTY: char = '.';
const WALL: char = '#';
const AMBER: char = 'A';
const BRONZE: char = 'B';
const COPPER: char = 'C';
const DESERT: char = 'D';

const WIDTH: usize = 13;
const HEIGHT: usize = 7;

type Pos = [usize; 2];

fn main() {
    let input1 = State::from_file("day23/input1.txt").expect("failed to read input");
    let input2 = State::from_file("day23/input2.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", solve(&input1).expect("no solution"));

    // Part 2
    println!("Part 2: {}", solve(&input2).expect("no solution"));
}

fn solve(input: &State) -> Option<usize> {
    let mut edge = vec![(0, input.clone())];
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(input.clone(), 0);

    while let Some((_, state)) = edge.pop() {
        //println!("Cost: {}", cost_so_far[&state]);
        //state.print();

        if state.is_solved() {
            return Some(cost_so_far[&state]);
        }

        for (pos, c) in state.iter() {
            if state.is_final(pos, c) {
                // Don't move if we're already in the corect position.
                continue;
            }

            for (steps, new_pos) in state.can_move(pos) {
                if state.is_illegal(new_pos) {
                    // Amphipods will never stop on the space immediately outside any room.
                    continue;
                }

                if state.is_hallway(pos) {
                    if state.is_hallway(new_pos) {
                        // Once an amphipod stops moving in the hallway,
                        // it will stay in that spot until it can move into a room.
                        continue;
                    }

                    if !state.is_final(new_pos, c) {
                        // Amphipods will never move from the hallway into a room
                        // unless that room is their destination room and
                        // that room contains no amphipods which do not
                        // also have that room as their own destination.
                        continue;
                    }
                } else {
                    if !state.is_hallway(new_pos) {
                        // Once an amphipod stops moving in the hallway,
                        // it will stay in that spot until it can move into a room.
                        continue;
                    }
                }

                let new_cost = cost_so_far[&state] + steps * energy(c);
                let mut new_state = state.clone();
                new_state[pos] = EMPTY;
                new_state[new_pos] = c;

                if cost_so_far.contains_key(&new_state) && new_cost >= cost_so_far[&new_state] {
                    // This state is strictly worse than one we've already seen
                    continue;
                }

                cost_so_far.insert(new_state.clone(), new_cost);
                let h = (new_cost + state.heuristic()) as i64;
                edge.push((h, new_state));
            }
        }
        edge.sort_by_key(|&(h, _)| -h);
    }

    None
}

const fn energy(c: char) -> usize {
    match c {
        AMBER => 1,
        BRONZE => 10,
        COPPER => 100,
        DESERT => 1000,
        _ => 0,
    }
}

const fn expected_column(c: char) -> usize {
    match c {
        AMBER => 3,
        BRONZE => 5,
        COPPER => 7,
        DESERT => 9,
        _ => 0,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    tiles: [[char; WIDTH]; HEIGHT],
}

impl State {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut tiles = [[' '; WIDTH]; HEIGHT];
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                tiles[y][x] = c;
            }
        }

        Ok(State { tiles })
    }

    fn is_solved(&self) -> bool {
        for c in [AMBER, BRONZE, COPPER, DESERT] {
            let x = expected_column(c);
            for y in 2..HEIGHT {
                if self[[x, y]] == WALL {
                    break;
                }

                if self[[x, y]] != c {
                    return false;
                }
            }
        }

        true
    }

    fn is_hallway(&self, pos: Pos) -> bool {
        pos[1] == 1
    }

    fn is_illegal(&self, pos: Pos) -> bool {
        [[3, 1], [5, 1], [7, 1], [9, 1]].contains(&pos)
    }

    fn is_final(&self, pos: Pos, c: char) -> bool {
        let col = expected_column(c);
        if pos[0] != col {
            return false;
        }

        assert!(pos[1] >= 2);
        for y in pos[1]+1..HEIGHT {
            if self[[col, y]] == WALL {
                break;
            }

            if self[[col, y]] != c {
                return false;
            }
        }

        true
    }

    fn iter(&self) -> impl '_ + Iterator<Item=(Pos, char)> {
        (0..WIDTH*HEIGHT).into_iter().filter_map(|i| {
            let pos = [i % WIDTH, i / WIDTH];
            if matches!(self[pos], AMBER | BRONZE | COPPER | DESERT) {
                Some((pos, self[pos]))
            } else {
                None
            }
        })
    }

    fn can_move(&self, from: Pos) -> Vec<(usize, Pos)> {
        let mut edge = vec![from];
        let mut steps = HashMap::new();
        while let Some(pos) = edge.pop() {
            for adj in self.adjacent(pos) {
                if steps.contains_key(&adj) {
                    continue;
                }

                edge.push(adj);
                steps.insert(adj, steps.get(&pos).copied().unwrap_or(0) + 1);
            }
        }

        let mut steps: Vec<_> = steps.into_iter().map(|(pos, d)| (d, pos)).collect();
        steps.sort_by_key(|&(d, _)| d);

        steps
    }

    fn adjacent(&self, pos: Pos) -> impl '_ + Iterator<Item=Pos> {
        [[pos[0] + 1, pos[1]], [pos[0], pos[1] + 1], [pos[0] - 1, pos[1]], [pos[0], pos[1] - 1]]
            .into_iter().filter(|&[x, y]| {
                self.tiles[y][x] == EMPTY
            })
    }

    fn heuristic(&self) -> usize {
        let mut fill: HashMap<char, usize> = HashMap::new();
        let mut h = 0;
        for (pos, c) in self.iter() {
            if self.is_final(pos, c) {
                continue;
            }

            *fill.entry(c).or_default() += 1;

            // Must not over-estimate or else this will not find the optimal solution!
            let col = expected_column(c);
            let dx = (pos[0] as i32 - col as i32).abs() as usize;
            let dy = (pos[1] - 1) + fill[&c];
            let energy = (dx + dy) * energy(c);

            h += energy;
        }

        h
    }

    fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", self.tiles[y][x]);
            }
            println!();
        }
        println!();
    }

}

impl Index<Pos> for State {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.tiles[index[1]][index[0]]
    }
}

impl IndexMut<Pos> for State {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.tiles[index[1]][index[0]]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = State::from_file("example1.txt").expect("failed to read input");

        assert_eq!(solve(&input).expect("no solution"), 12521);
    }

    #[test]
    fn test_part2() {
        let input = State::from_file("example2.txt").expect("failed to read input");

        assert_eq!(solve(&input).expect("no solution"), 44169);
    }
}
