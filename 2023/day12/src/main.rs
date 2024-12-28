//! Advent of Code 2023: Day 12
//! https://adventofcode.com/2023/day/12

use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut combinations = vec![];
    for record in &input.values {
        let unknown_index: Vec<usize> = (0..record.0.len()).filter(|&i| matches!(record.0[i], State::Unknown)).collect();
        let n_combinations: usize = 2 << (unknown_index.len() - 1);
        let mut state = record.0.clone();

        let mut count = 0;
        for n in 0..n_combinations {
            for (i, &x) in unknown_index.iter().enumerate() {
                state[x] = if n >> i & 1 == 1 { State::Damaged } else { State::Operational };
            }

            if is_match(&state, &record.1) {
                count += 1;
            }
        }

        combinations.push(count);
    }

    //println!("{:?}", combinations);
    combinations.into_iter().sum()
}

fn is_match(state: &[State], groups: &[usize]) -> bool {
    let mut index = 0;
    let mut run = 0;

    for (i, s) in state.iter().enumerate() {
        let damaged = matches!(s, State::Damaged);
        if damaged {
            run += 1;
        }

        let last = i == (state.len() - 1);
        if (!damaged || last) && run > 0 {
            if index >= groups.len() {
                return false;
            }

            if groups[index] != run {
                return false;
            }

            index += 1;
            run = 0;
        }
    }

    index == groups.len()
}

fn part2(input: &Input) -> usize {
    for (records, groups) in &input.values {
        let mut records = records.clone();
        println!("{records:?} {groups:?}");

        let total = records.len();
        let n_damaged: usize = groups.iter().sum();
        let n_ok = total - n_damaged;

        println!("total: {total}, damaged: {n_damaged}, ok: {n_ok}");

        // We can split these up into groups of ok/broken/ok/.../broken/ok
        // N0 + A + (1 + N1) + B + (1 + N2) + ... + Z + Nn = total
        // We know that N0 .. Nn must be in the range 0..a
        let mut a = n_ok - (groups.len() - 1);
        println!("a: {a}");

        // There's always at least one OK spring between broken sets
        let mut ns = vec![[0, a]; groups.len() + 1];
        for n in 1..groups.len() {
            ns[n][0] += 1;
            ns[n][1] += 1;
        }

        let mut ok_known = records.iter().filter(|r| matches!(r, State::Operational)).count();
        let mut damaged_known = records.iter().filter(|r| matches!(r, State::Damaged)).count();
        let mut unknown = records.iter().filter(|r| matches!(r, State::Unknown)).count();
        assert_eq!(ok_known + damaged_known + unknown, records.len());

        let mut n = 0;
        let mut ok_seen = 0;
        let mut damaged_seen = 0;
        let mut unknown_seen = 0;
        for i in 0..records.len() {
            println!("{i:02}: OK={ok_seen:2}, NG={damaged_seen:2}, UNK={unknown_seen:2} {ns:?}");
            match records[i] {
                State::Damaged => {
                    damaged_seen += 1;

                    ns[n][1] = ns[n][1].min(i); // This can probably be more agressive

                    if damaged_seen == groups[0] {
                        if (i + 1) > groups[n] {
                            // This set must be preceeded by an undamaged mirror
                            assert!(!matches!(records[i - groups[n]], State::Damaged));

                            if matches!(records[i - groups[n]], State::Unknown) {
                                records[i - groups[n]] = State::Operational;
                                ok_known += 1;
                                unknown -= 1;
                            }

                            ns[n][0] += 1;
                            a = a.saturating_sub(1);
                            for m in (0..ns.len()).filter(|&m| m != n) {
                                ns[m][1] = ns[m][1].min(a);
                            }
                        }

                        if (i + 1) < records.len() {
                            // This set must be followed by an undamaged mirror
                            assert!(!matches!(records[i + 1], State::Damaged));

                            if matches!(records[i + 1], State::Unknown) {
                                records[i + 1] = State::Operational;
                                ok_known += 1;
                                unknown -= 1;
                            }

                            assert_eq!(ns[n + 1][0], 1);  // already accounted for there being at least 1
                        }

                        ok_seen = 0;
                        damaged_seen = 0;
                        unknown_seen = 0;

                        n += 1;
                    }
                },
                State::Operational => {
                    ok_seen += 1;
                },
                State::Unknown => {
                    unknown_seen += 1;
                },
            }

            // We have to always maintain these invariants
            assert!(ok_seen <= n_ok);
            assert!(damaged_seen <= n_damaged);
            assert_eq!(ok_known + damaged_known + unknown, records.len());
            assert!(ns.iter().map(|[lower, _]| lower).sum::<usize>() <= n_ok, "violated lower-bounds: {ns:?}");
            assert!(ns.iter().map(|[_, upper]| upper).sum::<usize>() >= n_ok, "violated upper-bounds: {ns:?}");
            assert!(ns.iter().all(|[lower, upper]| lower <= upper));
        }

        // We've done all we can
        println!();
    }

    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Unknown,
    Operational,
    Damaged,
}

impl State {
    fn from_char(c: char) -> Self {
        match c {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => panic!("Unknown state: {c:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(Vec<State>, Vec<usize>)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let a = a.chars().map(State::from_char).collect();
            let b = b.split(',').map(|s| s.parse().unwrap()).collect();

            (a, b)
        }).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 7025);
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
