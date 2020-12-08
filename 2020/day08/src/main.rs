use std::path::Path;
use std::fs;
use std::collections::HashSet;

type Program = Vec<Instruction>;

fn main() {
    let program = read_input("input.txt");

    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program));
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

fn part2(input: &Program) -> i32 {
    for i in 0..input.len() {
        let mut program = input.clone();

        match program[i].op {
            Operation::NOP => program[i].op = Operation::JMP,
            Operation::JMP => program[i].op = Operation::NOP,
            _ => (),
        }

        let mut cpu = CPU::from_program(&program);
        let mut seen = HashSet::new();
        while !seen.contains(&cpu.pc) && cpu.pc < cpu.program.len() {
            seen.insert(cpu.pc);
            cpu.step();
        }

        if cpu.pc == program.len() {
            return cpu.acc;
        }
    }

    panic!("No solution found!")
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

struct CPU<'a> {
    pc: usize,
    acc: i32,
    program: &'a [Instruction],
}

impl<'a> CPU<'a> {
    fn from_program(program: &'a Program) -> Self {
        CPU { pc: 0, acc: 0, program }
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

    #[test]
    fn test_part2() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input), 8);
    }
}

