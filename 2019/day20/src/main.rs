use std::collections::{HashMap, BinaryHeap};
use std::fs;
use std::path::Path;

type Portal = (char, char);
type Pos = (usize, usize);

const OPEN: char = '.';
const SPACE: char = ' ';
const START: Portal = ('A', 'A');
const END: Portal = ('Z', 'Z');

fn main() {
    let map = Map::from_file("input.txt");
    map.draw();

    // Part 1
    let distance = find_distance_to_exit(&map);
    println!("Part 1: Steps required to reach ZZ from AA: {}", distance);

    // Part 2
    let distance = find_distance_to_exit_recursive(&map);
    println!("Part 2: Steps required to reach ZZ from AA in a recursive maze: {}", distance);
}

/// Find distance from start to exit.
fn find_distance_to_exit(map: &Map) -> u32 {
    // Search using A*
    let mut edge: BinaryHeap<(i32, Pos)> = [(0, map.start)].iter().copied().collect();
    let mut distance_so_far: HashMap<Pos, i32> = HashMap::new();
    while let Some((ndist, pos)) = edge.pop() {
        let distance = -ndist;

        if pos == map.end {
            return distance as u32;
        }

        let adjacent = map.adjacent(pos);
        for adj in adjacent {
            let dist = distance + 1;
            if !distance_so_far.contains_key(&adj) || dist < distance_so_far[&adj] {
                distance_so_far.insert(adj, dist);
                edge.push((-dist, adj));
            }
        }
    }
    panic!("Could not find exit!");
}

/// Find distance from start to exit.
fn find_distance_to_exit_recursive(map: &Map) -> u32 {
    // Search using A*
    // We start at priority 0, the start (AA) and level 0
    let mut edge: BinaryHeap<(i32, Pos, u32)> = [(0, map.start, 0)].iter().copied().collect();
    let mut distance_so_far: HashMap<(Pos, u32), i32> = [((map.start, 0), 0)].iter().copied().collect();
    while let Some((_, pos, level)) = edge.pop() {
        let distance = distance_so_far[&(pos, level)];

        if pos == map.end {
            return distance as u32;
        }

        let adjacent = map.adjacent(pos).into_iter().filter(|&adj| {
            if level == 0 {
                // At level 0 we can't use the portals to an inner location (going up a level)
                !map.outer_portals.contains_key(&pos)
            } else {
                // At other levels all portals work, but start/end are blocked
                adj != map.start && adj != map.end
            }
        });

        for adj in adjacent {
            let level = if map.inner_portals.contains_key(&pos) && map.outer_portals.contains_key(&adj) {
                // This is an inner portal leading to an outer location
                level + 1
            } else if map.outer_portals.contains_key(&pos) && map.inner_portals.contains_key(&adj) {
                // This is an outer portal leading to an inner location
                level - 1
            } else {
                // Otherwise the level remains unchanged
                level
            };
            let new_distance = distance + 1;
            if !distance_so_far.contains_key(&(adj, level)) || new_distance < distance_so_far[&(adj, level)] {
                distance_so_far.insert((adj, level), new_distance);
                let priority = -(level as i32);
                edge.push((priority, adj, level));
            }
        }
    }
    panic!("Could not find exit!");
}

struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    inner_portals: HashMap<Pos, Pos>,
    outer_portals: HashMap<Pos, Pos>,
}

impl Map {
    fn from_file<T: AsRef<Path>>(path: T) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read input");
        let width = contents.find('\n').unwrap();
        let height = contents.lines().count();
        let tiles: Vec<_> = contents.lines().flat_map(|line| line.chars()).collect();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let portals= find_portals(&tiles, width, height);

        let mut inner = HashMap::new();
        let mut outer = HashMap::new();
        for (&portal, positions) in &portals {
            if portal == START {
                start = positions[0];
            } else if portal == END {
                end = positions[0];
            } else {
                // Portals are bi-directional
                let p1 = positions[0];
                let p2 = positions[1];
                if p1.0 < 5 || p1.0 > width - 5 || p1.1 < 5 || p1.1 > height - 5 {
                    // P1 is the outer portal
                    outer.insert(p1, p2);
                    inner.insert(p2, p1);
                } else {
                    // P1 is the inner portal
                    inner.insert(p1, p2);
                    outer.insert(p2, p1);
                }
                inner.insert(positions[0], positions[1]);
                inner.insert(positions[1], positions[0]);
            }
        }

        Map { tiles, width, height, start, end, inner_portals: inner, outer_portals: outer }
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.at((x, y)));
            }
            println!();
        }
    }


    fn at(&self, (x, y): Pos) -> char {
        self.tiles.get(y * self.width + x).copied().unwrap_or(SPACE)
    }

    fn adjacent(&self, (x, y): Pos) -> Vec<Pos> {
        let mut adjacent = Vec::new();

        // Tiles reachable through Portals are also adjacent
        if let Some(&pos) = self.inner_portals.get(&(x, y)) {
            adjacent.push(pos);
        } else if let Some(&pos) = self.outer_portals.get(&(x, y)) {
            adjacent.push(pos);
        }

        let pos = (x + 1, y);
        if is_open(self.at(pos)) || self.is_portal_position(pos) {
            adjacent.push(pos);
        }

        let pos = (x, y + 1);
        if is_open(self.at(pos)) || self.is_portal_position(pos) {
            adjacent.push(pos);
        }

        let pos = (x - 1, y);
        if is_open(self.at(pos)) || self.is_portal_position(pos) {
            adjacent.push(pos);
        }

        let pos = (x, y - 1);
        if is_open(self.at(pos)) || self.is_portal_position(pos) {
            adjacent.push(pos);
        }

        adjacent
    }

    fn is_portal_position(&self, pos: Pos) -> bool {
        self.inner_portals.contains_key(&pos) || self.outer_portals.contains_key(&pos)
    }
}

/// Find all portals on a map.
fn find_portals(tiles: &[char], width: usize, height: usize) -> HashMap<Portal, Vec<Pos>> {
    assert_eq!(width * height, tiles.len());
    let mut portals: HashMap<Portal, Vec<Pos>> = HashMap::new();

    let at = |(x, y): Pos| -> char {
        tiles.get(y * width + x).copied().unwrap_or(SPACE)
    };

    for y in 0..height {
        for x in 0..width {
            let c1 = at((x, y));
            if c1.is_alphabetic() {
                // Is this a horizontal label?
                let c2 = at((x + 1, y));
                if c2.is_alphabetic() {
                    if at((x + 2, y)) == OPEN {
                        portals.entry((c1, c2)).or_default().push((x + 2, y));
                    } else {
                        portals.entry((c1, c2)).or_default().push((x - 1, y));
                    }
                    continue;
                }

                // Is this a vertical label?
                let c2 = at((x, y + 1));
                if c2.is_alphabetic() {
                    if at((x, y + 2)) == OPEN {
                        portals.entry((c1, c2)).or_default().push((x, y + 2));
                    } else {
                        portals.entry((c1, c2)).or_default().push((x, y - 1));
                    }
                    continue;
                }
            }
        }
    }

    portals
}

/// Is a tile open?
fn is_open(tile: char) -> bool {
    tile == OPEN
}
