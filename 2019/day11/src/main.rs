use intcode::emulator;
use intcode::emulator::{Word, Program};
use std::collections::{VecDeque, HashSet};
use std::fmt;

const BLACK: Word = 0;
const WHITE: Word = 1;

const LEFT: Word = 0;
const RIGHT: Word = 1;

fn main() {
    let program = emulator::Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    println!("Part 1");
    println!("══════");
    let mut map = Map::new(80, 90);
    run(&program, &mut map, Pos::new(45, 75));
    println!("Number of panels painted at least once: {}", map.painted.len());
    println!();

    // Part 2
    println!("Part 2");
    println!("══════");
    let start = Pos::new(1, 1);
    let mut map = Map::new(80, 8);
    map.paint(start, WHITE);
    run(&program, &mut map, start);
}

fn run(program: &Program, map: &mut Map, pos: Pos) {
    let mut cpu = emulator::IntcodeEmulator::new();
    cpu.load_program(&program);

    let mut robot = Robot::new(pos);

    loop {
        match cpu.run() {
            emulator::Exception::Halt => break,
            emulator::Exception::Input => {
                cpu.add_input(robot.camera(map));
            },
            emulator::Exception::Output(word) => {
                robot.handle_input(word, map);
            },
            exception => panic!("Unhandled exception: {}", exception),
        }
    }

    map.draw();
}

struct Map {
    width: usize,
    height: usize,
    grid: Vec<Word>,
    painted: HashSet<Pos>,
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        Map { width, height, grid: vec![BLACK; width * height], painted: HashSet::new() }
    }

    fn at(&self, pos: Pos) -> Word {
        assert!(pos.x < self.width);
        assert!(pos.y < self.height);
        self.grid[pos.y * self.width + pos.x]
    }

    fn paint(&mut self, pos: Pos, color: Word) {
        assert!(pos.x < self.width);
        assert!(pos.y < self.height);
        self.grid[pos.y * self.width + pos.x] = color;
        self.painted.insert(pos);
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
            let pos = Pos::new(x, y);
                match self.at(pos) {
                    BLACK => print!("░"),
                    WHITE => print!("█"),
                    _ => print!("?"),
                }
            }
            println!();
        }
    }
}

struct Robot {
    pos: Pos,
    direction: Direction,
    input_buffer: VecDeque<Word>,
}

impl Robot {
    fn new(pos: Pos) -> Robot {
        Robot { pos, direction: Direction::Up, input_buffer: VecDeque::new() }
    }

    fn camera(&self, map: &Map) -> Word {
        map.at(self.pos)
    }

    fn handle_input(&mut self, input: Word, map: &mut Map) {
        self.input_buffer.push_back(input);
        if self.input_buffer.len() < 2 {
            // Need more input
            return;
        }

        let colour = self.input_buffer.pop_front().unwrap();
        let direction = self.input_buffer.pop_front().unwrap();

        // Paint
        map.paint(self.pos, colour);

        // Turn
        match direction {
            LEFT => self.direction = self.direction.left(),
            RIGHT => self.direction = self.direction.right(),
            _ => panic!("Unknown direction: {}", direction),
        }

        // Move
        self.pos.move_direction(self.direction);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(self) -> Direction {
        use Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn right(self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    fn move_direction(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
