use std::path::Path;
use std::fs;
use std::collections::HashSet;

const ORIGIN: Pos = (0, 0);

fn main() {
    let input = read_input("input.txt");
    for wire in &input {
        println!("{}", wire.iter().map(Move::to_string).collect::<Vec<_>>().join(","));
    }

    // Part 1
    let mut grid: HashSet<Pos> = HashSet::new();
    let mut intersections: Vec<Pos> = Vec::new();
    for wire in &input {
        // Check for any intersections
        let mut positions: Vec<Pos> = Vec::new();
        let mut pos = ORIGIN;
        for m in wire {
            let line_positions = m.new_pos(pos);
            for &pos in &line_positions {
                if grid.contains(&pos) {
                    intersections.push(pos);
                };
                positions.push(pos);
            }
            pos = *line_positions.last().unwrap();
        }

        // Add all the positions
        for pos in positions {
            grid.insert(pos);
        }
    }

    intersections.sort_by_key(|&p| distance(ORIGIN, p));
    let closest = intersections[0];
    println!("Part 1: {:?} (distance: {})", closest, distance(ORIGIN, closest));

    // Part 2
    let mut signal_delays = Vec::new();
    for &intersection in &intersections {
        let mut steps = 0;
        for wire in &input {
            let mut pos = ORIGIN;
            'outer: for m in wire {
                let line_positions = m.new_pos(pos);
                for &pos in &line_positions {
                    steps += 1;
                    if pos == intersection {
                        break 'outer;
                    }
                }
                pos = *line_positions.last().unwrap();
            }
        }
        signal_delays.push(steps);
    }
    signal_delays.sort();
    println!("Part 2: Fewest combined steps: {}", signal_delays[0]);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Vec<Move>> {
    let input = fs::read_to_string(path).expect("Failed to read input");
    let mut result = Vec::new();
    for line in input.lines() {
        let moves: Vec<Move> = line.trim().split(",").map(Move::from_str).collect();
        result.push(moves);
    }

    result
}

type Pos = (i32, i32);

fn distance(p1: Pos, p2: Pos) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Move {
    fn from_str(string: &str) -> Move {
        let val = string[1..].parse::<u32>().expect("Failed to parse move");
        match &string[..1] {
            "U" => Move::Up(val),
            "D" => Move::Down(val),
            "L" => Move::Left(val),
            "R" => Move::Right(val),
            c => panic!("Unknown direction {}", c),
        }
    }

    fn to_string(&self) -> String {
        match self {
            &Move::Up(v) => format!("U{}", v),
            &Move::Down(v) => format!("D{}", v),
            &Move::Left(v) => format!("L{}", v),
            &Move::Right(v) => format!("R{}", v),
        }
    }

    fn new_pos(&self, start: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        let (dir, dist) = match self {
            &Move::Up(v) => ((0, 1), v),
            &Move::Down(v) => ((0, -1), v),
            &Move::Left(v) => ((-1, 0), v),
            &Move::Right(v) => ((1, 0), v),
        };

        let mut pos = start;
        for _ in 0..dist {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            result.push(pos);
        }

        result
    }
}
