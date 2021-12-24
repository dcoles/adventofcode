//! Advent of Code 2021: Day 24
//! https://adventofcode.com/2021/day/24

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Program::from_file("day24/input.txt").expect("failed to read input");

    // I did this by hand-decoding the instructions constraints:
    //
    //   input_14 = input_1 + 8  - 11
    //   input_13 = input_2 + 16 - 13
    //   input_4  = input_3 + 4  - 11
    //   input_12 = input_5 + 13 - 13
    //   input_11 = input_6 + 5  - 11
    //   input_8  = input_7 + 0  - 5
    //   input_10 = input_9 + 7  + 0
    //
    // (Where input must be between 1 to 9)
    //
    // Therefore:
    //
    //   1: 4-9 ~ 14: 1-6
    //   2: 1-6 ~ 13: 4-9
    //   3: 8-9 ~  4: 1-2
    //   5: 1-9 ~ 12: 1-9
    //   6: 7-9 ~ 11: 1-3
    //   7: 6-9 ~  8: 1-4
    //   9: 1-2 ~ 10: 8-9

    // Part 1
    let part1 = [9, 6, 9, 2, 9, 9, 9, 4, 2, 9, 3, 9, 9, 6];
    println!("Part 1: {:?} is {}", part1, if run(&input, part1) == 0 { "valid" } else { "invalid" });

    // Part 2
    let part2 = [4, 1, 8, 1, 1, 7, 6, 1, 1, 8, 1, 1, 4, 1];
    println!("Part 2: {:?} is {}", part2, if run(&input, part2) == 0 { "valid" } else { "invalid" });
}

fn run(input: &Program, stream: impl IntoIterator<Item=i64>) -> i64 {
    let mut stream = stream.into_iter();
    let mut vars: HashMap<char, i64> = HashMap::new();
    for op in input.instructions.iter() {
        match op {
            Op::Input(a) => {
                let a = a.var().unwrap();
                vars.insert(a, stream.next().unwrap());
            },
            Op::Add(a, b) => {
                let a = a.var().unwrap();
                let x = match b {
                    &Operand::Var(v) => *vars.get(&v).unwrap_or(&0),
                    &Operand::Number(x) => x,
                };
                *vars.entry(a).or_default() += x;
            },
            Op::Multiply(a, b) => {
                let a = a.var().unwrap();
                let x = match b {
                    &Operand::Var(v) => *vars.get(&v).unwrap_or(&0),
                    &Operand::Number(x) => x,
                };
                *vars.entry(a).or_default() *= x;
            },
            Op::Divide(a, b) => {
                let a = a.var().unwrap();
                let x = match b {
                    &Operand::Var(v) => *vars.get(&v).unwrap_or(&0),
                    &Operand::Number(x) => x,
                };
                assert_ne!(x, 0);
                *vars.entry(a).or_default() /= x;
            },
            Op::Modulo(a, b) => {
                let a = a.var().unwrap();
                let x = match b {
                    &Operand::Var(v) => *vars.get(&v).unwrap_or(&0),
                    &Operand::Number(x) => x,
                };
                assert!(*vars.get(&a).unwrap_or(&0) >= 0);
                assert!(x > 0);
                *vars.entry(a).or_default() %= x;
            },
            Op::Equal(a, b) => {
                let a = a.var().unwrap();
                let x = match b {
                    &Operand::Var(v) => *vars.get(&v).unwrap_or(&0),
                    &Operand::Number(x) => x,
                };
                vars.insert(a, if *vars.get(&a).unwrap_or(&0) == x { 1 } else { 0 });
            },
        }
    }

    vars[&'z']
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Op>,
}

impl Program {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut instructions = Vec::new();
        for line in input.lines() {
            instructions.push(Op::from_str(line));
        }

        Ok(Program { instructions })
    }
}

#[derive(Debug, Clone)]
enum Op {
    Input(Operand),
    Add(Operand, Operand),
    Multiply(Operand, Operand),
    Divide(Operand, Operand),
    Modulo(Operand, Operand),
    Equal(Operand, Operand),
}

impl Op {
    fn from_str(s: &str) -> Self {
        let (op, args) = s.split_once(" ").unwrap();
        match op {
            "inp" => Op::Input(Operand::from_str(args)),
            "add" => {
                let (a, b) = args.split_once(" ").unwrap();
                Op::Add(Operand::from_str(a), Operand::from_str(b))
            },
            "mul" => {
                let (a, b) = args.split_once(" ").unwrap();
                Op::Multiply(Operand::from_str(a), Operand::from_str(b))
            },
            "div" => {
                let (a, b) = args.split_once(" ").unwrap();
                Op::Divide(Operand::from_str(a), Operand::from_str(b))
            },
            "mod" => {
                let (a, b) = args.split_once(" ").unwrap();
                Op::Modulo(Operand::from_str(a), Operand::from_str(b))
            },
            "eql" => {
                let (a, b) = args.split_once(" ").unwrap();
                Op::Equal(Operand::from_str(a), Operand::from_str(b))
            },
            _ => panic!("Unknown op code: {}", op),
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Var(char),
    Number(i64),
}

impl Operand {
    fn from_str(s: &str) -> Self {
        if s.chars().all(|c| c.is_ascii_alphabetic()) {
            Operand::Var(s.chars().next().unwrap())
        } else {
            Operand::Number(s.parse().unwrap())
        }
    }

    fn var(&self) -> Option<char> {
        match self {
            &Operand::Var(v) => Some(v),
            _ => None,
        }
    }

    fn number(&self) -> Option<i64> {
        match self {
            &Operand::Number(x) => Some(x),
            _ => None,
        }
    }
}
