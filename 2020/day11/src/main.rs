mod map;

use std::cmp::min;
use std::collections::HashMap;

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

const DEBUG: bool = false;

fn main() {
    let map = map::Map::from_file("input.txt");
    println!("Part 1: {}", part1(&map));
}

fn part1(map: &map::Map) -> usize {
    let mut map = map.clone();
    if DEBUG { map.print(); }

    while tick(&mut map) {
        if DEBUG { map.print(); }
    }

    map.count(OCCUPIED)
}

fn tick(map: &mut map::Map) -> bool {
    let mut changed = false;

    let mut occupied_adjacent: HashMap<map::Pos, usize> = HashMap::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            occupied_adjacent.insert((x, y), count_occupied_adjacent(map, (x, y)));
        }
    }

    for y in 0..map.height() {
        for x in 0..map.width() {
            match map.at((x, y)) {
                EMPTY => if occupied_adjacent[&(x, y)] == 0 { map.set((x, y), OCCUPIED); changed = true },
                OCCUPIED => if occupied_adjacent[&(x, y)] >= 4 { map.set((x, y), EMPTY); changed = true },
                FLOOR => (),
                t => panic!("Unknown tile {:?}", t),
            }
        }
    }

    changed
}

fn count_occupied_adjacent(map: &map::Map, (x, y): map::Pos) -> usize {
    let mut count = 0;

    for dy in y.saturating_sub(1)..=min(y + 1, map.height() - 1) {
        for dx in x.saturating_sub(1)..=min(x + 1, map.width() - 1) {
            if dx == x && dy == y {
                continue;
            }

            if map.at((dx, dy)) == OCCUPIED {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = map::Map::from_file("sample1.txt");
        assert_eq!(part1(&input), 37);
    }
}

