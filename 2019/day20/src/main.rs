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

struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    portals: HashMap<Pos, Pos>,
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

        let mut portals_ = HashMap::new();
        for (&portal, positions) in &portals {
            if portal == START {
                start = positions[0];
            } else if portal == END {
                end = positions[0];
            } else {
                // Portals are bi-directional
                portals_.insert(positions[0], positions[1]);
                portals_.insert(positions[1], positions[0]);
            }
        }

        Map { tiles, width, height, start, end, portals: portals_ }
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
        if let Some(&pos) = self.portals.get(&(x, y)) {
            adjacent.push(pos);
        }

        let pos = (x + 1, y);
        if is_open(self.at(pos)) || self.portals.contains_key(&pos) {
            adjacent.push(pos);
        }

        let pos = (x, y + 1);
        if is_open(self.at(pos)) || self.portals.contains_key(&pos) {
            adjacent.push(pos);
        }

        let pos = (x - 1, y);
        if is_open(self.at(pos)) || self.portals.contains_key(&pos) {
            adjacent.push(pos);
        }

        let pos = (x, y - 1);
        if is_open(self.at(pos)) || self.portals.contains_key(&pos) {
            adjacent.push(pos);
        }

        adjacent
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
