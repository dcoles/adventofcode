use std::fs;

fn main() {
    // Part 1
    run(10, true);
}

fn run(n: usize, print: bool) {
    let mut input = read_input("input.txt");
    if print { input.print() };

    for _n in 0..n {
        input.tick();
        if print { input.print() };
    }

    let mut n_wooded = 0;
    let mut n_lumberyards = 0;
    for row in &input.cells {
        for &tile in row {
            if tile == '|' {
                n_wooded += 1;
            } else if tile == '#' {
                n_lumberyards += 1;
            }
        }
    }

    println!("After {} minutes there are {} wooded acres and {} lumberyards",
             n, n_wooded, n_lumberyards);
    println!("Total resource value: {}", n_wooded * n_lumberyards);
}

fn read_input(filename: &str) -> Map {
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    let mut cells = Vec::new();
    let mut width = std::usize::MAX;
    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();
        width = width.min(line.len());
        cells.push(line);
    }

    Map { cells, width }
}

struct Map {
    cells: Vec<Vec<char>>,
    width: usize,
}

type Pos = (usize, usize);

impl Map {
    fn print(&self) {
        for row in &self.cells {
            for &tile in row {
                match tile {
                    '#' => print!("\x1b[31m{}\x1b[0m", tile),  // Red
                    '|' => print!("\x1b[32m{}\x1b[0m", tile),  // Green
                    _ => print!("{}", tile),
                }
            }
            println!();
        }
        println!();
    }

    fn get(&self, pos: Pos) -> char {
        self.cells[pos.1][pos.0]
    }

    fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.cells.len() {
            for x in 0..self.width {
                let pos = (x, y);
                let tile = self.get(pos);
                let adj = self.adjacent(pos);
                match tile {
                    '.' => if self.count_tiles_at(&adj, '|') >= 3 {
                        new_cells[pos.1][pos.0] = '|';
                    },
                    '|' => if self.count_tiles_at(&adj, '#') >= 3 {
                        new_cells[pos.1][pos.0] = '#';
                    },
                    '#' => if self.count_tiles_at(&adj, '#') == 0 || self.count_tiles_at(&adj, '|') == 0 {
                        new_cells[pos.1][pos.0] = '.';
                    },
                    t => panic!("Unknown tile {:?}", t),
                }
            }
        }

        self.cells = new_cells;
    }

    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        for (xoff, yoff) in &[(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)] {
            let (x, y) = (pos.0 as i32 + xoff, pos.1 as i32 + yoff);
            if 0 <= x && x < self.width as i32 && 0 <= y && y < self.cells.len() as i32 {
                result.push((x as usize, y as usize));
            }
        }

        result
    }

    fn count_tiles_at(&self, positions: &Vec<Pos>, tile: char) -> usize {
        positions.iter().filter(|&&p| self.get(p) == tile).count()
    }
}
