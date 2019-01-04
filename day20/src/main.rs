use std::fs;

const WIDTH: usize = 105;
const HEIGHT: usize = 105;

fn main() {
    let input = read_input("input.txt");
    let mut map = Map::new();

    let mut stack = Vec::new();
    let mut pos = (WIDTH/2, HEIGHT/2);
    let mut n_max = 0;
    for c in input {
        let n = map.get(pos);
        match c {
            '^' => (),
            'N' => pos = (pos.0, pos.1 - 1),
            'S' => pos = (pos.0, pos.1 + 1),
            'E' => pos = (pos.0 + 1, pos.1),
            'W' => pos = (pos.0 - 1, pos.1),
            '(' => stack.push(pos),
            '|' => pos = *stack.last().unwrap(),
            ')' => { stack.pop(); },
            '$' => (),
            _ => panic!("Unknown tile {:?}", c),
        }
        if map.get(pos) == 0 {
            map.set(pos, n + 1);
            n_max = n_max.max(n + 1);
        }
    }
    map.print();
    println!("Largest number of doors required to reach a room is {}", n_max - 1);

    let mut count = 0;
    for row in &map.tiles[..] {
        for &cell in &row[..] {
            if cell > 1000 {
                count += 1;
            }
        }
    }
    println!("{} rooms have a shortest path that passes through at least 1000 doors", count);
}

fn read_input(filename: &str) -> Vec<char> {
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    input.trim().chars().collect()
}

type Pos = (usize, usize);

struct Map {
    tiles: Box<[[u32; WIDTH]; HEIGHT]>,
}

impl Map {
    fn new() -> Map {
        Map { tiles: Box::new([[0; WIDTH]; HEIGHT]) }
    }

    fn get(&self, pos: Pos) -> u32 {
        self.tiles[pos.1][pos.0]
    }

    fn set(&mut self, pos: Pos, val: u32) {
        self.tiles[pos.1][pos.0] = val;
    }

    fn print(&self) {
        for row in &self.tiles[..] {
            for &cell in &row[..] {
                print!("{:5}", cell);
            }
            println!();
        }
    }
}
