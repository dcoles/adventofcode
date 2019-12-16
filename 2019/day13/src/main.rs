use intcode::emulator::{Program, IntcodeEmulator, Exception, Word};
use std::collections::{VecDeque};
use std::{fmt, cmp, time, thread, env, io};
use std::io::Write;
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: usize = 44;
const HEIGHT: usize = 20;
const FPS: u64 = 12;
const TURBO_FPS: u64 = 9999;

// Use full-width characters for the score (not all terminals support)
const FULLWIDTH: bool = true;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let mut arcade = ArcadeCabinet::new();
    arcade.run(&program);
    println!("Part 1: Tiles on screen: {}", arcade.state.borrow().n_blocks);

    // Part 2
    println!("Part 2:");
    let mut arcade = ArcadeCabinet::new();
    arcade.freeplay(true);
    if env::args().any(|a| a.trim() == "--turbo") {
        arcade.turbo(true);
    } else {
        println!();
        println!(" [ You may wish to run this with --turbo ]");
        println!();
        thread::sleep(time::Duration::from_secs(4));
    }
    arcade.run(&program);

    // Be nice and reset the user's terminal
    print!("\x1Bc");
    println!("Final score: {}", arcade.state.borrow().score);
}

struct ArcadeCabinet {
    freeplay: bool,
    cpu: IntcodeEmulator,
    state: Rc<RefCell<GameState>>
}

impl ArcadeCabinet {
    fn new() -> ArcadeCabinet {
        ArcadeCabinet {
            freeplay: false,
            cpu: IntcodeEmulator::default(),
            state: Rc::new(RefCell::new(GameState::new())),
        }
    }

    fn freeplay(&mut self, freeplay: bool) {
        self.freeplay = freeplay;
    }

    fn turbo(&mut self, turbo: bool) {
        self.state.borrow_mut().fps = if turbo { TURBO_FPS } else { FPS };
    }

    fn run(&mut self, program: &Program) {
        print!("\x1B[8;{};{}t", HEIGHT + 2, WIDTH);  // Resize console
        print!("\x1B[2J");  // Clear screen
        print!("\x1B[?25l");  // Hide cursor

        self.cpu.load_program(program);
        let state = Rc::clone(&self.state);
        self.cpu.set_input_handler(Box::new(move || state.borrow_mut().handle_input()));
        let state = Rc::clone(&self.state);
        self.cpu.set_output_handler(Box::new(move |word| state.borrow_mut().handle_output(word)));

        if self.freeplay {
            self.cpu.mem_mut()[0] = 2;
        }

        loop {
            match self.cpu.run() {
                Exception::Halt => break,
                exception => {
                    self.cpu.dump_registers();
                    self.cpu.print_disassembled();
                    self.cpu.dump_memory();
                    panic!("Unhandled exception: {}", exception);
                },
            }
        }

        // Make sure previous line is closed
        println!();
    }

    fn draw_score(score: Word) {
        print!("\x1B[H");  // Move cursor to HOME
        print!("\x1B[2K");  // Clear score line
        print!("\x1B]0;SCORE: {:08}\x07", score);  // Show score in console title
        if FULLWIDTH {
            let chars: String = format!("{:08}", score).chars().map(|c| match c {
                '0' => '０',
                '1' => '１',
                '2' => '２',
                '3' => '３',
                '4' => '４',
                '5' => '５',
                '6' => '６',
                '7' => '７',
                '8' => '８',
                '9' => '９',
                _ => '?',
            }).collect();
            println!("SCORE: {}", chars);
        } else {
            println!("SCORE: {:08}", score);
        }
    }

    fn draw_tile(tile: Tile, pos: (Word, Word)) {
        let color = match tile {
           Tile::Block => match pos.1 {
                2..=3 => "\x1B[35m",  // Magenta
                4..=5 => "\x1B[31m",  // Red
                6..=7 => "\x1B[33m",  // Yellow
                8..=9 => "\x1B[32m",  // Green
                10..=11 => "\x1B[34m",  // Blue
                12..=13 => "\x1B[36m",  // Cyan
                _ => "",
            },
            Tile::Ball | Tile::Paddle => "\x1B[35;1m",  // Magenta
            _ => "",
        };
        print!("\x1B[{};{}H{}{}\x1B[m", pos.1 + 2, pos.0 + 1, color, tile);
    }
}

struct GameState {
    score: Word,
    output_queue: VecDeque<Word>,
    ball_pos: (Word, Word),
    paddle_pos: (Word, Word),
    n_blocks: u32,
    fps: u64,
}

impl GameState {
    fn new() -> Self {
        GameState {
            score: 0,
            output_queue: VecDeque::new(),
            ball_pos: (0, 0),
            paddle_pos: (0, 0),
            n_blocks: 0,
            fps: FPS,
        }
    }

    fn handle_input(&mut self) -> io::Result<Word> {
        let input = match self.ball_pos.0.cmp(&self.paddle_pos.0) {
            cmp::Ordering::Less => -1,  // Left
            cmp::Ordering::Greater => 1,  // Right
            cmp::Ordering::Equal => 0,  // Hold
        };

        Ok(input)
    }

    fn handle_output(&mut self, word: Word) -> io::Result<()> {
        self.output_queue.push_back(word);
        if self.output_queue.len() < 3 {
            // Need more input
            return Ok(());
        }

        let x = self.output_queue.pop_front().unwrap();
        let y = self.output_queue.pop_front().unwrap();
        let tile_id = self.output_queue.pop_front().unwrap();

        if x == -1 && y == 0 {
            self.score = tile_id;
            ArcadeCabinet::draw_score(self.score);
        } else {
            let tile = tile_id.into();

            match tile {
                Tile::Ball => {
                    self.ball_pos = (x, y);
                    ArcadeCabinet::draw_tile(tile, (x, y));

                    // Update of the ball position is used to flush display output
                    // and rate-limit the game's FPS
                    io::stdout().flush().expect("Failed to flush stdout");
                    if self.fps < 1000 {
                        thread::sleep(time::Duration::from_micros(1_000_000 / self.fps));
                    }
                },
                Tile::Paddle => {
                    self.paddle_pos = (x, y);
                    ArcadeCabinet::draw_tile(tile, (x, y));
                },
                Tile::Block => {
                    self.n_blocks += 1;
                    ArcadeCabinet::draw_tile(tile, (x, y));
                },
                _ => ArcadeCabinet::draw_tile(tile, (x, y)),
            }
        }

        Ok(())
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
            Block => '■',
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
