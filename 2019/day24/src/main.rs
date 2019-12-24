use std::fs;
use std::path::Path;
use std::collections::{HashSet, BTreeSet};

type Pos = (i32, usize, usize);

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const LEVEL_ZERO: i32 = 0;
const BUG: char = '#';
const EMPTY: char = '.';

fn main() {
    let map = Map::from_file("input.txt");
    map.draw();

    // Part 1
    let first_repeat = simulate(&map);
    first_repeat.draw();
    println!("Part 1: Biodiversity rating: {}", biodiversity_rating(&first_repeat));
}

fn simulate(map: &Map) -> Map {
    let mut map = map.clone();
    let mut seen = HashSet::new();

    while !seen.contains(&map.bugs) {
        seen.insert(map.bugs.clone());
        let mut new_map = map.clone();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = (0, x, y);
                let adjacent_bugs = map.number_of_adjacent_bugs(pos);
                if map.is_bug_at(pos) && adjacent_bugs != 1 {
                    // A bug dies (becoming an empty space) unless
                    // there is exactly one bug adjacent to it.
                    new_map.kill(pos);
                } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
                    // An empty space becomes infested with a bug if
                    // exactly one or two bugs are adjacent to it.
                    new_map.infest(pos);
                }
            }
        }
        map = new_map;
    }

    map
}

fn biodiversity_rating(map: &Map) -> u32 {
    let mut rating = 0;
    let mut p = 1;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pos = (0, x, y);
            if map.is_bug_at(pos) {
                rating += p;
            }
            p <<= 1;
        }
    }

    rating
}


#[derive(Clone, Debug)]
struct Map {
    bugs: BTreeSet<Pos>,
}

impl Map {
    fn from_file<T: AsRef<Path>>(path: T) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read input");

        let mut bugs = BTreeSet::new();
        for (y, line) in contents.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == BUG {
                    bugs.insert((LEVEL_ZERO, x, y));
                }
            }
        }

        Map { bugs }
    }

    fn infest(&mut self, pos: Pos) {
        self.bugs.insert(pos);
    }

    fn kill(&mut self, pos: Pos) {
        self.bugs.remove(&pos);
    }

    fn total_number_of_bugs(&self) -> usize {
        self.bugs.len()
    }

    fn number_of_adjacent_bugs(&self, (d, x, y): Pos) -> usize {
        let mut n = 0;

        if x > 0 && self.is_bug_at((d, x - 1, y)) { n += 1 };
        if self.is_bug_at((d, x + 1, y)) { n += 1 };
        if y > 0 && self.is_bug_at((d, x, y - 1)) { n += 1 };
        if self.is_bug_at((d, x, y + 1)) { n += 1 };

        n
    }

    fn is_bug_at(&self, pos: Pos) -> bool {
        self.bugs.contains(&pos)
    }

    fn draw(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", if self.is_bug_at((0, x, y)) { BUG } else { EMPTY });
            }
            println!();
        }
        println!();
    }
}
