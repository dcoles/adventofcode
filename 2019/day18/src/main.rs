use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet, BinaryHeap};

type Pos = (usize, usize);

const ENTRANCE: char = '@';
const OPEN: char = '.';
const WALL: char = '#';

fn main() {
    let map = Map::from_file("input.txt");
    map.draw();

    // Part 1
    let (path, distance) = find_shortest_path(&map);
    println!("Part 1: Shortest path that collects all the keys: {} (distance: {})", path, distance);
}

fn find_shortest_path(map: &Map) -> (String, i32) {
    let objects = map.find_objects();
    let map_keys: HashSet<_> = objects.keys().copied().filter(|&c| is_key(c)).collect();

    let mut distance_cache: HashMap<(Pos, String), HashMap<char, usize>> = HashMap::new();
    let mut edge: BinaryHeap<(i32, String)> = vec![(0, ENTRANCE.to_string())].into_iter().collect();
    let mut cost_so_far: HashMap<(char, String), i32> = [((ENTRANCE, String::new()), 0)].iter().cloned().collect();
    while let Some((neg_dist, path)) = edge.pop() {
        let dist = -neg_dist;  // Make distance positive again
        let key = path.chars().last().unwrap();
        let keys: HashSet<_> = path.chars().skip(1).collect();

        if keys.len() == map_keys.len() {
            return (path, dist);
        }

        let mut s_keys: Vec<_> = keys.iter().copied().collect();
        s_keys.sort();
        let s_keys: String = s_keys.into_iter().collect();

        let distances = distance_cache.entry((objects[&key], s_keys)).or_insert_with(|| map.find_distances(objects[&key], &keys));
        let neighbours: HashSet<_> = distances.keys().copied().filter(|&o| is_key(o) && !path.contains(o)).collect();
        for &n in &neighbours {
            let mut new_path = path.to_owned();
            new_path.push(n);

            let mut new_s_keys: Vec<_> = new_path.chars().collect();
            new_s_keys.sort();
            let new_s_keys: String = new_s_keys.into_iter().collect();

            let new_cost = dist + (distances[&n] as i32);

            let cur_cost_so_far = cost_so_far.get(&(n, new_s_keys.clone()));
            if cur_cost_so_far.is_none() || new_cost < cur_cost_so_far.copied().unwrap() {
                cost_so_far.insert((n, new_s_keys), new_cost);

                // Use negative distance to make max-heap behave like a min-heap
                edge.push((-new_cost, new_path));
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

    fn find_distances(&self, from: Pos, keys: &HashSet<char>) -> HashMap<char, usize> {
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

    fn adjacent(&self, (x, y): Pos, keys: &HashSet<char>) -> Vec<Pos> {
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
