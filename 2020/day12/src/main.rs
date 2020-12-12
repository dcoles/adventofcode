use std::path::Path;
use std::fs;

type Input = Vec<Action>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {:?}", part1(&input).distance());
    println!("Part 2: {:?}", part2(&input).distance());
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| Action::from_str(line).expect("Failed to parse instruction"))
        .collect()
}

fn part1(input: &Input) -> Ship {
    let mut ship = Ship::new();

    for &action in input {
        ship.do_action(action);
    }

    ship
}

fn part2(input: &Input) -> Ship {
    let mut ship = Ship::new();

    for &action in input {
        ship.do_action_waypoint(action);
    }

    ship
}

#[derive(Copy, Clone, Debug)]
enum Action {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl Action {
    fn from_str(s: &str) -> Option<Self> {
        let value = s[1..].parse().ok()?;

        use Action::*;
        match &s[..1] {
            "N" => Some(North(value)),
            "S" => Some(South(value)),
            "E" => Some(East(value)),
            "W" => Some(West(value)),
            "L" => Some(Left(value)),
            "R" => Some(Right(value)),
            "F" => Some(Forward(value)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Ship {
    position: (i32, i32),
    waypoint: (i32, i32),
    direction: u32,
}

impl Ship {
    fn new() -> Self {
        Ship { position: (0, 0), waypoint: (10, 1), direction: 0 }
    }

    fn do_action(&mut self, action: Action) {
        use Action::*;

        match action {
            North(val) => self.position.1 += val as i32,
            South(val) => self.position.1 -= val as i32,
            East(val) => self.position.0 += val as i32,
            West(val) => self.position.0 -= val as i32,
            Left(val) => self.direction = (self.direction + val) % 360,
            Right(val) => self.direction = (self.direction + 360 - val) % 360,
            Forward(val) => {
                match self.direction {
                    0 => self.position.0 += val as i32,
                    90 => self.position.1 += val as i32,
                    180 => self.position.0 -= val as i32,
                    270 => self.position.1 -= val as i32,
                    other => panic!("Unhandled angle: {}", other),
                }
            }
        }
    }

    fn do_action_waypoint(&mut self, action: Action) {
        use Action::*;

        match action {
            North(val) => self.waypoint.1 += val as i32,
            South(val) => self.waypoint.1 -= val as i32,
            East(val) => self.waypoint.0 += val as i32,
            West(val) => self.waypoint.0 -= val as i32,
            Left(val) => {
                match val {
                    0 => (),
                    90 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
                    180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
                    270 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
                    other => panic!("Unhandled angle: {}", other),
                }
            },
            Right(val) => {
                match val {
                    0 => (),
                    90 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
                    180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
                    270 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
                    other => panic!("Unhandled angle: {}", other),
                }
            },
            Forward(val) => self.position = (
                self.position.0 + val as i32 * self.waypoint.0,
                self.position.1 + val as i32 * self.waypoint.1,
            ),
        }
    }

    fn distance(&self) -> u32 {
        (self.position.0.abs() + self.position.1.abs()) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input).distance(), 25);
    }

    #[test]
    fn test_part2() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input).distance(), 286);
    }
}

