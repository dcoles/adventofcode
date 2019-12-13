use intcode::emulator::{Program, IntcodeEmulator, Exception, Word};
use std::collections::{VecDeque};
use std::{fmt, cmp, time, thread, env};

const WIDTH: usize = 44;
const HEIGHT: usize = 20;
const FPS: u64 = 12;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let mut arcade = ArcadeCabinet::new(&program);
    arcade.run();
    let n_block = arcade.grid.iter().filter(|&&v| v == Tile::Block).count();
    println!("Part 1: Tiles on screen: {}", n_block);

    // Part 2
    println!("Part 2:");
    let mut arcade = ArcadeCabinet::new(&program);
    arcade.freeplay();
    if env::args().any(|a| a.trim() == "--turbo") {
        arcade.turbo();
    } else {
        println!();
        println!(" [ You may wish to run this with --turbo ]");
        println!();
        thread::sleep(time::Duration::from_secs(4));
    }
    arcade.run();

    // Be nice and reset the user's terminal
    print!("\x1Bc");
    println!("Final score: {}", arcade.score);
}

struct ArcadeCabinet {
    cpu: IntcodeEmulator,
    fps: u64,
    score: Word,
    grid: [Tile; WIDTH * HEIGHT],
    output_queue: VecDeque<Word>,
    ball_pos: (Word, Word),
    paddle_pos: (Word, Word),
}

impl ArcadeCabinet {
    fn new(program: &Program) -> ArcadeCabinet {
        let mut cpu = IntcodeEmulator::new();
        cpu.load_program(program);

        ArcadeCabinet { cpu, fps: FPS, score: 0, grid: [Tile::Empty; WIDTH * HEIGHT], output_queue: VecDeque::new(), ball_pos: (0, 0), paddle_pos: (0, 0) }
    }

    fn freeplay(&mut self) {
        self.cpu.mem_mut()[0] = 2;
    }

    fn turbo(&mut self) {
        self.fps = 9999;
    }

    fn display(&self) {
        print!("\x1B[8;{};{}t", HEIGHT + 2, WIDTH);  // Resize console
        print!("\x1B[H");  // Move cursor to HOME
        print!("\x1B[2K");  // Clear score line
        print!("\x1B]0;SCORE: {:08}\x07", self.score);  // Show score in console title
        println!("SCORE: {:08}", self.score);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", self.at(x, y));
            }
            println!();
        }
    }

    fn at(&self, x: usize, y: usize) -> Tile {
        assert!(x < WIDTH, "x out of range: {} exceeds WIDTH", x);
        assert!(y < HEIGHT, "y out of range: {} exceeds HEIGHT", x);
        self.grid[y * WIDTH + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        assert!(x < WIDTH, "x out of range: {} exceeds WIDTH", x);
        assert!(y < HEIGHT, "y out of range: {} exceeds HEIGHT", x);
        &mut self.grid[y * WIDTH + x]
    }

    fn run(&mut self) {
        print!("\x1B[2J");  // Clear screen
        print!("\x1B[?25l");  // Hide cursor

        loop {
            match self.cpu.run() {
                Exception::Halt => {
                    self.display();
                    break
                },
                Exception::Input => self.handle_input(),
                Exception::Output(word) => self.handle_output(word),
                exception => {
                    self.cpu.dump_registers();
                    self.cpu.print_disassembled();
                    self.cpu.dump_memory();
                    panic!("Unhandled exception: {}", exception);
                },
            }
        }
    }

    fn handle_input(&mut self) {
        let input = match self.ball_pos.0.cmp(&self.paddle_pos.0) {
            cmp::Ordering::Less => -1,  // Left
            cmp::Ordering::Greater => 1,  // Right
            cmp::Ordering::Equal => 0,  // Hold
        };

        self.cpu.add_input(input);
    }

    fn handle_output(&mut self, word: Word) {
        self.output_queue.push_back(word);
        if self.output_queue.len() < 3 {
            // Need more input
            return;
        }

        let x = self.output_queue.pop_front().unwrap();
        let y = self.output_queue.pop_front().unwrap();
        let tile_id = self.output_queue.pop_front().unwrap();

        if x == -1 && y == 0 {
            self.score = tile_id;
        } else {
            let tile = tile_id.into();
            *self.at_mut(x as usize, y as usize) = tile;

            match tile {
                Tile::Ball => {
                    self.ball_pos = (x, y);
                    self.display();
                    if self.fps < 1000 {
                        thread::sleep(time::Duration::from_micros(1_000_000 / self.fps));
                    }
                },
                Tile::Paddle => self.paddle_pos = (x, y),
                _ => (),
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn to_char(self) -> char {
        use Tile::*;
        match self {
            Empty => ' ',
            Wall => '█',
            Block => '□',
            Paddle => '═',
            Ball => '●',
        }
    }
}

impl From<Word> for Tile {
    fn from(word: Word) -> Self {
        use Tile::*;
        match word {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!("Unknown tile {}", word),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
