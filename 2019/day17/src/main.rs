use intcode::emulator::{Program, IntcodeEmulator, Word, Context};
use std::{io, thread};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

type Pos = (usize, usize);

const SPACE: char = '.';
const SCAFFOLDING: char = '#';
const FPS: u64 = 24;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let view = get_view(&program);
    let map = Map::from_view(&view);
    let intersections = map.find_intersections();
    let calibration: usize = alignment_parameters(&intersections).into_iter().sum();

    println!("Part 1: Sum of alignment parameters: {}", calibration);

    // Part 2
    println!("Part 2:");
    let mut robot = Robot::new(&program);
    robot.set_active(true);
    robot.run();

}

fn alignment_parameters(intersections: &[Pos]) -> Vec<usize> {
    intersections.into_iter().map(|&(x, y)| x * y).collect()
}

fn get_view(program: &Program) -> String {
    let output = Rc::new(RefCell::new(String::new()));
    {
        let output = Rc::clone(&output);
        let input_handler = Box::new(|_: &mut Context| Err(io::Error::new(io::ErrorKind::BrokenPipe, "No input")));
        let output_handler = Box::new(move |_: &mut Context, word| {
            let c: char = (word as u8).into();
            output.borrow_mut().push(c);

            Ok(())
        });
        let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
        cpu.load_program(&program);
        cpu.run().expect("Failed to run program");
    }

    Rc::try_unwrap(output).unwrap().into_inner()
}

struct Robot {
    cpu: IntcodeEmulator,
    program: Program,
}

impl Robot {
    fn new(program: &Program) -> Self {
        // Input handler for Robot program
        let mut p: VecDeque<_> = [
            //        1         2
            //2345678901234567890
            "A,B,B,A,C,B,C,C,B,A",  // Main
            "R,10,R,8,L,10,L,10",  // A
            "R,8,L,6,L,6",  // B
            "L,10,R,10,L,6",  // C
            "y",  // enable continuous video feed
            "",  // EOF
        ].join("\n").chars().collect();
        let input_handler = Box::new(move |_: &mut Context| {
            if let Some(c) = p.pop_front() {
                print!("{}", c);  // Echo input
                Ok(c as Word)
            } else {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "No more input"))
            }
        });

        // Output handler
        let mut line = String::new();
        let mut last_line = String::new();
        let output_handler = Box::new(move |_: &mut Context, word| {
            // Check if in ASCII range
            if (0..=127).contains(&word) {
                let c: char = (word as u8).into();
                line.push(c);

                if c == '\n' {
                    // Detect double newline
                    if &last_line == "\n" {
                        print!("\x1B[H");  // Move to home

                        // Status?
                        if line.chars().next().unwrap_or('\0').is_alphanumeric() {
                            print!("\x1B[2J");  // Clear screen
                        }

                        thread::sleep(Duration::from_millis(1000 / FPS));
                    }

                    print!("{}", line);

                    last_line = line.clone();
                    line.clear();
                }
            } else {
                print!("\x1B[1000H\x1B[K");  // Move to end and clear line
                println!("STATUS: {}", word);
            }

            Ok(())
        });

        let cpu = IntcodeEmulator::new(input_handler, output_handler);

        Robot { cpu, program: program.clone() }
    }

    fn set_active(&mut self, active: bool) {
        self.program[0] = if active { 2 } else { 1 };
    }

    fn run(&mut self) {
        print!("\x1B[2J");  // Clear screen
        print!("\x1B[?25l");  // Hide cursor
        print!("\x1B[H");  // Move to home

        self.cpu.load_program(&self.program);

        match self.cpu.run() {
            Ok(()) => (),
            Err(exception) => panic!("Unhandled exception: {}", exception),
        }

        println!("END");
        print!("\x1Bc");  // Reset the terminal
    }
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

