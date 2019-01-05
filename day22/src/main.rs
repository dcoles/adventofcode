const WIDTH: usize = 16;
const DEPTH: usize = 7740;
const ORIGIN: Pos = (0, 0);
const TARGET: Pos = (12, 763);

fn main() {
    let map = Map::new();
    println!("Risk level: {}", map.risk_level(ORIGIN, TARGET));
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
                let erosion_level = self.cells[y][x];
                match erosion_level % 3 {
                    0 => print!("."),
                    1 => print!("="),
                    2 => print!("|"),
                    _ => panic!(),
                }
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
}
