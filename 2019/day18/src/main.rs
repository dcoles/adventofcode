use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet, BinaryHeap, BTreeSet};

type Pos = (usize, usize);

const ENTRANCE: char = '@';
const ENTRANCE1: char = '1';
const ENTRANCE2: char = '2';
const ENTRANCE3: char = '3';
const ENTRANCE4: char = '4';
const OPEN: char = '.';
const WALL: char = '#';

fn main() {
    // Part 1
    let map = Map::from_file("input1.txt");
    map.draw();

    let (path, distance) = find_shortest_path(&map, &[ENTRANCE]);
    println!("Part 1: Shortest path that collects all the keys: {:?} (distance: {})", path, distance);

    // Part 2
    let map = Map::from_file("input2.txt");
    map.draw();

    let (path, distance) = find_shortest_path(&map, &[ENTRANCE1, ENTRANCE2, ENTRANCE3, ENTRANCE4]);
    println!("Part 2: Shortest path that collects all the keys: {:?} (distance: {})", path, distance);
}

fn find_shortest_path(map: &Map, start: &[char]) -> (Vec<String>, i32) {
    let objects = map.find_objects();
    let map_keys: HashSet<_> = objects.keys().copied().filter(|&c| is_key(c)).collect();

    // Cache of (pos, keys) -> {obj -> distance}
    let mut distance_cache: HashMap<(Pos, BTreeSet<char>), HashMap<char, usize>> = HashMap::new();

    // Priority queue of (distance-so-far, [path])
    let mut edge: BinaryHeap<(i32, Vec<String>)> = [
        (0, start.iter().map(|c| c.to_string()).collect()),
    ].iter().cloned().collect();

    // Map of ([robot_tile], {keys}) -> distance
    let mut dist_so_far: HashMap<(Vec<char>, BTreeSet<char>), i32> = [
        ((start.iter().copied().collect(), BTreeSet::new()), 0),
    ].iter().cloned().collect();

    while let Some((neg_dist, path)) = edge.pop() {
        let dist = -neg_dist;  // Make distance positive again
        let key: Vec<_> = path.iter().map(|k| k.chars().last().unwrap()).collect();
        let keys: BTreeSet<char> = path.iter().flat_map(|k| k.chars().skip(1)).collect();

        // We're done if we have all the keys
        if keys.len() == map_keys.len() {
            return (path, dist);
        }

        // Get a mapping of key -> (robot-index, distance)
        let mut distances: HashMap<char, (usize, i32)> = HashMap::new();
        for (i, &k) in key.iter().enumerate() {
            let pos = objects[&k];
            distances.extend(distance_cache.entry((pos, keys.clone())).or_insert_with(|| map.find_distances(objects[&k], &keys)).iter().map(|(&k, &d)| (k, (i, d as i32))));
        }
        
        let neighbours: HashSet<_> = distances.keys().copied().filter(|&o| is_key(o) && !path.iter().any(|p| p.contains(o))).collect();
        for &n in &neighbours {
            let mut key = key.clone();
            let mut keys = keys.clone();
            keys.insert(n);
            let i = distances[&n].0;
            let new_dist = dist + (distances[&n].1 as i32);

            let mut path = path.clone();
            path[i].push(n);
            key[i] = n;

            let cur_dist = dist_so_far.get(&(key.clone(), keys.clone())).copied();
            if cur_dist.is_none() || new_dist < cur_dist.unwrap() {
                dist_so_far.insert((key, keys), new_dist);

                // Use negative distance to make max-heap behave like a min-heap
                edge.push((-new_dist, path));
            }
        }
    }

    panic!("Not all keys are reachable!")
}

#[derive(Clone)]
struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_file<T: AsRef<Path>>(path: T) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read input");

        let mut tiles = Vec::new();
        let height = contents.lines().count();
        let width = contents.lines().next().unwrap_or("").len();

        for line in contents.lines() {
            tiles.extend(line.chars());
        }

        assert_eq!(tiles.len(), width * height);
        Map { tiles, width, height }
    }

    fn at(&self, (x, y): Pos) -> char {
        assert!(x < self.width);
        assert!(y < self.height);
        self.tiles[y * self.width + x]
    }

    fn find_objects(&self) -> HashMap<char, Pos> {
        self.tiles.iter().copied()
            .enumerate()
            .filter(|&(_, c)| c != WALL || c == ENTRANCE)
            .map(|(n, c)| (c, (n % self.width, n / self.width)))
            .collect()
    }

    fn find_distances(&self, from: Pos, keys: &BTreeSet<char>) -> HashMap<char, usize> {
        let mut distances = HashMap::new();

        // Do a simple BFS
        let mut seen: HashSet<Pos> = HashSet::new();
        let mut edge: HashSet<Pos> = [from].iter().copied().collect();
        let mut d = 0;
        while !edge.is_empty() {
            // Remember distances to keys and doors
            for (_, c) in edge.iter().copied().map(|p| (p, self.at(p)) ).filter(|&(p, c)| (is_door(c) || is_key(c) && p != from)) {
                distances.insert(c, d);
            }
            seen.extend(edge.iter().copied());

            // Calculate new edge
            edge = edge.into_iter()
                .flat_map(|p| self.adjacent(p, keys).into_iter().filter(|&p| !seen.contains(&p)))
                .collect();

            d += 1;
        }

        distances
    }

    fn adjacent(&self, (x, y): Pos, keys: &BTreeSet<char>) -> Vec<Pos> {
        let mut adjacent = Vec::new();

        let tile = self.at((x, y));
        if is_door(tile) && !keys.contains(&key_for_door(tile)) {
            // Nothing is adjacent to a locked door
            return adjacent;
        }

        if x > 0 && !self.is_wall((x - 1, y)) {
            adjacent.push((x - 1, y));
        }

        if x < self.width - 1 && !self.is_wall((x + 1, y)) {
            adjacent.push((x + 1, y));
        }

        if y > 0 && !self.is_wall((x, y - 1)) {
            adjacent.push((x, y - 1));
        }

        if y < self.height - 1 && !self.is_wall((x, y + 1)) {
            adjacent.push((x, y + 1));
        }

        adjacent
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.at((x, y));
                let color = match c {
                    c if is_key(c) => "\x1B[31m",  // Red
                    c if is_door(c) => "\x1B[35m",  // Magenta
                    OPEN => "\x1B[33m",  // Yellow
                    ENTRANCE => "\x1b[36m",  // Cyan
                    _ => "",
                };
                print!("{}{}\x1B[m", color, c);
            }
            println!();
        }
    }

    fn is_wall(&self, (x, y): Pos) -> bool {
        self.at((x, y)) == WALL
    }
}

fn is_door(obj: char) -> bool {
    obj.is_ascii_uppercase()
}

fn is_key(obj: char) -> bool {
    obj.is_ascii_lowercase()
}

fn key_for_door(door: char) -> char {
    assert!(door.is_ascii_uppercase());
    door.to_ascii_lowercase()
}
