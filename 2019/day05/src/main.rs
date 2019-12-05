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
    let mut comp = IntcodeEmulator::new();
    comp.stdin().push(1);
    comp.load_program(&vec![3,0,4,0,99]);
    comp.run();
    println!("STDOUT: {:?}", comp.stdout());
    assert_eq!(&vec![Word::from(1)], comp.stdout());

    // Part 1
    println!("== Part 1 ==");
    let mut comp = IntcodeEmulator::new();
    comp.stdin().push(1);
    comp.load_program(&input);
    comp.run();
    println!("STDOUT: {:?}", comp.stdout());

    // Part 2
    println!("== Part 2 ==");
    let mut comp = IntcodeEmulator::new();
    comp.stdin().push(5);
    comp.load_program(&input);
    comp.run();
    println!("STDOUT: {:?}", comp.stdout());
}

fn read_input<T: AsRef<Path>>(path: T) -> Program {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.trim().split(",").map(|line| line.parse::<Word>().expect("Failed to parse input")).collect()
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
        match param.mode {
            MODE_POSITION => self.mem[param.value as usize],
            MODE_IMMEDIATE => param.value,
            _ => panic!("Unknown mode {}", param.mode),
        }
    }

    /// Store a value to memory
    fn store(&mut self, param: Param) -> &mut Word {
        match param.mode {
            MODE_POSITION => &mut self.mem[param.value as usize],
            MODE_IMMEDIATE => panic!("Illegal store in immediate mode"),
            _ => panic!("Unknown mode {}", param.mode),
        }
    }

    /// First parameter
    fn p1(&self) -> Param {
        Param { value: self.mem[self.ip+1], mode: self.modes() % 10 }
    }

    /// Second parameter
    fn p2(&self) -> Param {
        Param { value: self.mem[self.ip+2], mode: self.modes() / 10 % 10 }
    }

    /// Third parameter
    fn p3(&self) -> Param {
        Param { value: self.mem[self.ip+3], mode: self.modes() / 100 % 10 }
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

#[derive(Copy, Clone)]
struct Param {
    value: Word,
    mode: Word,
}
