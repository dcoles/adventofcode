//! Advent of Code 2024: Day 20
//! https://adventofcode.com/2024/day/20

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::str::FromStr;
use lib::grid::{Grid, Pos};

fn main() {
    //let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let cheats = find_cheats(&input.map, 2);

    cheats.into_iter()
        .filter_map(|(delta, cheats)| (delta >= 100).then_some(cheats.len()))
        .sum()
}

fn find_cheats(map: &Grid, max_cheat: usize) -> BTreeMap<usize, BTreeSet<(Pos, Pos)>> {
    let (track, distance) = find_track(map);

    let mut cheats: BTreeMap<usize, BTreeSet<(Pos, Pos)>> = BTreeMap::default();

    for n in 0..track.len() {
        for m in (n + 1)..track.len() {
            let start = track[n];
            let end = track[m];

            let cheat_distance = (end[0] - start[0]).abs() as usize + (end[1] - start[1]).abs() as usize;
            if cheat_distance > max_cheat {
                continue;
            }

            if let Some(delta) = distance[&end].checked_sub(distance[&start] + cheat_distance) {
                if delta > 0 {
                    cheats.entry(delta).or_default().insert((start, end));
                }
            }
        }
    }

    cheats
}

fn find_track(map: &Grid) -> (Vec<Pos>, BTreeMap<Pos, usize>) {
    let start = map.positions().find(|&p| map[p] == 'S').unwrap();
    let end = map.positions().find(|&p| map[p] == 'E').unwrap();

    let mut track = vec![start];
    let mut distance: BTreeMap<Pos, usize> = [(start, 0)].into_iter().collect();

    let mut pos = start;
    loop {
        let cur_distance = distance[&pos];
        pos = map.adjacent(pos).into_iter().find(|&p| map[p] != '#' && !distance.contains_key(&p)).unwrap();

        track.push(pos);
        distance.insert(pos, cur_distance + 1);

        if pos == end {
            break;
        }
    }

    (track, distance)
}

fn part2(input: &Input) -> usize {
    let cheats = find_cheats(&input.map, 20);

    cheats.into_iter()
        .filter_map(|(delta, cheats)| (delta >= 100).then_some(cheats.len()))
        .sum()
}

#[derive(Debug, Clone)]
struct Input {
    map: Grid,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let map = Grid::from_str(&input)?;

        Ok(Self { map })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();
        let cheats = find_cheats(&input.map, 2);

        assert_eq!(cheats[&2].len(), 14);
        assert_eq!(cheats[&4].len(), 14);
        assert_eq!(cheats[&6].len(), 2);
        assert_eq!(cheats[&8].len(), 4);
        assert_eq!(cheats[&10].len(), 2);
        assert_eq!(cheats[&12].len(), 3);
        assert_eq!(cheats[&20].len(), 1);
        assert_eq!(cheats[&36].len(), 1);
        assert_eq!(cheats[&38].len(), 1);
        assert_eq!(cheats[&40].len(), 1);
        assert_eq!(cheats[&64].len(), 1);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1426);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();
        let cheats = find_cheats(&input.map, 20);

        assert_eq!(cheats[&50].len(), 32);
        assert_eq!(cheats[&52].len(), 31);
        assert_eq!(cheats[&54].len(), 29);
        assert_eq!(cheats[&56].len(), 39);
        assert_eq!(cheats[&58].len(), 25);
        assert_eq!(cheats[&60].len(), 23);
        assert_eq!(cheats[&62].len(), 20);
        assert_eq!(cheats[&64].len(), 19);
        assert_eq!(cheats[&66].len(), 12);
        assert_eq!(cheats[&68].len(), 14);
        assert_eq!(cheats[&70].len(), 12);
        assert_eq!(cheats[&72].len(), 22);
        assert_eq!(cheats[&74].len(), 4);
        assert_eq!(cheats[&76].len(), 3);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 1000697);
    }
}
