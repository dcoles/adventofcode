use std::collections::HashMap;
use std::collections::HashSet;

const WIDTH: usize = 30;
const ORIGIN: Pos = (0, 0);

// Sample
//const DEPTH: usize = 510;
//const TARGET: Pos = (10, 10);

// Actual
const DEPTH: usize = 7740;
const TARGET: Pos = (12, 763);


fn main() {
    let map = Map::new();
    println!("Risk level: {}", map.risk_level(ORIGIN, TARGET));

    // Part 2
    plan(&map);
}

fn plan(map: &Map) {
    let mut edge: HashSet<(Pos, Equip)> = HashSet::new();
    let mut came_from: HashMap<(Pos, Equip), (Pos, Equip)> = HashMap::new();
    let mut cost_so_far: HashMap<(Pos, Equip), u32> = HashMap::new();

    let start = (ORIGIN, Equip::Torch);
    let goal = (TARGET, Equip::Torch);
    cost_so_far.insert(start, 0);

    let mut estimated_cost: HashMap<(Pos, Equip), u32> = HashMap::new();
    estimated_cost.insert(start, distance_heuristic(start, goal));

    edge.insert(start);
    while ! edge.is_empty() {
        let mut open: Vec<_> = edge.iter().collect();
        open.sort_by_key(|pe| estimated_cost.get(pe).unwrap());
        let &&current = open.first().unwrap();

        if current == goal {
            // Found!
            println!("Total cost: {}", *cost_so_far.get(&current).unwrap());
            return;
        }

        edge.remove(&current);

        for adj in map.adjacent(current.0, current.1) {
            let new_cost = *cost_so_far.get(&current).unwrap() + distance_heuristic(current, adj);

            if ! cost_so_far.contains_key(&adj) || new_cost < *cost_so_far.get(&adj).unwrap() {
                cost_so_far.insert(adj, new_cost);
                estimated_cost.insert(adj, new_cost + distance_heuristic(adj, goal));
                edge.insert(adj);
                came_from.insert(adj, current);
            }
        }
    }
}

// This is accurate for adjacent nodes, but may under-estimate for longer distances.
fn distance_heuristic(first: (Pos, Equip), second: (Pos, Equip)) -> u32 {
    let (first_pos, first_equip) = first;
    let (second_pos, second_equip) = second;

    ((first_pos.0 as i32 - second_pos.0 as i32).abs()
        + (first_pos.1 as i32 - second_pos.1 as i32).abs()
        + if first_equip != second_equip { 7 } else { 0 }) as u32
}

fn print_costs(gscore: &HashMap<(Pos, Equip), u32>) {
    for y in 0..DEPTH {
        for x in 0..WIDTH {
            print!("({:2},", *gscore.get(&((x, y), Equip::Nothing)).unwrap_or(&99));
            print!("{:2},", *gscore.get(&((x, y), Equip::Torch)).unwrap_or(&99));
            print!("{:2}) ", *gscore.get(&((x, y), Equip::Climbing)).unwrap_or(&99));
        }
        println!();
    }
    println!();
}

type Pos = (usize, usize);

struct Map {
    cells: Box<[[u32; WIDTH]; DEPTH]>,
}

impl Map {
    fn new() -> Map {
        let mut cells = Box::new([[0; WIDTH]; DEPTH]);
        for y in 0..DEPTH {
            for x in 0..WIDTH {
                let geo_index = match (x, y) {
                    (0, 0) => 0,
                    TARGET => 0,
                    (_, 0) => x as u32 * 16807,
                    (0, _) => y as u32 * 48271,
                    _ => cells[y][x-1] * cells[y-1][x],
                };
                cells[y][x] = (geo_index + DEPTH as u32) % 20183;
            }
        }
        Map { cells }
    }

    fn print(&self) {
        for y in 0..DEPTH {
            for x in 0..WIDTH {
                print!("{}", self.get_tile((x, y)));
            }
            println!();
        }
    }

    fn risk_level(&self, pos1: Pos, pos2: Pos) -> u32 {
        let mut result = 0;

        for y in pos1.1..=pos2.1 {
            for x in pos1.0..=pos2.0 {
                result += self.cells[y][x] % 3;
            }
        }

        result
    }

    fn get_tile(&self, pos: Pos) -> char {
        let erosion_level = self.cells[pos.1][pos.0];
        match erosion_level % 3 {
            0 => '.',
            1 => '=',
            2 => '|',
            _ => unreachable!(),
        }

    }

    fn adjacent(&self, pos: Pos, equip: Equip) -> Vec<(Pos, Equip)> {
        let mut result = Vec::new();
        for (xoff, yoff) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let x = pos.0 as i32 + xoff;
            let y = pos.1 as i32 + yoff;
            if 0 <= x && x < WIDTH as i32 && 0 <= y && y < DEPTH as i32 {
                let adj = (x as usize, y as usize);
                let adj_tile = self.get_tile(adj);
                if equip.can_use(adj_tile) {
                    result.push((adj, equip));
                }
            }
        }

        for &adj_equip in [Equip::Nothing, Equip::Torch, Equip::Climbing].iter().filter(|&&e| e != equip) {
            if adj_equip.can_use(self.get_tile(pos)) {
                result.push((pos, adj_equip));
            }
        }

        result
    }

}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Equip {
    Nothing,
    Torch,
    Climbing,
}

impl Equip {
    fn can_use(&self, tile: char) -> bool {
        match tile {
            '.' => self == &Equip::Torch || self == &Equip::Climbing,
            '=' => self == &Equip::Climbing || self == &Equip::Nothing,
            '|' => self == &Equip::Torch || self == &Equip::Nothing,
            _ => panic!("Unknown tile {:?}", tile),
        }
    }
}
