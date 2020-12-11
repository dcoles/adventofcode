mod map;

use std::collections::HashMap;
use crate::map::Pos;

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

const DEBUG: bool = false;

fn main() {
    let map = map::Map::from_file("input.txt");

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &map::Map) -> usize {
    let mut map = map.clone();
    if DEBUG { map.print(); }

    while tick(&mut map, 4, 1) {
        if DEBUG { map.print(); }
    }

    map.count(OCCUPIED)
}

fn part2(map: &map::Map) -> usize {
    let mut map = map.clone();
    if DEBUG { map.print(); }

    while tick(&mut map, 5, std::usize::MAX) {
        if DEBUG { map.print(); }
    }

    map.count(OCCUPIED)
}

fn tick(map: &mut map::Map, max_occupied: usize, max_scan: usize) -> bool {
    let mut changed = false;

    let mut occupied_adjacent: HashMap<map::Pos, usize> = HashMap::new();
    for y in 0..map.height() as i32 {
        for x in 0..map.width() as i32 {
            occupied_adjacent.insert((x, y), count_occupied_adjacent(map, (x, y), max_scan));
        }
    }

    for y in 0..map.height() as i32 {
        for x in 0..map.width() as i32 {
            match map.at((x, y)) {
                EMPTY => if occupied_adjacent[&(x, y)] == 0 { map.set((x, y), OCCUPIED); changed = true },
                OCCUPIED => if occupied_adjacent[&(x, y)] >= max_occupied { map.set((x, y), EMPTY); changed = true },
                FLOOR => (),
                t => panic!("Unknown tile {:?}", t),
            }
        }
    }

    changed
}

fn count_occupied_adjacent(map: &map::Map, (x, y): map::Pos, max_scan: usize) -> usize {
    let mut count = 0;

    for &(dx, dy) in &[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
        if scan(map, (x, y), (dx, dy), max_scan) == Some(OCCUPIED) {
            count += 1;
        }
    }

    count
}

fn scan(map: &map::Map, (x, y): Pos, (dx, dy): Pos, max_distance: usize) -> Option<map::Tile> {
    let mut pos = (x, y);
    for _ in 0..max_distance {
        pos = (pos.0 + dx, pos.1 + dy);

        match map.get(pos) {
            Some(tile) if tile != FLOOR => return Some(tile),
            None => return None,
            _ => (),
        }
    }

    map.get(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = map::Map::from_file("sample1.txt");
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = map::Map::from_file("sample1.txt");
        assert_eq!(part2(&input), 26);
    }

    #[test]
    fn test_part2_sample2() {
        let input = map::Map::from_file("sample2.txt");
        assert_eq!(count_occupied_adjacent(&input, (3, 4), std::usize::MAX), 8);
    }

    #[test]
    fn test_part2_sample3() {
        let input = map::Map::from_file("sample3.txt");
        assert_eq!(count_occupied_adjacent(&input, (1, 1), std::usize::MAX), 0);
    }

    #[test]
    fn test_part2_sample4() {
        let input = map::Map::from_file("sample4.txt");
        assert_eq!(count_occupied_adjacent(&input, (3, 3), std::usize::MAX), 0);
    }
}

