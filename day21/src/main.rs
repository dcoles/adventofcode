use std::collections::HashSet;
use std::fs;

const DEBUG: bool = false;

fn main() {
    let input = read_input("input.txt");

    // Part 1
    let mut executor = Executor::new();
    executor.run(&input, |ip, _reg| ip == 28);  // Break at halt condition
    let key = executor.reg[5];  // Grab the expected value
    let mut executor = Executor::new();
    executor.reg[0] = key;
    executor.run(&input, |_,_| false);
    println!("The lowest non-negative integer value for register 0 that causes a halt is {}", key);

    // Part 2
    let mut last = 0;
    let mut seen = HashSet::new();
    let mut executor = Executor::new();
    executor.run(&input, |ip, reg| {
        if ip == 28 {
            if seen.contains(&reg[5]) {
                return true;
            }
            seen.insert(reg[5]);
            last = reg[5];
        }
        false
    });  // Break at halt condition
    println!("The lowest non-negative integer value for register 0 that causes a halt after the most instructions is {}", last);
}

fn read_input(filename: &str) -> Program {
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    let mut ip_reg = None;
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.starts_with("#ip") {
            let reg: usize = line[3..].trim().parse().unwrap();
            ip_reg = Some(reg);
        } else {
            let mut iter = line.split_whitespace();
            let opcode = Opcode::from_str(&iter.next().unwrap());
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            let c: usize = iter.next().unwrap().parse().unwrap();
            instructions.push((opcode, a, b, c));
        }
    }

    Program { ip_reg, instructions }
}

struct Program {
    ip_reg: Option<usize>,
    instructions: Vec<Instruction>,
}

struct Executor {
    ip: usize,
    reg: [usize; 6]
}

impl Executor {
    fn new() -> Executor {
        Executor { ip: 0, reg: Default::default() }
    }

    fn run<F>(&mut self, program: &Program, mut break_if: F)
    where F: FnMut(usize, &Registers) -> bool {
        while self.ip < program.instructions.len() {
            if break_if(self.ip, &self.reg) {
                break;
            }
            let (opcode, a, b, c) = program.instructions[self.ip];
            if let Some(ip_reg) = program.ip_reg {
                self.reg[ip_reg] = self.ip;
            }
            opcode.call(&mut self.reg, a, b, c);
            if let Some(ip_reg) = program.ip_reg {
                self.ip = self.reg[ip_reg];
            }
            if DEBUG { println!("ip={} {:?}", self.ip, self.reg) };
            self.ip += 1;
        }

        println!("HALT ip={} reg={:?}", self.ip, self.reg);
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Opcode {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

type Registers = [usize; 6];
type Instruction = (Opcode, usize, usize, usize);

impl Opcode {
    fn from_str(opcode: &str) -> Opcode {
        use self::Opcode::*;
        match opcode {
            "addr" => ADDR,
            "addi" => ADDI,
            "mulr" => MULR,
            "muli" => MULI,
            "banr" => BANR,
            "bani" => BANI,
            "borr" => BORR,
            "bori" => BORI,
            "setr" => SETR,
            "seti" => SETI,
            "gtir" => GTIR,
            "gtri" => GTRI,
            "gtrr" => GTRR,
            "eqir" => EQIR,
            "eqri" => EQRI,
            "eqrr" => EQRR,
            _ => panic!("Unknown opcode {:?}", opcode),
        }
    }
}

impl Opcode {
    fn call(&self, reg: &mut Registers, a: usize, b: usize, c: usize) {
        match self {
            Opcode::ADDR => reg[c] = reg[a] + reg[b],
            Opcode::ADDI => reg[c] = reg[a] + b,
            Opcode::MULR => reg[c] = reg[a] * reg[b],
            Opcode::MULI => reg[c] = reg[a] * b,
            Opcode::BANR => reg[c] = reg[a] & reg[b],
            Opcode::BANI => reg[c] = reg[a] & b,
            Opcode::BORR => reg[c] = reg[a] | reg[b],
            Opcode::BORI => reg[c] = reg[a] | b,
            Opcode::SETR => reg[c] = reg[a],
            Opcode::SETI => reg[c] = a,
            Opcode::GTIR => reg[c] = if a > reg[b] { 1 } else { 0 },
            Opcode::GTRI => reg[c] = if reg[a] > b { 1 } else { 0 },
            Opcode::GTRR => reg[c] = if reg[a] > reg[b] { 1 } else { 0 },
            Opcode::EQIR => reg[c] = if a == reg[b] { 1 } else { 0 },
            Opcode::EQRI => reg[c] = if reg[a] == b { 1 } else { 0 },
            Opcode::EQRR => reg[c] = if reg[a] == reg[b] { 1 } else { 0 },
        }
    }
}
