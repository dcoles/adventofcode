use intcode::emulator::{IntcodeEmulator, Program, Exception, Word};
use std::convert::{TryFrom, TryInto};
use std::collections::HashMap;
use std::{io, ops, thread};
use std::io::Write;
use std::time::Duration;

type Map = HashMap<Pos, char>;

const UNKNOWN: char = ' ';
const WALL: char = '#';
const OPEN: char = '.';
const TARGET: char = '*';
const DROID: char = '@';
const BEL: &str = "";  // Set to "\x07" if you feel your life lacks excitement
const ORIGIN: Pos = Pos::new(0, 0);
const WIDTH: u32 = 80;
const HEIGHT: u32 = 60;
const SCREEN_OFFSET_X: u32 = 40;
const SCREEN_OFFSET_Y: u32 = 30;
const FPS: u64 = 12;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    let mut map: HashMap<Pos, char> = HashMap::new();
    map.insert(ORIGIN, OPEN);  // We start in the open

    let mut droid = Droid::new(&program, ORIGIN);
    let mut planner = Planner::new();

    print!("\x1B[2J");  // Clear screen
    print!("\x1B[?25l");  // Hide cursor
    print!("\x1B[8;{};{}t", HEIGHT, WIDTH);  // Resize console

    // Find the leak
    let mut target = ORIGIN;
    for t in 0.. {
        draw( *map.get(&droid.pos).unwrap(), droid.pos);
        let command = planner.plan(&map, droid.pos);
        target = droid.pos + command.direction();
        match droid.input(command) {
            Status::Wall => {
                map.insert(target, WALL);
                draw( WALL, target);
            },
            Status::Moved => {
                map.insert(target, OPEN);
                draw( OPEN, target);
            },
            Status::MovedAndFoundTarget => {
                map.insert(target, TARGET);
                draw( TARGET, target);
                print_status(&format!("Steps {}", t));
                thread::sleep(Duration::from_secs(10));
                break;
            },
        }
        draw( DROID, droid.pos);
        io::stdout().flush().expect("Failed to flush stdout");
        thread::sleep(Duration::from_millis(1000 / FPS));
    }

    let distance = planner.distance.get(&target).unwrap();
    print_status(&format!("Found leak at {:?} (distance: {})", target, distance));
}

fn draw(tile: char, pos: Pos) {
    let color = match tile {
        TARGET => "\x1B[31m",  // Red
        DROID => "\x1B[32m",  // Green
        OPEN => "\x1B[34m",  // Blue
        _ => "",
    };
    print!("\x1B[{};{}H{}{}\x1B[m", SCREEN_OFFSET_Y as i32 - pos.y + 1, SCREEN_OFFSET_X as i32 + pos.x + 1, color, tile);
}

fn print_status(message: &str) {
    // Move to status line (second last line), clear it and print message
    println!("\x1B[{}H\x1B[K{}", HEIGHT - 1, message);
}

struct Planner {
    visits: HashMap<Pos, u32>,
    distance: HashMap<Pos, u32>,
}

impl Planner {
    fn new() -> Self {
        let mut distance = HashMap::new();
        distance.insert(ORIGIN, 0);
        Planner { visits: HashMap::new(), distance }
    }

    fn plan(&mut self, map: &Map, current_pos: Pos) -> MovementCommand {
        use MovementCommand::*;

        // Find valid tile choices (e.g. not a wall)
        let mut choices: Vec<_> = [North, South, East, West].iter()
            .map(|&command| {
                let target = current_pos + command.direction();

                (command, target, *map.get(&target).unwrap_or(&UNKNOWN))
            })
            .filter(|&(_, _, tile)| tile != WALL)
            .collect();

        // Sort by attempted visits to the tile
        // (We could also take into account the distance from origin, but this does pretty well)
        choices.sort_by_key(|&(_, pos, _)| self.visits.get(&pos).unwrap_or(&0));

        // Pick the first one
        let &(command, target, _) = choices.first().unwrap();
        *self.visits.entry(target).or_default() += 1;

        // Update our distance calculation
        let &cur_distance = self.distance.get(&current_pos).unwrap();
        let &target_distance = self.distance.get(&target).unwrap_or(&std::u32::MAX);
        self.distance.insert(target, target_distance.min(cur_distance + 1));

        command
    }
}

struct Droid {
    pos: Pos,
    cpu: IntcodeEmulator,
}

impl Droid {
    fn new(program: &Program, pos: Pos) -> Droid {
        let mut cpu = IntcodeEmulator::new();
        cpu.load_program(program);

        Droid { pos, cpu }
    }

    fn input(&mut self, movement: MovementCommand) -> Status {
        loop {
            match self.cpu.run() {
                Exception::Halt => panic!("Program halted"),
                Exception::Input => {
                    self.cpu.add_input(movement.into());
                },
                Exception::Output(word) => {
                    let status: Status = word.try_into().expect("Unknown status");
                    match status {
                        Status::Wall => {
                            print_status(&format!("{}* Bonk! *", BEL));
                        },
                        Status::Moved => {
                            print_status("");
                            self.moved(movement);
                        },
                        Status::MovedAndFoundTarget => {
                            print_status("");
                            self.moved(movement);
                        },
                    }
                    return status;
                },
                exception => panic!("Unhandled exception: {}", exception),
            }
        }
    }

    fn moved(&mut self, command: MovementCommand) {
        self.pos += command.direction();
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    const fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

impl ops::Add<Movement> for Pos {
    type Output = Pos;

    fn add(self, rhs: Movement) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::AddAssign<Movement> for Pos {
    fn add_assign(&mut self, rhs: Movement) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Movement {
    x: i32,
    y: i32,
}

impl Movement {
    const fn new(x: i32, y: i32) -> Self {
        Movement { x, y }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MovementCommand {
    North,
    South,
    West,
    East,
}

impl MovementCommand {
    fn direction(self) -> Movement {
        use MovementCommand::*;
        match self {
            North => Movement::new(0, 1),
            South => Movement::new(0, -1),
            West => Movement::new(-1, 0),
            East => Movement::new(1, 0),
        }
    }
}

impl Into<Word> for MovementCommand {
    fn into(self) -> Word {
        use MovementCommand::*;
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Status {
    Wall,
    Moved,
    MovedAndFoundTarget,
}

impl TryFrom<Word> for Status {
    type Error = ();

    fn try_from(word: Word) -> Result<Self, Self::Error> {
        use Status::*;
        match word {
            0 => Ok(Wall),
            1 => Ok(Moved),
            2 => Ok(MovedAndFoundTarget),
            _ => Err(()),
        }
    }

}
