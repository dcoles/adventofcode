use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Sender, Receiver};

type Word = i32;

const MODE_POSITION: Word = 0;
const MODE_IMMEDIATE: Word = 1;

const OP_ADD: Word = 1;  // [p3] = [p1] + [p2]
const OP_MUL: Word = 2;  // [p3] = [p1] * [p2]
const OP_INPUT: Word = 3;  // [p1] = read(STDIN)
const OP_OUTPUT: Word = 4;  // write(STDOUT) = [p1]
const OP_JUMP_IF_TRUE: Word = 5;  // if [p1] != 0 { ip = [p2] }
const OP_JUMP_IF_FALSE: Word = 6; // if [p1] == 0 { ip = [p2] }
const OP_LT: Word = 7;  // [p3] = if [p1] < [p2] { 1 } else { 0 }
const OP_EQ: Word = 8;  // [p3] = if [p1] == [p2] { 1 } else { 0 }
const OP_HALT: Word = 99;  // ...but don't catch fire

const DEBUG: bool = false;

type Program = Vec<Word>;

fn main() {
    let input = read_input("input.txt");

    // Testing
    assert_eq!(43210,
               run_pipeline(&vec![4, 3, 2, 1, 0],
                            &vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]));
    assert_eq!(54321,
               run_pipeline(&vec![0, 1, 2, 3, 4],
                            &vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]));
    assert_eq!(65210,
               run_pipeline(&vec![1, 0, 4, 3, 2],
                            &vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0]));

    // Part 1
    let mut max_thrust = 0;
    let mut phase = Vec::new();
    for perm in permutations(&vec![0,1,2,3,4]) {
        let thrust = run_pipeline(&perm, &input);
        if thrust > max_thrust {
            max_thrust = thrust;
            phase = perm;
        }
    }

    println!("Part 1: Max thrust is {} ({:?})", max_thrust, phase);
}

fn read_input<T: AsRef<Path>>(path: T) -> Program {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.trim().split(",").map(|line| line.parse::<Word>().expect("Failed to parse input")).collect()
}

fn run_pipeline(phases: &Vec<Word>, program: &Program) -> Word {
    let mut input = 0;
    for i in 0..phases.len() {
        let stdin = vec![phases[i], input];
        input = run(program, &stdin);
    }

    input
}

fn permutations(input: &Vec<Word>) -> Vec<Vec<Word>> {
    let mut input = input.clone();
    let mut output = Vec::new();
    let len = input.len();
    permutations_(&mut output, &mut input, 0, len);

    output
}

fn permutations_(output: &mut Vec<Vec<Word>>, input: &mut Vec<Word>, start: usize, end: usize) {
    if start == end {
        output.push(input.clone());
    }

    for i in start..end {
        input.swap(start, i);
        permutations_(output, input, start+1, end);
        input.swap(start, i);
    }
}

/// Run a single program on an IntcodeEmulator
fn run(program: &Program, stdin: &Vec<Word>) -> Word {
    let (stdin_source, stdin_sink) = channel();
    let (stdout_source, stdout_sink) = channel();

    for &val in stdin {
        stdin_source.send(val).expect("Failed to write to STDIN");
    }

    let mut cpu = IntcodeEmulator::new(stdin_sink, stdout_source);
    cpu.load_program(program);
    cpu.run();

    stdout_sink.recv().expect("Failed to read stdout")
}

struct IntcodeEmulator {
    ip: usize,
    mem: Vec<Word>,
    stdin: Receiver<Word>,
    stdout: Sender<Word>,
}

impl IntcodeEmulator {
    /// Create a new IntcodeEmulator
    fn new(stdin: Receiver<Word>, stdout: Sender<Word>) -> IntcodeEmulator {
        IntcodeEmulator { ip: 0, mem: vec![OP_HALT], stdin, stdout }
    }

    /// Load a program into memory
    fn load_program(&mut self, program: &Program) {
        self.mem = program.clone();
    }

