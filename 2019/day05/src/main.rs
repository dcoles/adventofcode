use std::{fs, ops};
use std::path::Path;
use std::ops::Index;

type Word = i32;

const MODE_POSITION: Word = 0;
const MODE_IMMEDIATE: Word = 1;

const OP_ADD: Word = 1;  // r[p3] = r[p1] + r[p2]
const OP_MUL: Word = 2;  // r[p3] = r[p1] * r[p2]
const OP_INPUT: Word = 3;  // r[p1] = read(STDIN)
const OP_OUTPUT: Word = 4;  // write(STDOUT) = r[p1]
const OP_HALT: Word = 99;

fn main() {
    let input = read_input("input.txt");

    // Testing
    let program = Program(vec![3,0,4,0,99]);
    let mut stdin = vec![1];
    let mut stdout = Vec::new();
    run(&program, &mut stdin, &mut stdout);
    assert_eq!(vec![1], stdout);

    // Part 1
    let mut program = input.clone();
    let mut stdin = vec![1];
    let mut stdout = Vec::new();
    run(&program, &mut stdin, &mut stdout);

    print!("STDOUT: {:?}", stdout);
}

fn read_input<T: AsRef<Path>>(path: T) -> Program {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    let instructions = contents.trim().split(",").map(|line| line.parse::<Word>().expect("Failed to parse input")).collect();

    Program(instructions)
}

fn run(program: &Program, stdin: &mut Vec<Word>, stdout: &mut Vec<Word>) {
    let mut mem = program.clone();
    let mut ip = 0;
    loop {
        let mode_op = mem[ip];
        let op = mode_op % 100;
        let mode = mode_op / 100;
        println!("{:08x} {}", ip, opcode_to_str(op));
        match op {
            OP_ADD => {
                let (p1, m1) = (mem[ip+1], mode % 10);
                let (p2, m2) = (mem[ip+2], mode / 10 % 10);
                let (p3, m3) = (mem[ip+3], mode / 100 % 10);
                *mem.store(p3, m3) = mem.load(p1, m1) + mem.load(p2, m2);
                ip += 4;
            },
            OP_MUL => {
                let (p1, m1) = (mem[ip+1], mode % 10);
                let (p2, m2) = (mem[ip+2], mode / 10 % 10);
                let (p3, m3) = (mem[ip+3], mode / 100 % 10);
                *mem.store(p3, m3) = mem.load(p1, m1) * mem.load(p2, m2);
                ip += 4;
            },
            OP_INPUT => {
                let (p1, m1) = (mem[ip+1], mode % 10);
                *mem.store(p1, m1) = stdin.pop().expect("STDIN EOF");
                ip += 2;
            },
            OP_OUTPUT => {
                let (p1, m1) = (mem[ip+1], mode % 10);
                stdout.push(mem.load(p1, m1));
                ip += 2;
            },
            OP_HALT => {
                ip += 1;
                break;
            },
            _ => panic!("Unknown opcode {} @ {}", op, ip),
        }
    }
}

fn opcode_to_str(opcode: Word) -> &'static str {
    match opcode {
        OP_ADD => "ADD",
        OP_MUL => "MUL",
        OP_INPUT => "INPUT",
        OP_OUTPUT => "OUTPUT",
        OP_HALT => "HALT",
        _ => "UNKNOWN",
    }
}

#[derive(Clone)]
struct Program(Vec<Word>);

impl Program {
    fn load(&self, param: Word, mode: Word) -> Word {
        match mode {
            MODE_POSITION => self[param],
            MODE_IMMEDIATE => param,
            _ => panic!("Unknown mode {}", mode),
        }
    }

    fn store(&mut self, param: Word, mode: Word) -> &mut Word {
        match mode {
            MODE_POSITION => &mut self[param],
            MODE_IMMEDIATE => panic!("Illegal store in immediate mode"),
            _ => panic!("Unknown mode {}", mode),
        }
    }
}

impl ops::Index<Word> for Program {
    type Output = Word;

    fn index(&self, index: Word) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl ops::IndexMut<Word> for Program {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
