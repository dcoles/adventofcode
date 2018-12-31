use std::fs;
use std::slice::Iter;
use std::str;

fn main() {
    let input = read_input("input.txt");

    let mut count = 0;
    for sample in &input {
        let n_match = Opcode::iterator().filter(|o| {
            let mut t = sample.input.clone();
            o.call(&mut t, sample.opcode[1], sample.opcode[2], sample.opcode[3]);
            t == sample.output
        }).count();
        if n_match >= 3 {
            count += 1;
        }
    }

    println!("{} samples match three or more opcodes", count);
}

fn read_input(filename: &str) -> Vec<Sample> {
    let mut samples = Vec::new();
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    let mut lines = input.lines();
    while let Some(sample) = read_sample(&mut lines) {
        samples.push(sample);
    }

    samples
}

fn read_sample(lines: &mut str::Lines) -> Option<Sample> {
    let input = lines.next()?;
    let opcode = lines.next()?;
    let output = lines.next()?;
    lines.next()?;

    if input == "" {
        return None;
    }

    let input: [usize; 4] = read_array(&input[9..]);
    let opcode: [usize; 4] = read_array(&opcode[..]);
    let output: [usize; 4] = read_array(&output[9..]);

    Some(Sample { input, opcode, output })
}

fn read_array(input: &str) -> [usize; 4] {
    let mut result = [0; 4];
    let input: Vec<_> = input.replace(",", "")
        .replace("[", "")
        .replace("]", "")
        .split_whitespace().map(|v|
            v.trim().parse().expect("Failed to read array")
        ).collect();
    result.copy_from_slice(&input[..]);

    result
}

#[derive(Debug)]
struct Sample {
    input: [usize; 4],
    opcode: [usize; 4],
    output: [usize; 4],
}

#[derive(Debug)]
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

type Registers = [usize; 4];

impl Opcode {
    fn iterator() -> Iter<'static, Opcode> {
        use self::Opcode::*;
        static OPCODES: [Opcode; 16] = [
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
        ];
        OPCODES.into_iter()
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
