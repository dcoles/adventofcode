use std::path::Path;
use std::fs;
use std::collections::BTreeMap;

fn main() {
    let map = Map::from_file("input.txt");

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &Map) -> usize {
    let mut map = map.clone();

    for _ in 0..6 {
        map.tick3();
    }

    map.active_positions().len()
}

fn part2(map: &Map) -> usize {
    let mut map = map.clone();

    for _ in 0..6 {
        map.tick4();
    }

    map.active_positions().len()
}

const INACTIVE: char = '.';
const ACTIVE: char = '#';
type Pos = (i32, i32, i32, i32);

#[derive(Clone)]
struct Map {
    tiles: BTreeMap<Pos, bool>,
    min: Pos,
    max: Pos,
}

impl Map {
    fn from_file<T: AsRef<Path>>(path: T) -> Self {
        let mut tiles = BTreeMap::new();
        let min = (0, 0, 0, 0);
        let mut max = (0, 0, 0, 0);
        let input = fs::read_to_string(path).expect("Failed to read input");
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                tiles.insert((x as i32, y as i32, 0, 0), char == ACTIVE);
                max = (max.0.max(x as i32), max.1.max(y as i32), 0, 0);
            }
        }

        Map { tiles, min, max }
    }

    fn is_active(&self, pos: Pos) -> bool {
        self.tiles.get(&pos).copied().unwrap_or(false)
    }

    fn active(&mut self, pos: Pos, active: bool) {
        self.tiles.insert(pos, active);
        self.min = (self.min.0.min(pos.0), self.min.1.min(pos.1), self.min.2.min(pos.2), self.min.3.min(pos.3));
        self.max = (self.max.0.max(pos.0), self.max.1.max(pos.1), self.max.2.max(pos.2), self.max.3.max(pos.3));
    }

    fn tick3(&mut self) {
        let cur = self.tiles.clone();

        for x in self.min.0-1..=self.max.0+1 {
            for y in self.min.1 - 1..=self.max.1 + 1 {
                for z in self.min.2 - 1..=self.max.2 + 1 {
                    let active = self.is_active((x, y, z, 0));
                    let neighbours = adjacent3((x, y, z, 0));
                    let n_active = neighbours.iter().filter(|&p| cur.get(p).copied().unwrap_or(false)).count();
                    if active {
                        if !(n_active == 2 || n_active == 3) {
                            self.active((x, y, z, 0), false);
                        }
                    } else {
                        if n_active == 3 {
                            self.active((x, y, z, 0), true);
                        }
                    }
                }
            }
        }
    }

    fn tick4(&mut self) {
        let cur = self.tiles.clone();

        for x in self.min.0-1..=self.max.0+1 {
            for y in self.min.1 - 1..=self.max.1 + 1 {
                for z in self.min.2 - 1..=self.max.2 + 1 {
                    for w in self.min.3 - 1..=self.max.3 + 1 {
                        let active = self.is_active((x, y, z, w));
                        let neighbours = adjacent4((x, y, z, w));
                        let n_active = neighbours.iter().filter(|&p| cur.get(p).copied().unwrap_or(false)).count();
                        if active {
                            if !(n_active == 2 || n_active == 3) {
                                self.active((x, y, z, w), false);
                            }
                        } else {
                            if n_active == 3 {
                                self.active((x, y, z, w), true);
                            }
                        }
                    }
                }
            }
        }
    }

    fn active_positions(&self) -> Vec<Pos> {
        self.tiles.iter().filter(|(_, active)| **active).map(|(pos, _)| *pos).collect()
    }

    fn print(&self, z: i32, w: i32) {
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                print!("{}", if self.is_active((x, y, z, w)) { ACTIVE } else { INACTIVE });
            }
            println!();
        }
        println!();
    }
}

fn adjacent3(pos: Pos) -> Vec<Pos> {
    let mut positions = Vec::new();
    for x in pos.0-1..=pos.0+1 {
        for y in pos.1-1..=pos.1+1 {
            for z in pos.2-1..=pos.2+1 {
                if (x, y, z, 0) != pos {
                    positions.push((x, y, z, 0));
                }
            }
        }
    }

    positions
}

fn adjacent4(pos: Pos) -> Vec<Pos> {
    let mut positions = Vec::new();
    for x in pos.0-1..=pos.0+1 {
        for y in pos.1-1..=pos.1+1 {
            for z in pos.2-1..=pos.2+1 {
                for w in pos.3-1..=pos.3+1 {
                    if (x, y, z, w) != pos {
                        positions.push((x, y, z, w));
                    }
                }
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent3() {
        let adj = adjacent3((0, 0, 0, 0));
        assert_eq!(adj.len(), 26);
    }

    #[test]
    fn test_adjacent4() {
        let adj = adjacent4((0, 0, 0, 0));
        assert_eq!(adj.len(), 80);
    }

    #[test]
    fn test_sample1_part1() {
        let map = Map::from_file("sample1.txt");
        assert_eq!(part1(&map), 112);
    }

    #[test]
    fn test_sample1_part2() {
        let map = Map::from_file("sample1.txt");
        assert_eq!(part2(&map), 848);
    }
}

