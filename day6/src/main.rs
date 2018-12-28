use std::collections::HashSet;
use std::fs;

fn main() {
    let mut target_grid = Grid::new(400, 400);
    let mut result_grid = Grid::new(target_grid.width, target_grid.height);

    let input= read_input();
    for (coord, n) in input.iter().zip(1u8..) {
        target_grid.set(&coord, n);
    }

    for y in 0..10 {
        println!("{}...", y);
        for x in 0..10 {
            let c = Coord(y as i32, x as i32);
            result_grid.set(&c, find_nearest(&target_grid, &c));
        }
    }

    result_grid.print();
}

fn read_input() -> Vec<Coord> {
    let mut result = Vec::new();

    let input = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    for line in input.lines() {
        let v: Vec<i32> = line.splitn(2, ",")
            .map(|s| s.trim().parse().expect("Failed to parse coord"))
            .collect();
        result.push(Coord(v[0], v[1]));
    }

    result
}

fn find_nearest(grid: &Grid, point: &Coord) -> u8 {
    let mut seen: HashSet<Coord> = HashSet::new();

    // Start at the current point
    let mut edge: HashSet<Coord> = HashSet::new();
    let point: Coord = point.clone();
    edge.insert(point.clone());

    let mut found: HashSet<u8> = HashSet::new();
    while found.is_empty() && !edge.is_empty() {
        let mut new_edge: HashSet<Coord> = HashSet::new();
        for c in edge.iter() {
            for n in grid.neighbours(c) {
                let val = grid.get(&n);
                if val != 0 {
                    found.insert(val);
                }
                if ! seen.contains(&n) {
                    new_edge.insert(n);
                }
            }
        }
        seen.extend(edge);
        edge = new_edge;
    }

    if found.len() != 1 {
        return 0
    } else {
        return *found.iter().next().unwrap();
    }
}

#[derive(Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
struct Coord(i32, i32);

struct Grid {
    width: usize,
    height: usize,
    values: Vec<Vec<u8>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {width, height, values: vec![vec![0; width]; height] }
    }

    fn print(&self) {
        for row in &self.values {
            println!("{}", row.iter()
                .map(|v| if *v != 0 { format!("{:2}", v) } else { String::from(" .") })
                .collect::<Vec<String>>()
                .join(" "))
        }
    }

    fn get(&self, coord: &Coord) -> u8 {
        self.values[coord.1 as usize][coord.0 as usize]
    }

    fn set(&mut self, coord: &Coord, value: u8) {
        self.values[coord.1 as usize][coord.0 as usize] = value;
    }

    fn valid(&self, coord: &Coord) -> bool {
        0 <= coord.0 && coord.0 < self.width as i32
            && 0 <= coord.1 && coord.1 < self.height as i32
    }

    fn neighbours(&self, coord: &Coord) -> HashSet<Coord> {
        let mut result = HashSet::new();
        for (xoff, yoff) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x: i32 = coord.0 + *xoff;
            let y: i32 = coord.1 + *yoff;
            let c = Coord(x, y);
            if self.valid(&c) {
                result.insert(c);
            }
        }
        result
    }
}
