use std::fs;
use std::path::Path;
use std::collections::{HashSet, BTreeMap};

type Pos = (usize, usize);
type Map = BTreeMap<Pos, char>;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const BUGS: char = '#';
const EMPTY: char = '.';

fn main() {
    let map = read_input("input.txt");
    draw(&map);

    // Part 1
    let first_repeat = simulate(&map);
    println!("Part 1: Biodiversity rating: {}", biodiversity_rating(&first_repeat));
}

fn simulate(map: &Map) -> Map {
    let mut map = map.clone();
    let mut seen = HashSet::new();

    while !seen.contains(&map) {
        seen.insert(map.clone());
        let mut new_map = map.clone();
        for (&pos, &tile) in map.iter() {
            let adjacent_bugs = n_adjacent_bugs(pos, &map);
            if tile == BUGS && adjacent_bugs != 1 {
                // A bug dies (becoming an empty space) unless
                // there is exactly one bug adjacent to it.
                new_map.insert(pos, EMPTY);
            } else if (1..=2).contains(&adjacent_bugs) {
                // An empty space becomes infested with a bug if
                // exactly one or two bugs are adjacent to it.
                new_map.insert(pos, BUGS);
            }
        }
        map = new_map;
    }

    map
}

fn biodiversity_rating(map: &Map) -> u32 {
    let mut rating = 0;
    let mut p = 1;
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            if map[&(x, y)] == BUGS {
                rating += p;
            }
            p <<= 1;
        }
    }

    rating
}

fn draw(map: &Map) {
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            let c = map[&(x, y)];
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn read_input<T: AsRef<Path>>(path: T) -> Map {
    let contents = fs::read_to_string(path).expect("Failed to read input");

    let mut map = BTreeMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c);
        }
    }

    map
}

fn n_adjacent_bugs((x, y): Pos, map: &Map) -> usize {
    let mut n = 0;

    if x > 0 && *map.get(&(x - 1, y)).unwrap_or(&EMPTY) == BUGS { n += 1 };
    if *map.get(&(x + 1, y)).unwrap_or(&EMPTY) == BUGS { n += 1 };
    if y > 0 && *map.get(&(x, y - 1)).unwrap_or(&EMPTY) == BUGS { n += 1 };
    if *map.get(&(x, y + 1)).unwrap_or(&EMPTY) == BUGS { n += 1 };

    n
}
