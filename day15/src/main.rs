use std::collections::HashSet;
use std::fmt;
use std::fs;

const INF: i32 = std::i32::MAX;
const UNSET: i32 = -1;
const DEBUG: bool = false;

fn main() {
    part1();
}

fn part1() {
    let mut world = World::from_file("input.txt");
    world.print();

    while world.round() {
        world.print();
    }
    world.print();

    println!("Combat ends after {} full rounds", world.n_rounds);
    let goblin_total_hp: i32 = world.chars.iter().filter(|c| c.race == 'G').map(|c| c.hp).sum();
    let elf_total_hp: i32 = world.chars.iter().filter(|c| c.race == 'E').map(|c| c.hp).sum();
    println!("Goblins have {} total hit points left", goblin_total_hp);
    println!("Elves have {} total hit points left", elf_total_hp);
}

type Map = Vec<Vec<char>>;
type DistanceMap = Vec<Vec<i32>>;

struct World {
    map: Map,
    chars: Vec<Character>,
    n_rounds: u32,
}

impl World {
    fn from_file(filename: &str) -> World {
        let mut map = Vec::new();
        let mut chars = Vec::new();

        let input = fs::read_to_string(filename)
            .expect("Failed to read input");
        for (y, line) in input.lines().enumerate() {
            let mut line_map = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if World::is_character(c) {
                    chars.push(Character::new(c, (x, y)));
                    line_map.push('.');
                } else {
                    line_map.push(c);
                }
            }
            map.push(line_map);
        }

        World { map, chars, n_rounds: 0 }
    }

    fn is_character(c: char) -> bool {
        c == 'G' || c == 'E'
    }

    fn print(&self) {
        let map = map_with_characters(&self.map, &self.chars);

        println!("End of round {}", self.n_rounds);
        let mut chars = self.chars.clone();
        chars.sort_by_key(|c| (c.position.1, c.position.0));
        for c in &chars {
            println!("- {}", c);
        }

        for (y, line) in map.iter().enumerate() {
            print!("{:2} ", y);
            for &c in line {
                match c {
                    'E' => print!("\x1b[32mE\x1b[0m "),  // Green
                    'G' => print!("\x1b[31mG\x1b[0m "),  // Red
                    x => print!("{} ", x),
                }
            }
            println!();
        }
        println!();
    }


    fn round(&mut self) -> bool {
        // Actions are performed in reading order
        self.chars.sort_by_key(|c| (c.position.1, c.position.0));

        for n in 0..self.chars.len() {
            let character = &self.chars[n];
            if character.is_dead() {
                // He's dead Jim
                continue
            }

            let map = map_with_characters(&self.map, &self.chars);
            let targets: Vec<_> = self.chars.iter().filter(|c|
                character.is_target(c)
            ).collect();
            if targets.is_empty() {
                // Combat ends
                return false;
            }

            let in_range: Vec<&Character> = self.chars.iter().filter(|c|
                character.is_in_range(c.position) && character.is_target(c)
            ).collect();

            // No one in range? Try moving
            if in_range.is_empty() {
                // Identify open tiles
                let mut open_tiles = HashSet::new();
                for target in targets {
                    open_tiles.extend(target.adjacent().iter()
                        .filter(|&&p|
                            self.is_empty_tile(p)
                        )
                    );
                }

                // Where should we move?
                if let Some(pos) = character.plan_move(&open_tiles, &map) {
                    println!("{} moving to {},{}", character, pos.0, pos.1);

                    self.chars[n].position = pos;
                } else {
                    // End-turn
                    continue;
                }
            }

            let character = &self.chars[n];
            let ap = character.ap;
            let mut in_range: Vec<_> = self.chars.iter()
                .enumerate()
                .filter(|&(_, c)|
                    character.is_in_range(c.position) && character.is_target(c)
                ).collect();
            in_range.sort_by_key(|&(_, c)| (c.hp, c.position.1, c.position.0));

            if let Some(&(m, _)) = in_range.first() {
                println!("{} attacks {}", character, self.chars[m]);
                self.chars[m].harm(ap);
            }
        }
        self.n_rounds += 1;

        true
    }

    fn is_empty_tile(&self, position: Pos) -> bool {
        self.map[position.1][position.0] == '.'
            && self.chars.iter().all(|c| c.position != position || c.is_dead())
    }

    fn adjacent(position: Pos) -> Vec<Pos> {
        [
            ( position.0, position.1 - 1),
            ( position.0 - 1, position.1),
            ( position.0 + 1, position.1),
            ( position.0, position.1 + 1),
        ].to_vec()
    }
}

