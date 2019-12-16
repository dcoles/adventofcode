use intcode::emulator::{IntcodeEmulator, Program, Exception, Word};
use std::convert::{TryFrom, TryInto};
use std::collections::{HashMap, HashSet};
use std::{ops, thread, env};
use std::time::Duration;

const UNKNOWN: char = ' ';
const WALL: char = '#';
const OPEN: char = '.';
const TARGET: char = '*';
const DROID: char = '@';
const DEAD: char = '+';
const OXYGEN: char = 'O';
const DEAD_END: u32 = std::u32::MAX;  // Cost of tiles leading to a dead-end
const ORIGIN: Pos = Pos::new(0, 0);
const WIDTH: u32 = 80;
const HEIGHT: u32 = 60;
const SCREEN_OFFSET_X: u32 = 40;
const SCREEN_OFFSET_Y: u32 = 30;
const FPS: u64 = 12;

// Helper macro for printing the status line
macro_rules! print_status {
    ( $($e:expr),* ) => { print_status(&format!($($e),*)) };
}

fn draw(tile: char, pos: Pos) {
    let color = match tile {
        TARGET => "\x1B[31m",  // Red
        DROID => "\x1B[32m",  // Green
        OPEN => "\x1B[34m",  // Blue
        DEAD => "\x1B[33m",  // Yellow
        OXYGEN => "\x1B[36m",  // Cyan
        _ => "",
    };
    print!("\x1B[{};{}H{}{}\x1B[m", SCREEN_OFFSET_Y as i32 - pos.y + 1, SCREEN_OFFSET_X as i32 + pos.x + 1, color, tile);
}

fn print_status(message: &str) {
    // Move to status line (second last line), clear it and print message
    println!("\x1B[{}H\x1B[K{}", HEIGHT - 1, message);
}

fn main() {
    // The --turbo flag skips all animation delays
    let turbo = env::args().skip(1).any(|arg| &arg == "--turbo");

    // Part 1
    let program = Program::from_file("input.txt").expect("Failed to read input");

    let mut droid = Droid::new(&program, ORIGIN);
    let mut planner = Planner::new();

    print!("\x1B[2J");  // Clear screen
    print!("\x1B[?25l");  // Hide cursor
    print!("\x1B[8;{};{}t", HEIGHT, WIDTH);  // Resize console

    // Part 1: Find the broken O₂ system
    let mut o2_system = None;
    while let Some(command) = planner.plan(droid.pos) {
        // Clear the droid from the map
        draw( planner.get_tile(droid.pos), droid.pos);

        let target = droid.pos + command.direction();
        match droid.input(command) {
            Status::Wall => {
                planner.update_map(target, WALL);
                draw( WALL, target);
            },
            Status::Moved => {
                planner.update_map(target, OPEN);
                draw( OPEN, target);
            },
            Status::MovedAndFoundTarget => {
                o2_system = Some(target);
                planner.update_map(target, TARGET);
                draw( TARGET, target);
            },
        }

        draw( DROID, droid.pos);
        print_status!("{:?} (distance: {})", droid.pos, planner.distance_from_origin(droid.pos));

        if !turbo {
            //thread::sleep(Duration::from_millis(1000 / FPS));
        }
    }

    // Found the leak!
    let o2_system = o2_system.expect("Failed to find the O₂ system");
    let distance = planner.distance_from_origin(o2_system);
    draw(planner.get_tile(droid.pos), droid.pos); // Hide the droid, it's work is done

    // Part 2: Fill the map with O₂
    let mut oxygenated: HashSet<Pos> = [o2_system].iter().copied().collect();
    let mut edge: HashSet<Pos> = planner.adjacent(o2_system).into_iter().collect();
    let mut t = 0;
    while !edge.is_empty() {
        let mut new_edge = HashSet::new();
        for pos in edge.drain() {
            oxygenated.insert(pos);
            draw(OXYGEN, pos);
            new_edge.extend(planner.adjacent(pos).into_iter().filter(|&pos| !oxygenated.contains(&pos)));
        }

        edge = new_edge;
        t += 1;

        print_status!("t: {} min", t);

        if !turbo {
            thread::sleep(Duration::from_millis(1000 / FPS));
        }
    }

    print!("\x1Bc");  // Reset the terminal
    println!("Part 1: Found the O₂ system at {:?} (distance: {})", o2_system, distance);
    println!("Part 2: Time required to fill map with O₂: {} min", t);
}

