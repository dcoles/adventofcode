use std::fs;
use std::path::Path;

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

type Program = Vec<Word>;

fn main() {
    let input = read_input("input.txt");

    // Testing
    println!("== Testing ==");
    let stdout = run(&vec![3,0,4,0,99], vec![1]);
    println!("STDOUT: {:?}", stdout);
    assert_eq!(vec![1], stdout);

    // Part 1
    println!("== Part 1 ==");
    println!("STDOUT: {:?}", run(&input, vec![1]));

    // Part 2
    println!("== Part 2 ==");
    println!("STDOUT: {:?}", run(&input, vec![5]));
}

fn read_input<T: AsRef<Path>>(path: T) -> Program {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.trim().split(",").map(|line| line.parse::<Word>().expect("Failed to parse input")).collect()
}

/// Run a single program on an IntcodeEmulator
fn run(program: &Program, stdin: Vec<Word>) -> Vec<Word> {
    let mut cpu = IntcodeEmulator::new();
    *cpu.stdin() = stdin;
    cpu.load_program(program);
    cpu.run();

    cpu.stdout().clone()
}

struct IntcodeEmulator {
    ip: usize,
    mem: Vec<Word>,
    stdin: Vec<Word>,
    stdout: Vec<Word>,
}

impl IntcodeEmulator {
    /// Create a new IntcodeEmulator
    fn new() -> IntcodeEmulator {
        IntcodeEmulator { ip: 0, mem: vec![OP_HALT], stdin: Vec::new(), stdout: Vec::new() }
    }

    /// Input stream
    fn stdin(&mut self) -> &mut Vec<Word> {
        &mut self.stdin
    }

    /// Output stream
    fn stdout(&mut self) -> &mut Vec<Word> {
        &mut self.stdout
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
        println!("{:08x} {}", self.ip, IntcodeEmulator::opcode_to_str(op));
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
                *self.store(self.p1()) = self.stdin.pop().expect("STDIN EOF");
                self.ip += 2;
            },
            OP_OUTPUT => {
                self.stdout.push(self.load(self.p1()));
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