    /// Run a program
    fn run(&mut self) {
        self.ip = 0;
        while self.op() != OP_HALT {
            self.step()
        }
    }

    fn step(&mut self) {
        let op = self.op();
        if DEBUG {
            println!("{:08x} {}", self.ip, IntcodeEmulator::opcode_to_str(op));
        }
        match op {
            OP_ADD => {
                *self.store(self.p3()) = self.load(self.p1()) + self.load(self.p2());
                self.ip += 4;
            },
            OP_MUL => {
                *self.store(self.p3()) = self.load(self.p1()) * self.load(self.p2());
                self.ip += 4;
            },
            OP_INPUT => {
                *self.store(self.p1()) = self.stdin.recv().expect("STDIN EOF");
                self.ip += 2;
            },
            OP_OUTPUT => {
                self.stdout.send(self.load(self.p1())).expect("Failed to write to STDOUT");
                self.ip += 2;
            },
            OP_JUMP_IF_TRUE => {
                if self.load(self.p1()) != 0 {
                    self.ip = self.load(self.p2()) as usize;
                    return;
                }
                self.ip += 3;
            },
            OP_JUMP_IF_FALSE => {
                if self.load(self.p1()) == 0 {
                    self.ip = self.load(self.p2()) as usize;
                    return;
                }
                self.ip += 3;
            },
            OP_LT => {
                *self.store(self.p3()) = if self.load(self.p1()) < self.load(self.p2()) { 1 } else { 0 };
                self.ip += 4;
            },
            OP_EQ => {
                *self.store(self.p3()) = if self.load(self.p1()) == self.load(self.p2()) { 1 } else { 0 };
                self.ip += 4;
            },
            OP_HALT => return,
            _ => panic!("Unknown opcode {} @ {:08x}", op, self.ip),
        };
    }

    /// The current instruction's op-code
    fn op(&self) -> Word {
        self.mem[self.ip] % 100
    }

    /// The current instruction's parameter modes
    fn modes(&self) -> Word {
        self.mem[self.ip] / 100
    }

    /// Load a value from memory
    fn load(&self, param: Param) -> Word {
        match param {
            Param::Position(addr) => self.mem[addr],
            Param::Immediate(value) => value,
        }
    }

    /// Store a value to memory
    fn store(&mut self, param: Param) -> &mut Word {
        match param {
            Param::Position(addr) => &mut self.mem[addr],
            Param::Immediate(_) => panic!("Illegal store in immediate mode"),
        }
    }

    /// First parameter
    fn p1(&self) -> Param {
        let mode = self.modes() % 10;
        Param::new(self.mem[self.ip+1], mode).expect("Unknown mode")
    }

    /// Second parameter
    fn p2(&self) -> Param {
        let mode = self.modes() / 10 % 10;
        Param::new(self.mem[self.ip+2], mode).expect("Unknown mode")
    }

    /// Third parameter
    fn p3(&self) -> Param {
        let mode = self.modes() / 100 % 10;
        Param::new(self.mem[self.ip+3], mode).expect("Unknown mode")
    }

    fn opcode_to_str(opcode: Word) -> &'static str {
        match opcode {
            OP_ADD => "ADD",
            OP_MUL => "MUL",
            OP_INPUT => "INPUT",
            OP_OUTPUT => "OUTPUT",
            OP_JUMP_IF_TRUE => "JMPTRUE",
            OP_JUMP_IF_FALSE => "JMPFALSE",
            OP_LT => "CMPLT",
            OP_EQ => "CMPEQ",
            OP_HALT => "HALT",
            _ => "UNKNOWN",
        }
    }
}

enum Param {
    Position(usize),
    Immediate(Word),
}

impl Param {
    fn new(value: Word, mode: Word) -> Option<Param> {
        match mode {
            MODE_POSITION => Some(Param::Position(value as usize)),
            MODE_IMMEDIATE => Some(Param::Immediate(value)),
            _ => None,
        }
    }
}
