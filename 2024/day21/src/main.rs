//! Advent of Code 2024: Day 21
//! https://adventofcode.com/2024/day/21

use std::{fs, io};
use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::btree_map::Entry;
use std::iter::{IntoIterator, Iterator};
use std::path::Path;
use lazy_static::lazy_static;
use lib::vector::Vector;

type Vec2 = Vector<i32, 2>;

fn main() {
    //let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

const UP: Vec2 = Vec2::new([0, -1]);
const DOWN: Vec2 = Vec2::new([0, 1]);
const LEFT: Vec2 = Vec2::new([-1, 0]);
const RIGHT: Vec2 = Vec2::new([1, 0]);

lazy_static! {
    static ref NUMPAD: BTreeMap<Vec2, char> = [
        ([-2, -3], '7'), ([-1, -3], '8'), ([ 0, -3], '9'),
        ([-2, -2], '4'), ([-1, -2], '5'), ([ 0, -2], '6'),
        ([-2, -1], '1'), ([-1, -1], '2'), ([ 0, -1], '3'),
        /* -----------*/ ([-1,  0], '0'), ([ 0,  0], 'A'),
    ].into_iter().map(|(p, c)| (Vec2::new(p), c)).collect();
    static ref NUMS: BTreeMap<char, Vec2> = NUMPAD.iter().map(|(x, y)| (*y, *x)).collect();

    static ref DPAD: BTreeMap<Vec2, char> = [
        /* ---------- */ ([-1,  0], '^'), ([ 0,  0], 'A'),
        ([-2,  1], '<'), ([-1,  1], 'v'), ([ 0,  1], '>'),
    ].into_iter().map(|(p, c)| (Vec2::new(p), c)).collect();
    static ref DIRS: BTreeMap<char, Vec2> = DPAD.iter().map(|(x, y)| (*y, *x)).collect();
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State<const N: usize> {
    robots: [Vec2; N],
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        State {
            robots: [Vec2::default(); N],
        }
    }
}

impl<const N: usize> State<N> {
    fn activate(mut self) -> Result<(Self, Option<char>), ()> {
        let mut key = None;

        for n in 0..self.robots.len() {
            let pos = self.robots[n];

            if n < (N - 1) {
                let button = DPAD[&pos];
                let mut activate = false;
                let mut next_pos = self.robots[n + 1];

                match button {
                    '^' => next_pos += UP,
                    'v' => next_pos += DOWN,
                    '<' => next_pos += LEFT,
                    '>' => next_pos += RIGHT,
                    'A' => activate = true,
                    x => panic!("unexpected button: {x}"),
                }

                if n + 1 < (N - 1) {
                    if !DPAD.contains_key(&next_pos) {
                        return Err(());
                    }
                } else {
                    if !NUMPAD.contains_key(&next_pos) {
                        return Err(());
                    }
                }

                self.robots[n + 1] = next_pos;

                if !activate {
                    break;
                }
            } else {
                key = Some(NUMPAD[&pos]);
            }
        }

        Ok((self, key))
    }

    fn up(mut self) -> Result<Self, ()> {
        self.robots[0] += UP;

        if !DPAD.contains_key(&self.robots[0]) {
            return Err(())
        }

        Ok(self)
    }

    fn down(mut self) -> Result<Self, ()> {
        self.robots[0] += DOWN;

        if !DPAD.contains_key(&self.robots[0]) {
            return Err(())
        }

        Ok(self)
    }

    fn left(mut self) -> Result<Self, ()> {
        self.robots[0] += LEFT;

        if !DPAD.contains_key(&self.robots[0]) {
            return Err(())
        }

        Ok(self)
    }

    fn right(mut self) -> Result<Self, ()> {
        self.robots[0] += RIGHT;

        if !DPAD.contains_key(&self.robots[0]) {
            return Err(())
        }

        Ok(self)
    }
}

fn numeric_part(s: impl Iterator<Item=char>) -> usize {
    s.take_while(|c| c.is_numeric())
        .fold(0, |acc, c| 10 * acc + c.to_digit(10).unwrap() as usize)
}

fn part1(input: &Input) -> usize {
    let mut sum = 0;

    for code in input.values.iter() {
        let start = (0, State::<3>::default());
        let mut best: BTreeMap<_, usize> = [(start, 0)].into_iter().collect();
        let mut edge = vec![start];

        while let Some((n, state)) = edge.pop() {
            let cur_moves = best[&(n, state)];

            if n == code.len() {
                sum += best[&(n, state)] * numeric_part(code.iter().copied());
                break;
            }

            for new_state in [state.up(), state.down(), state.left(), state.right()] {
                if let Ok(new_state) = new_state {
                    if !best.contains_key(&(n, new_state)) || (cur_moves + 1) < best[&(n, new_state)] {
                        edge.push((n, new_state));
                        best.insert((n, new_state), cur_moves + 1);
                    }
                }
            }

            if let Ok((new_state, key)) = state.activate() {
                if let Some(key) = key {
                    if code[n] == key {
                        if !best.contains_key(&(n + 1, new_state)) || (cur_moves + 1) < best[&(n + 1, new_state)] {
                            edge.push((n + 1, new_state));
                            best.insert((n + 1, new_state), cur_moves + 1);
                        }
                    }
                } else {
                    if !best.contains_key(&(n, new_state)) || (cur_moves + 1) < best[&(n, new_state)] {
                        edge.push((n, new_state));
                        best.insert((n, new_state), cur_moves + 1);
                    }
                }
            }

            edge.sort_by_key(|&(n, s)| usize::MAX - (best[&(n, s)] + (code.len() - n)));
        }
    }

    sum
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;

    //let shortest_nums = find_shortest_pairs(&*NUMS, false);
    //let shortest_dirs = find_shortest_pairs(&*DIRS, true);

    //println!("{shortest_nums:?}");


    for code in input.values.iter() {
        println!("{code:?}");
        let start = Vec2::default();
        let mut visited: BTreeSet<(Vec2, usize, String)> = [(start, 0, String::new())].into_iter().collect();
        let mut edge = vec![(start, 0, String::new())];
        while let Some((pos, n, path)) = edge.pop() {
            if n == code.len() {
                println!("- {path}");
                continue;
            }

            let cur_num = NUMPAD[&pos];
            if cur_num == code[n] {
                let mut new_path = path.clone();
                new_path.push('A');
                edge.push((pos, n + 1, new_path));
                continue;
            }

            for (dir, c) in [(UP, "^"), (DOWN, "v"), (LEFT, "<"), (RIGHT, ">")] {
                let new_pos = pos + dir;
                let state = (new_pos, n, path.clone() + c);
                if !NUMPAD.contains_key(&new_pos) || visited.contains(&state) {
                    continue;
                }
                visited.insert(state.clone());
                edge.push(state);
            }
        }

        edge.sort_by_key(|(_, _, path)| usize::MAX - path.len())
    }

    /*
    for code in input.values.iter().map(|c| c.iter().collect::<String>()) {
        println!("{code}");

        let mut keys = vec![];
        let mut prev = 'A';
        for c in code.chars() {
            keys.push(shortest_nums[&(prev, c)].clone() + "A");
            prev = c;
        }


        for _ in 0..2 {
            let mut new_keys = vec![];
            for code2 in keys {
                let mut prev = 'A';
                for c in code2.chars() {
                    new_keys.push(shortest_dirs[&(prev, c)].clone() + "A");
                    prev = c;
                }
            }

            keys = new_keys;
        }

        let length: usize = keys.iter().map(|s| s.len()).sum();
        println!("{keys:?}: {length}");

        sum += length * numeric_part(code.chars());
    }
     */

    sum
}

fn find_shortest_pairs(key_map: &BTreeMap<char, Vec2>, order: bool) -> BTreeMap<(char, char), Vec<String>> {
    let mut shortest: BTreeMap<(char, char), Vec<String>> = BTreeMap::new();
    let key_lookup: BTreeMap<Vec2, char> = key_map.iter().map(|(k, p)| (*p, *k)).collect();

    for (start_key, start) in key_map.iter().map(|(k, p)| (*k, *p)) {
        let mut visited: BTreeSet<Vec2> = [start].into_iter().collect();

        let mut edge = vec![(start, String::new())];
        while let Some((pos, path)) = edge.pop() {
            let cur_key = key_lookup[&pos];
            match shortest.entry((start_key, cur_key)) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![path.clone()]);
                }
                Entry::Occupied(mut entry) => {
                    let paths = entry.get_mut();
                    if path.len() > paths[0].len() {
                        break;
                    }

                    paths.push(path.clone());
                }
            }

            let dirs = [(LEFT, "<"), (DOWN, "v"), (RIGHT, ">"), (UP, "^")];
            for (dir, c) in dirs {
                let new_pos = pos + dir;
                if !key_lookup.contains_key(&new_pos) || visited.contains(&new_pos) {
                    continue;
                }

                //visited.insert(new_pos);
                let new_path = path.clone() + c;

                edge.push((new_pos, new_path));
            }

            edge.sort_by_key(|(_, path)| usize::MAX - (100 * path.len()))
        }
    }

    shortest
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Vec<char>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let values = input.lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 126384);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 128962);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 0);
    }
}
