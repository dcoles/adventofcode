use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;
use std::fs;

type Input = Vec<Tile>;
type Pos = [usize; 2];

/// Size of a Tile
const SIZE: usize = 10;

const SEA_MONSTER_SIZE: usize = 15;
const SEA_MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

fn main() {
    let input = read_input("input.txt");

    // Part 1
    println!("Part 1: {}", part1(&input));
    
    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(tiles: &Input) -> u64 {
    let mut corners = Vec::new();
    for i in 0..tiles.len() {
        let mut n = 0;
        for j in 0..tiles.len() {
            if i == j {
                continue;
            }

            let mut tile = tiles[j].clone();

            for _ in 0..2 {
                for _ in 0..4 {
                    if tiles[i].right_edge() == tile.left_edge()
                        || tiles[i].left_edge() == tile.right_edge()
                        || tiles[i].top_edge() == tile.bottom_edge()
                        || tiles[i].bottom_edge() == tile.top_edge() {
                        n += 1;
                    }

                    tile.rotate();
                }

                tile.flip();
            }

            assert_eq!(tiles[j].data, tile.data)
        }

        if n == 2 {
            corners.push(tiles[i].id);
        }
    }

    corners.into_iter().product()
}

fn part2(tiles: &Input) -> usize {
    let start = tiles[0].id;
    let tiles: HashMap<u64, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();

    let mut positions = HashMap::new();
    positions.insert(start, ([0, 0], tiles[&start].clone()));

    let mut edge = vec![start];
    while let Some(id) = edge.pop() {
        let ([x, y], tile) = positions[&id].clone();

        'outer: for other in tiles.values().filter(|t| t.id != id) {
            if positions.contains_key(&other.id) {
                continue;
            }

            let mut other = other.clone();

            for _ in 0..2 {
                for _ in 0..4 {
                    if tile.right_edge() == other.left_edge() {
                        edge.push(other.id);
                        positions.insert(other.id, ([x + 1, y], other));
                        continue 'outer;
                    } else if tile.left_edge() == other.right_edge() {
                        edge.push(other.id);
                        positions.insert(other.id, ([x - 1, y], other));
                        continue 'outer;
                    } else if tile.top_edge() == other.bottom_edge() {
                        edge.push(other.id);
                        positions.insert(other.id, ([x, y - 1], other));
                        continue 'outer;
                    } else if tile.bottom_edge() == other.top_edge() {
                        edge.push(other.id);
                        positions.insert(other.id, ([x, y + 1], other));
                        continue 'outer;
                    }

                    other.rotate();
                }

                other.flip();
            }

        }
    }

    // Merge tiles into single map
    let mut map = HashMap::new();

    let min_x = positions.values().map(|&([x, _], _)| x).min().unwrap();
    let min_y = positions.values().map(|&([_, y], _)| y).min().unwrap();
    for (pos, tile) in positions.into_values() {
        let xx = (pos[0] - min_x) as usize;
        let yy = (pos[1] - min_y) as usize;
        for y in 1..SIZE-1 {
            for x in 1..SIZE-1 {
                map.insert([xx * (SIZE - 2) + x - 1, yy * (SIZE - 2) + y - 1], tile[[x, y]]);
            }
        }
    }

    // Search for seamonsters
    let mut n_seamonsters = 0;
    'found: for _ in 0..2 {
        for _ in 0..4 {
            for &pos in map.keys() {
                if is_seamonster(&map, pos) {
                    n_seamonsters += 1;
                }
            }

            if n_seamonsters > 0 {
                // Y'arr!
                break 'found;
            }

            rotate(&mut map);
        }
        flip(&mut map);
    }

    // Calculate the roughness of the water
    map.values().filter(|&&c| c == '#').count() - n_seamonsters * SEA_MONSTER_SIZE
}

fn is_seamonster(map: &HashMap<Pos, char>, pos: Pos) -> bool {
    for y in 0..SEA_MONSTER.len() {
        let line = SEA_MONSTER[y];
        for (x, c) in line.chars().enumerate() {
            if c == '#' && map.get(&[pos[0] + x, pos[1] + y]) != Some(&'#') {
                return false;
            }
        }
    }

    true
}

fn rotate(map: &mut HashMap<Pos, char>) {
    let width = map.keys().map(|&[x, _]| x).max().unwrap() + 1;
    let height = map.keys().map(|&[_, y]| y).max().unwrap() + 1;

    let mut new_map = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            new_map.insert([x, y], map[&[y, height - 1 - x]]);
        }
    }

    *map = new_map;
}

fn flip(map: &mut HashMap<Pos, char>) {
    let width = map.keys().map(|&[x, _]| x).max().unwrap() + 1;
    let height = map.keys().map(|&[_, y]| y).max().unwrap() + 1;

    let mut new_map = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            new_map.insert([x, y], map[&[width - 1 - x, y]]);
        }
    }

    *map = new_map;
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");

    input.trim().split("\n\n").map(|s| Tile::from_str(s)).collect()
}

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    data: [[char; SIZE]; SIZE],
}

impl Tile {
    fn from_str(s: &str) -> Self {
        let id = s.lines().next().unwrap()[5..9].parse().expect("Failed to parse number");

        let mut data = [['\0'; SIZE]; SIZE];
        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, c) in line.chars().enumerate() {
                data[y][x] = c;
            }
        }

        Tile { id, data }
    }

    /// Print tile to console.
    fn print(&self) {
        println!("Tile {}:", self.id);
        for y in 0..SIZE {
            for x in 0..SIZE {
                print!("{} ", self.data[y][x]);
            }
            println!();
        }
        println!();
    }

    /// Rotate a tile 90 degrees clockwise.
    fn rotate(&mut self) {
        let mut data = [['\0'; SIZE]; SIZE];
        for y in 0..SIZE {
            for x in 0..SIZE {
                data[y][x] = self.data[SIZE - 1 - x][y];
            }
        }

        self.data = data;
    }

    /// Flip tile across the y-axis.
    fn flip(&mut self) {
        let mut data = [['\0'; SIZE]; SIZE];
        for y in 0..SIZE {
            for x in 0..SIZE {
                data[y][x] = self.data[y][SIZE - 1 - x];
            }
        }

        self.data = data;
    }

    fn top_edge(&self) -> [char; SIZE] {
        let mut edge = ['\0'; SIZE];

        for x in 0..SIZE {
            edge[x] = self.data[0][x];
        }

        edge
    }

    fn bottom_edge(&self) -> [char; SIZE] {
        let mut edge = ['\0'; SIZE];

        for x in 0..SIZE {
            edge[x] = self.data[SIZE - 1][x];
        }

        edge
    }
    
    fn left_edge(&self) -> [char; SIZE] {
        let mut edge = ['\0'; SIZE];

        for y in 0..SIZE {
            edge[y] = self.data[y][0];
        }

        edge
    }
    
    fn right_edge(&self) -> [char; SIZE] {
        let mut edge = ['\0'; SIZE];

        for y in 0..SIZE {
            edge[y] = self.data[y][SIZE - 1];
        }

        edge
    }
}

impl Index<Pos> for Tile {
    type Output = char;

    fn index(&self, [x, y]: Pos) -> &Self::Output {
        &self.data[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("example1.txt");
        assert_eq!(part1(&input), 20899048083289);
    }
    
    #[test]
    fn test_part2() {
        let input = read_input("example1.txt");
        assert_eq!(part2(&input), 273);
    }
}
