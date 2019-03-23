use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::slice::Iter;
use std::str;

const N_OPCODES: usize = 16;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (samples, _) = read_input("input.txt");

    let mut count = 0;
    for sample in &samples {
        let n_match = Opcode::iterator().filter(|o| {
            let mut t = sample.input.clone();
            o.call(&mut t, sample.instr[1], sample.instr[2], sample.instr[3]);
            t == sample.output
        }).count();
        if n_match >= 3 {
            count += 1;
        }
    }

    println!("{} samples match three or more opcodes", count);
}

fn part2() {
    let (samples, program) = read_input("input.txt");

    // Find possible matches from samples
    let mut opcodes: HashMap<usize, HashSet<Opcode>> = HashMap::new();
    for sample in &samples {
        let mut matches: HashSet<Opcode> = Opcode::iterator().filter(|o| {
            let mut t = sample.input.clone();
            o.call(&mut t, sample.instr[1], sample.instr[2], sample.instr[3]);
            t == sample.output
        }).map(|&o| o).collect();
        if let Some(rest_matches) = opcodes.get(&sample.instr[0]) {
            matches = rest_matches.intersection(&matches).map(|&o| o).collect();
        }
        opcodes.insert(sample.instr[0], matches);
    }

    // Deduce unique opcodes
    let opcodes = loop {
        let unique: HashMap<_, _> = opcodes.iter().filter(|(_, m)|
            m.len() == 1
        ).map(|(&c, m)|
            (c, *m.iter().next().unwrap())
        ).collect();

        if unique.len() == N_OPCODES {
            break unique;
        }

        for (&opcode, &opcode_match) in &unique {
            for (&c, m) in &mut opcodes {
                if c == opcode {
                    continue;
                }
                m.remove(&opcode_match);
            }
        }
    };
    println!("{:#?}", opcodes);

    let mut reg: Registers = Default::default();
    for instr in program {
        let opcode = opcodes.get(&instr[0])
            .expect("Unknown opcode");
        opcode.call(&mut reg, instr[1], instr[2], instr[3]);
    }

    println!("Value in register 0: {}", reg[0]);
}

fn read_input(filename: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    let (input_samples, input_program) = input.split_at(
        input.find("\n\n\n").expect("Unable to find separator"));

    (read_samples(input_samples.trim()), read_program(input_program.trim()))
}

fn read_samples(input: &str) -> Vec<Sample> {
    let mut lines = input.lines();
    let mut samples = Vec::new();

    loop {
        let input = lines.next().unwrap_or_default();
        if input == "" {
            break;
        }
        let instruct = lines.next().unwrap();
        let output = lines.next().unwrap();
        lines.next();  // Separator

        let input: [usize; 4] = read_array(&input[9..]);
        let instruct: [usize; 4] = read_array(&instruct[..]);
        let output: [usize; 4] = read_array(&output[9..]);

        samples.push(Sample { input, instr: instruct, output} );
    }

    samples
}

fn read_program(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(read_array(line));
    }

    instructions
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
    instr: [usize; 4],
    output: [usize; 4],
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

type Registers = [usize; 4];
type Instruction = [usize; 4];

impl Opcode {
    fn iterator() -> Iter<'static, Opcode> {
        use self::Opcode::*;
        static OPCODES: [Opcode; N_OPCODES] = [
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