type Pos = (usize, usize);

#[derive(Copy, Clone, Debug)]
struct Character {
    race: char,
    position: Pos,
    ap: i32,
    hp: i32,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{},{} [{:3}hp]", self.race, self.position.0, self.position.1, self.hp)
    }
}

impl Character {
    fn new(race: char, position: Pos) -> Character {
        Character { race, position, ap: 3, hp: 200 }
    }

    fn harm(&mut self, hp: i32) {
        self.hp = 0.max(self.hp - hp);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn is_target(&self, character: &Character) -> bool {
        character.race != self.race && !character.is_dead()
    }

    fn is_in_range(&self, position: Pos) -> bool {
        self.adjacent().iter().any(|&p| p == position)
    }

    fn adjacent(&self) -> Vec<Pos> {
        World::adjacent(self.position)
    }

    fn plan_move(&self, targets: &HashSet<Pos>, map: &Map) -> Option<Pos> {
        if targets.is_empty() {
            if DEBUG { println!("{} has no targets!", self) };
            return None;
        }

        let reachable = reachable_distance(targets, self.position, &map);
        if reachable.is_empty() {
            if DEBUG { println!("{} has no reachable targets!", self) };
            return None;
        }

        // Take a single step towards closest reachable target
        let &(target, _distance) = reachable.first().unwrap();
        let distance_map = build_distance_map(target, &map);

        if DEBUG { print_distance_map(&distance_map) };

        let mut adjacent: Vec<Pos> = self.adjacent().into_iter().filter(|&p| distance_map[p.1][p.0] >= 0).collect();
        adjacent.sort_by_key(|&p| (distance_map[p.1][p.0], p.1, p.0));

        Some(adjacent[0])
    }
}

fn reachable_distance(positions: &HashSet<Pos>, origin: Pos, map: &Map) -> Vec<(Pos, i32)> {
    let mut result = Vec::new();
    let distance_map = build_distance_map(origin, map);

    for &pos in positions {
        let distance = distance_map[pos.1][pos.0];
        if 0 <= distance && distance < INF {
            result.push((pos, distance));
        }
    }

    result.sort_by_key(|&(p, d)| (d, p.1, p.0));

    result
}

fn map_with_characters(map: &Map, characters: &Vec<Character>) -> Map {
    let mut map = map.clone();

    for character in characters {
        if character.is_dead() {
            continue;
        }
        map[character.position.1][character.position.0] = character.race;
    }

    map
}

fn build_distance_map(origin: Pos, map: &Map) -> DistanceMap {
    let mut distance_map: DistanceMap = Vec::new();

    for line in map {
        distance_map.push(line.iter().map(|&c| if c != '.' { INF } else { UNSET }).collect());
    }

    let mut edge: HashSet<Pos> = HashSet::new();
    let mut new_edge: HashSet<Pos> = HashSet::new();
    edge.insert(origin);
    distance_map[origin.1][origin.0] = 0;
    while !edge.is_empty() {
        for &pos in &edge {
            let distance: i32 = distance_map[pos.1][pos.0];
            for &adj_pos in &World::adjacent(pos) {
                if distance_map[adj_pos.1][adj_pos.0] == UNSET {
                    distance_map[adj_pos.1][adj_pos.0] = distance + 1;
                    new_edge.insert(adj_pos);
                }
            }
        }

        edge = new_edge;
        new_edge = HashSet::new();
    }

    distance_map
}

fn print_distance_map(distance_map: &DistanceMap) {
    for (y, row) in distance_map.iter().enumerate() {
        print!("{:2}", y);
        for &val in row {
            match val {
                INF => print!(" X"),
                UNSET => print!(" ?"),
                x => print!("{:2}", x),
            }
        }
        println!();
    }
    println!();
}
