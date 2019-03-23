use std::fs;

const DEBUG: bool = false;

fn main() {
    let input = read_input("input.txt");

    // Part 1
    input.run([0, 0, 0, 0, 0, 0]);

    // Part 2
    input.run([1, 0, 0, 0, 0, 0]);
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

impl Program {
    fn run(&self, reg: Registers) {
        let mut ip = 0;
        let mut reg: Registers = reg;
        while ip < self.instructions.len() {

            let (opcode, a, b, c) = self.instructions[ip];
            if let Some(ip_reg) = self.ip_reg {
                reg[ip_reg] = ip;
            }
            if ip == 1 {
                self.inner_loop(&mut reg);
            } else {
                opcode.call(&mut reg, a, b, c);
            }
            if let Some(ip_reg) = self.ip_reg {
                ip = reg[ip_reg];
            }
            if DEBUG { println!("ip={} {:?}", ip, reg) };
            ip += 1;
        }

        println!("HALT ip={} reg={:?}", ip, reg);
    }

    fn inner_loop(&self, reg: &mut Registers) {
        let mut a = reg[0];
        let mut b = reg[1];
        let mut c = reg[2];
        let mut d = reg[3];
        let mut ip = reg[5];

        for c in 1..=d {
            if d % c == 0 {
                a += c;
            }
        }
        ip = 256;

        reg[0] = a;
        reg[1] = b;
        reg[2] = c;
        reg[3] = d;
        reg[5] = ip;
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
