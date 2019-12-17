use intcode::emulator::{Program, IntcodeEmulator};
use std::io;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

type Pos = (usize, usize);

const SPACE: char = '.';
const SCAFFOLDING: char = '#';
const ROBOT_UP: char = '^';
const ROBOT_DOWN: char = 'v';
const ROBOT_LEFT: char = '<';
const ROBOT_RIGHT: char = '>';

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let view = get_view(&program);
    let map = Map::from_view(&view);
    let intersections = map.find_intersections();
    let calibration: usize = alignment_parameters(&intersections).into_iter().sum();

    println!("Part 1: Sum of alignment parameters: {}", calibration);
}

fn alignment_parameters(intersections: &[Pos]) -> Vec<usize> {
    intersections.into_iter().map(|&(x, y)| x * y).collect()
}

fn get_view(program: &Program) -> String {
    let output = Rc::new(RefCell::new(String::new()));
    {
        let output = Rc::clone(&output);
        let input_handler = Box::new(|| Err(io::Error::new(io::ErrorKind::BrokenPipe, "No input")));
        let output_handler = Box::new(move |word| {
            let c: char = (word as u8).into();
            output.borrow_mut().push(c);

            Ok(())
        });
        let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
        cpu.load_program(&program);
        cpu.run();
    }

    Rc::try_unwrap(output).unwrap().into_inner()
}

struct Map {
    tiles: HashMap<Pos, char>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_view(view: &str) -> Map {
        let mut tiles = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in view.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if y == 0 {
                    width += 1;
                }

                // Ignore space
                if c == SPACE {
                    continue;
                }

                tiles.insert((x, y), c);
            }
            height += 1;
        }

        Map { tiles, width, height }
    }

    fn get_tile(&self, pos: (usize, usize)) -> char {
        self.tiles.get(&pos).copied().unwrap_or(SPACE)
    }

    fn find_intersections(&self) -> Vec<Pos> {
        self.tiles.keys()
            .copied()
            .filter(|&pos| self.is_intersection(pos))
            .collect()
    }

    fn is_intersection(&self, (x, y): Pos) -> bool {
        self.get_tile((x, y)) == SCAFFOLDING
            && x != 0
            && x < self.width
            && y != 0
            && y < self.height
            && self.get_tile((x + 1, y)) == SCAFFOLDING
            && self.get_tile((x - 1, y)) == SCAFFOLDING
            && self.get_tile((x, y + 1)) == SCAFFOLDING
            && self.get_tile((x, y - 1)) == SCAFFOLDING
    }
}

