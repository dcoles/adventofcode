use std::fs;
use std::path::Path;
use std::collections::{HashSet, BTreeSet};
use std::ops::RangeInclusive;

type Pos = (i32, usize, usize);

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const LEVEL_ZERO: i32 = 0;
const BUG: char = '#';
const EMPTY: char = '.';

fn main() {
    let map = Map::from_file("input.txt");
    println!("Initial state:");
    map.draw(0);

    // Part 1
    let first_repeat = simulate(&map);
    println!("First repeated layout:");
    first_repeat.draw(0);
    println!("Part 1: Biodiversity rating: {}", biodiversity_rating(&first_repeat));

    // Part 2
    let minutes = 200;
    let after = simulate_for(&map, minutes);
    println!("After {} minutes:", minutes);
    after.draw_all_levels();
    println!("Part 2: Number of bugs: {}", after.total_number_of_bugs());
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

fn simulate_for(map: &Map, minutes: usize) -> Map {
    let mut map = map.clone();

    for _ in 0..minutes {
        let mut new_map = map.clone();

        let depth_range = map.depth_range();
        // Have to simulate the empty levels above and below the range of populated ones
        for d in (depth_range.start() - 1)..=(depth_range.end() + 1) {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if x == 2 && y == 2 {
                        // Recursive tile
                        continue;
                    }

                    let pos = (d, x, y);
                    let adjacent_bugs = map.number_of_adjacent_bugs_3d(pos);
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

    fn depth_range(&self) -> RangeInclusive<i32> {
        let &(min, _, _) = self.bugs.iter().next().unwrap();
        let &(max, _, _) = self.bugs.iter().next_back().unwrap();

        // Always include the empty level on either side
        min..=max
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

    fn number_of_adjacent_bugs_3d(&self, (d, x, y): Pos) -> usize {
        assert_ne!((x , y), (2, 2), "Recursive tile");
        let mut n = 0;

        if y == 0 && self.is_bug_at((d - 1, 2, 1)) {
            n += 1
        } else if y == HEIGHT - 1 && self.is_bug_at((d - 1, 2, 3)) {
            n += 1
        }

        if x == 0 && self.is_bug_at((d - 1, 1, 2)) {
            n += 1
        } else if x == WIDTH - 1 && self.is_bug_at((d - 1, 3, 2)) {
            n += 1
        }

        if x == 2 && y == 1 {
            for x in 0..WIDTH {
                if self.is_bug_at((d + 1, x, 0)) { n += 1; }
            }
        } else if x == 2 && y == 3 {
            for x in 0..WIDTH {
                if self.is_bug_at((d + 1, x, 4)) { n += 1; }
            }
        } else if x == 1 && y == 2 {
            for y in 0..HEIGHT {
                if self.is_bug_at((d + 1, 0, y)) { n += 1; }
            }
        } else if x == 3 && y == 2 {
            for y in 0..HEIGHT {
                if self.is_bug_at((d + 1, 4, y)) { n += 1; }
            }
        }

        if x > 0 && !(y == 2 && x == 3) && self.is_bug_at((d, x - 1, y)) {
            n += 1;
        }

        if x < WIDTH - 1 && !(y == 2 && x == 1) && self.is_bug_at((d, x + 1, y)) {
            n += 1;
        }

        if y > 0 && !(x == 2 && y == 3) && self.is_bug_at((d, x, y - 1)) {
            n += 1;
        }

        if y < HEIGHT - 1 && !(x == 2 && y == 1) && self.is_bug_at((d, x, y + 1)) {
            n += 1;
        }

        n
    }

    fn is_bug_at(&self, pos: Pos) -> bool {
        self.bugs.contains(&pos)
    }

    fn draw_all_levels(&self) {
        for depth in self.depth_range() {
            println!("Depth {}:", depth);
            self.draw(depth)
        }
    }

    fn draw(&self, depth: i32) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", if self.is_bug_at((depth, x, y)) { BUG } else { EMPTY });
            }
            println!();
        }
        println!();
    }
}
