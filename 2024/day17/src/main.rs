//! Advent of Code 2024: Day 17
//! https://adventofcode.com/2024/day/17

use std::{fs, io};
use std::path::Path;

fn main() {
    //let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    let p2 = part2(&input);
    assert_eq!(State::new([p2, 0, 0]).run(&input.program), input.program);
    //println!("Part 2: {}", part2(&input));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    /// Division (output Register A)
    Adv,
    /// Bitwise XOR of Register B and literal
    Bxl,
    /// Combo
    Bst,
    /// Jump Non-Zero
    Jnz,
    /// Bitwise XOR of Register B and Register C
    Bxc,
    /// Output
    Out,
    /// Division (output Register B)
    Bdv,
    /// Division (output Register C)
    Cdv,
}

impl TryFrom<usize> for Op {
    type Error = usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            opcode => Err(opcode)
        }
    }
}

const BITMASK: usize = 0b111;

#[derive(Debug, Clone)]
struct State {
    ip: usize,
    regs: [usize; 3],
}

impl State {
    fn new(regs: [usize; 3]) -> Self {
        State {
            ip: 0,
            regs,
        }
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.regs[REG_A],
            5 => self.regs[REG_B],
            6 => self.regs[REG_C],
            _ => panic!("invalid combo operand: {operand}"),
        }
    }

    fn run(&mut self, program: &[usize]) -> Vec<usize> {
        let mut ip = 0;
        let mut output = vec![];

        while ip < program.len() {
            let op = Op::try_from(program[ip]).unwrap();
            let operand = program[ip + 1];
            ip += 2;

            match op {
                Op::Adv => {
                    self.regs[REG_A] = self.regs[REG_A] >> self.combo(operand);
                }
                Op::Bxl => {
                    self.regs[REG_B] = self.regs[REG_B] ^ operand;
                }
                Op::Bst => {
                    self.regs[REG_B] = self.combo(operand) & BITMASK;
                }
                Op::Jnz => {
                    if self.regs[REG_A] != 0 {
                        ip = operand;
                    }
                }
                Op::Bxc => {
                    self.regs[REG_B] = self.regs[REG_B] ^ self.regs[REG_C];
                }
                Op::Out => {
                    output.push(self.combo(operand) & BITMASK);
                }
                Op::Bdv => {
                    self.regs[REG_B] = self.regs[REG_A] >> self.combo(operand);
                }
                Op::Cdv => {
                    self.regs[REG_C] = self.regs[REG_A] >> self.combo(operand);
                }
            }
        }

        output
    }

    fn run2(&mut self, program: &[usize]) -> usize {
        let mut acc = 0;

        let mut z = 0;
        'outer: for x in program.iter().copied().rev() {
            println!("{x}: acc={acc}");

            for n in 0..=0b1111111111 {
                self.regs = [acc << 6 | n, 0, 0];

                let mut ip = 0;
                while ip < program.len() {
                    let op = Op::try_from(program[ip]).unwrap();
                    let operand = program[ip + 1];
                    ip += 2;

                    match op {
                        Op::Adv => {
                            self.regs[REG_A] = self.regs[REG_A] >> self.combo(operand);
                        }
                        Op::Bxl => {
                            self.regs[REG_B] = self.regs[REG_B] ^ operand;
                        }
                        Op::Bst => {
                            self.regs[REG_B] = self.combo(operand) & BITMASK;
                        }
                        Op::Jnz => {
                            if self.regs[REG_A] != 0 {
                                ip = operand;
                            }
                        }
                        Op::Bxc => {
                            self.regs[REG_B] = self.regs[REG_B] ^ self.regs[REG_C];
                        }
                        Op::Out => {
                            if self.combo(operand) & BITMASK == x {
                                acc = (acc << 3) | (n >> 3);
                                z += 1;
                                if z == program.len() {
                                    acc = (acc << 3) | (n & 0b111);
                                }
                                continue 'outer;
                            }
                        }
                        Op::Bdv => {
                            self.regs[REG_B] = self.regs[REG_A] >> self.combo(operand);
                        }
                        Op::Cdv => {
                            self.regs[REG_C] = self.regs[REG_A] >> self.combo(operand);
                        }
                    }

                }
            }
            panic!();
        }

        println!("acc = {acc}");
        acc
    }

    fn combo2(&mut self, operand: usize, value: usize) {
        match operand {
            0 => assert_eq!(value, 0),
            1 => assert_eq!(value, 1),
            2 => assert_eq!(value, 2),
            3 => assert_eq!(value, 3),
            4 => self.regs[REG_A] = (self.regs[REG_A] & !0b111) | (value & 0b111),
            5 => self.regs[REG_B] = (self.regs[REG_B] & !0b111) | (value & 0b111),
            6 => self.regs[REG_C] = (self.regs[REG_C] & !0b111) | (value & 0b111),
            _ => panic!("unknown combo operand {operand}"),
        }
    }
}

fn part1(input: &Input) -> String {
    let output = State::new(input.registers).run(&input.program);

    let output: Vec<_> = output.into_iter().map(|x| x.to_string()).collect();

    output.join(",")
}

fn part2(input: &Input) -> usize {
    State::new(input.registers).run2(&input.program)
}

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[derive(Debug, Clone)]
struct Input {
    registers: [usize; 3],
    program: Vec<usize>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let (chunk1, chunk2) = input.split_once("\n\n").unwrap();

        let mut registers = [0; 3];
        for (n, line) in chunk1.lines().enumerate() {
            let (_, value) = line.trim().split_once(": ").unwrap();
            registers[n] = value.parse().unwrap();
        }

        let (_, program) = chunk2.split_once(": ").unwrap();
        let program = program.trim().split(',').map(|s| s.parse().unwrap()).collect();

        Ok(Self { registers, program })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), "5,1,3,4,3,7,2,1,7");
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 117440);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        // Not 217269162253005
        assert_eq!(part2(&input), 0);
    }
}
