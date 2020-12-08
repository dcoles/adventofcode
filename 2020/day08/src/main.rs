use std::path::Path;
use std::fs;
use std::collections::HashSet;

type Program = Vec<Instruction>;

fn main() {
    let program = read_input("input.txt");

    println!("Part 1: {}", part1(&program));
}

fn read_input<T: AsRef<Path>>(path: T) -> Program {
    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let op = Operation::from_str(it.next().expect("Missing instruction op"))
                .expect("Failed to parse operation");
            let arg = it.next().expect("Missing instruction arg")
                .parse().expect("Failed to parse instruction arg");

            Instruction { op, arg }
        })
        .collect()
}

fn part1(input: &Program) -> i32 {
    let mut cpu = CPU::from_program(input);

    let mut seen = HashSet::new();
    while !seen.contains(&cpu.pc) {
        seen.insert(cpu.pc);
        cpu.step();
    }

    cpu.acc
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    NOP,
    JMP,
    ACC,
}

impl Operation {
    fn from_str(s: &str) -> Option<Operation> {
        match s {
            "nop" => Some(Operation::NOP),
            "jmp" => Some(Operation::JMP),
            "acc" => Some(Operation::ACC),
            _ => None
        }
    }
}

struct CPU {
    pc: usize,
    acc: i32,
    program: Vec<Instruction>,
}

impl CPU {
    fn from_program(program: &Program) -> Self {
        CPU { pc: 0, acc: 0, program: program.clone() }
    }

    fn step(&mut self) {
        let instr = self.program[self.pc];
        match instr.op {
            Operation::NOP => (),
            Operation::JMP => {
                self.pc = (self.pc as i32 + instr.arg) as usize;
                return;
            },
            Operation::ACC => self.acc += instr.arg,
        }
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 5);
    }
}

