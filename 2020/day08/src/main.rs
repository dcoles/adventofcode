use std::path::Path;
use std::fs;
use std::collections::HashSet;

type Program = Vec<Instruction>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

/// Run program until just before an instruction would be executed a second time,
/// then return value of the accumulator.
fn part1(input: &Program) -> i32 {
    let mut cpu = CPU::from_program(input);
    run_until_loop(&mut cpu);

    cpu.acc
}

/// Mutate NOP/JMP instructions in a program until it exits cleanly,
/// then return value of the accumulator.
fn part2(input: &Program) -> i32 {
    for i in 0..input.len() {
        let mut program = input.clone();

        match program[i].op {
            Operation::NOP => program[i].op = Operation::JMP,
            Operation::JMP => program[i].op = Operation::NOP,
            _ => continue,
        }

        let mut cpu = CPU::from_program(&program);
        run_until_loop(&mut cpu);

        if cpu.is_eof() {
            return cpu.acc;
        }
    }

    panic!("No solution found!")
}

/// Step through program instructions until EOF or program would re-execute an instruction.
fn run_until_loop(cpu: &mut CPU) {
    let mut seen = HashSet::new();
    while !cpu.is_eof() && !seen.contains(&cpu.pc) {
        seen.insert(cpu.pc);
        cpu.step().expect("Execution failed");
    }
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

    fn is_eof(&self) -> bool {
        self.pc == self.program.len()
    }

    fn step(&mut self) -> Result<(), &'static str> {
        if self.pc >= self.program.len() {
            return Err("Attempt to execute out of bounds");
        }

        let instr = self.program[self.pc];
        match instr.op {
            Operation::NOP => (),
            Operation::JMP => {
                self.pc = (self.pc as i32 + instr.arg) as usize;
                return Ok(());
            },
            Operation::ACC => self.acc += instr.arg,
        }

        self.pc += 1;
        Ok(())
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

