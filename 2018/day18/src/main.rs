use std::collections::HashMap;
use std::fs;

fn main() {
    // Part 1
    run(10, true);

    // Part 2
    run(1000000000 , false);
}

fn run(n: usize, print: bool) {
    let mut input = read_input("input.txt");
    if print { input.print() };

    input.run_until(n);

    if print { input.print() };

    let mut n_wooded = 0;
    let mut n_lumberyards = 0;
    for &tile in &input.cells {
        if tile == '|' {
            n_wooded += 1;
        } else if tile == '#' {
            n_lumberyards += 1;
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
    let mut height = 0;
    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();
        width = width.min(line.len());
        height += 1;
        cells.extend(line);
    }

    Map { cells, width, height, next_state: HashMap::new(), minutes: 0 }
}

struct Map {
    cells: Vec<char>,
    width: usize,
    height: usize,
    next_state: HashMap<Vec<char>, (usize, Vec<char>)>,
    minutes: usize,
}

type Pos = (usize, usize);

impl Map {
    fn print(&self) {
        for (n, &tile) in self.cells.iter().enumerate() {
            match tile {
                '#' => print!("\x1b[31m{}\x1b[0m", tile),  // Red
                '|' => print!("\x1b[32m{}\x1b[0m", tile),  // Green
                _ => print!("{}", tile),
            }
            if n % self.width == self.width - 1 {
                println!();
            }
        }
        println!();
    }

    fn get(&self, pos: Pos) -> char {
        let n = self.width * pos.1 + pos.0;

        self.cells[n]
    }

    fn run_until(&mut self, minutes: usize) {
        while self.minutes < minutes {
            // Look for a cycle
            if let Some((n, next)) = self.next_state.get(&self.cells) {
                let delta = self.minutes - *n;
                self.minutes += ((minutes - self.minutes) / delta) * delta;
            }
            self.tick();
        }
    }

    fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                let tile = self.get(pos);
                let adj = self.adjacent(pos);
                let n = self.width * y + x;
                match tile {
                    '.' => if self.count_tiles_at(&adj, '|') >= 3 {
                        new_cells[n] = '|';
                    },
                    '|' => if self.count_tiles_at(&adj, '#') >= 3 {
                        new_cells[n] = '#';
                    },
                    '#' => if self.count_tiles_at(&adj, '#') == 0 || self.count_tiles_at(&adj, '|') == 0 {
                        new_cells[n] = '.';
                    },
                    t => panic!("Unknown tile {:?}", t),
                }
            }
        }

        self.next_state.insert(self.cells.clone(), (self.minutes, new_cells.clone()));
        self.cells = new_cells;
        self.minutes += 1;
    }

    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        for (xoff, yoff) in &[(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)] {
            let (x, y) = (pos.0 as i32 + xoff, pos.1 as i32 + yoff);
            if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
                result.push((x as usize, y as usize));
            }
        }

        result
    }

    fn count_tiles_at(&self, positions: &Vec<Pos>, tile: char) -> usize {
        positions.iter().filter(|&&p| self.get(p) == tile).count()
    }
}