struct Planner {
    map: HashMap<Pos, char>,
    cost: HashMap<Pos, u32>,
    distance: HashMap<Pos, u32>,
    unexplored: HashSet<Pos>,
}

impl Planner {
    fn new() -> Self {
        let map = [(ORIGIN, OPEN)].iter().copied().collect();  // We start in the open
        let cost = [(ORIGIN, 1)].iter().copied().collect();  // Already visited origin
        let distance = [(ORIGIN, 0)].iter().copied().collect();

        Planner { map, cost, distance, unexplored: HashSet::new() }
    }

    /// Plan the next move
    fn plan(&mut self, current_pos: Pos) -> Option<MovementCommand> {
        use MovementCommand::*;

        // Find valid tile choices (e.g. not a wall)
        let mut choices: Vec<(MovementCommand, Pos, char)> = [North, South, East, West].iter()
            .map(|&command| {
                let target = current_pos + command.direction();

                (command, target, self.get_tile(target))
            })
            // Don't try tiles or dead-end
            .filter(|&(_, pos, tile)| tile != WALL && !self.is_dead_end(pos))
            .collect();

        if choices.len() == 1 {
            // This path leads to a dead-end
            if self.get_tile(current_pos) == OPEN {
                self.update_map(current_pos, DEAD);
            }
            self.cost.insert(current_pos, DEAD_END);
        }

        // Keep track of places we seen, but haven't explored yet
        let unexplored_choices: Vec<_> = choices.iter()
            .filter(|&&(_, pos, _)| self.get_tile(pos) == UNKNOWN)
            .map(|&(_, pos, _)| pos).collect();
        self.unexplored.extend(&unexplored_choices);

        if self.unexplored.is_empty() {
            // Stop when there's no where left to explore
            return None;
        }

        // Sort by attempted visits to the tile
        // (We could also take into account the distance from origin, but this does pretty well)
        choices.sort_by_key(|&(_, pos, _)| self.cost(pos));

        // Pick the first one
        let &(command, target, _) = choices.first().unwrap();
        *self.cost.entry(target).or_default() += 1;
        self.unexplored.remove(&target);

        // Update our distance calculation
        let distance = self.distance_from_origin(current_pos) + 1;
        let target_distance = self.distance_from_origin(target);
        self.distance.insert(target, target_distance.min(distance));

        Some(command)
    }

    /// What do we know about this tile
    fn get_tile(&self, pos: Pos) -> char {
        self.map.get(&pos).copied().unwrap_or(UNKNOWN)
    }

    /// Update the map
    fn update_map(&mut self, pos: Pos, tile: char) {
        self.map.insert(pos, tile);
    }

    /// Distance postion is from origin
    fn distance_from_origin(&self, pos: Pos) -> u32 {
        self.distance.get(&pos).copied().unwrap_or(std::u32::MAX)
    }

    /// Cost for movement to this position
    fn cost(&self, pos: Pos) -> u32 {
        self.cost.get(&pos).copied().unwrap_or(0)
    }

    /// Is this position considered a dead-end
    fn is_dead_end(&self, pos: Pos) -> bool {
        self.cost.get(&pos).copied().unwrap_or(0) == DEAD_END
    }

    /// Adjacent tiles to this position
    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        use MovementCommand::*;
        [North, South, East, West].iter()
            .copied()
            .map(|c| pos + c.direction())
            .filter(|&p| self.get_tile(p) != WALL)
            .collect()
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
                        Status::Moved => {
                            self.moved(movement);
                        },
                        Status::MovedAndFoundTarget => {
                            self.moved(movement);
                        },
                        _ => (),
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

