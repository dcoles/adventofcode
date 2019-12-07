use std::fs;
use std::path::Path;
use std::collections::VecDeque;

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

fn main() {
    let input = read_input("input.txt");

    // Part 1
    assert_eq!(43210,
               run_pipeline(&[4, 3, 2, 1, 0],
                            &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                            false));
    assert_eq!(54321,
               run_pipeline(&[0, 1, 2, 3, 4],
                            &[3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0],
                            false));
    assert_eq!(65210,
               run_pipeline(&[1, 0, 4, 3, 2],
                            &[3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0],
                            false));


    let (max_thrust, phase) = find_max(&[0,1,2,3,4], &input, false);
    println!("Part 1: Max thrust is {} ({:?})", max_thrust, phase);

    // Part 2
    assert_eq!(139629729,
               find_max(&[9,8,7,6,5],
                        &[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
                        true).0);

    assert_eq!(18216,
               find_max(&[9,8,7,6,5],
                        &[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
                        true).0);

    let (max_thrust, phase) = find_max(&[5,6,7,8,9], &input, true);
    println!("Part 2: Max thrust is {} ({:?})", max_thrust, phase);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Word> {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.trim().split(',').map(|line| line.parse::<Word>().expect("Failed to parse input")).collect()
}

/// Find the permutation of phases that gives the maximum thrust
fn find_max(phases: &[Word], program: &[Word], feedback: bool) -> (Word, Vec<Word>) {
    let mut max_thrust = 0;
    let mut phase = Vec::new();
    for perm in permutations(phases) {
        let thrust = run_pipeline(&perm, program, feedback);
        if thrust > max_thrust {
            max_thrust = thrust;
            phase = perm;
        }
    }

    (max_thrust, phase)
}

/// Run a pipeline of amplifiers
fn run_pipeline(phases: &[Word], program: &[Word], feedback: bool) -> Word {
    // Set up amplifiers
    let mut amplifiers = Vec::new();
    for &phase in phases {
        let mut amp = IntcodeEmulator::new();
        amp.load_program(&program);
        amp.add_input(phase);
        amplifiers.push(amp);
    }

    // Write initial input
    amplifiers[0].add_input(0);

    // Drive the pipeline until it halts
    let mut output = 0;
    for i in (0..phases.len()).cycle() {
        match amplifiers[i].run() {
            Exception::Halt => break,
            Exception::Input => panic!("EOF reading from STDIN"),
            Exception::Output(out) => {
                // Last amp outputs to thrusters
                if i == phases.len() - 1 {
                    output = out;
                    if feedback {
                        // Feedback into first amplifier
                        amplifiers[0].add_input(out);
                    }
                } else {
                    // Feed into next amplifier
                    amplifiers[i + 1].add_input(out);
                }
            },
        }
    }

    output
}

/// Calculate all permutations of a slice
fn permutations(input: &[Word]) -> Vec<Vec<Word>> {
    let mut input = input.to_owned();
    let len = input.len();

    fn permutations_(input: &mut [Word], k: usize) -> Vec<Vec<Word>> {
        if k == 1 {
            return vec![input.to_vec()];
        }

        let mut output = permutations_(input, k - 1);
        for i in 0..k-1 {
            if k % 2 == 0 {
                input.swap(i, k - 1);
            } else {
                input.swap(0, k - 1)
            }
            let mut perms = permutations_(input, k - 1);
            output.append(&mut perms);
        }

        output
    }

    permutations_(&mut input, len)
}

/// Emulates an Intcode computer
struct IntcodeEmulator {
    ip: usize,
    mem: Vec<Word>,
    input: VecDeque<Word>,
}

impl IntcodeEmulator {
    /// Create a new IntcodeEmulator
    fn new() -> IntcodeEmulator {
        IntcodeEmulator { ip: 0, mem: vec![OP_HALT], input: VecDeque::new() }
    }

    /// Load a program into memory
    fn load_program(&mut self, program: &[Word]) {
        self.ip = 0;
        self.mem = program.to_owned();
    }

    /// Queue input
    fn add_input(&mut self, input: Word) {
        self.input.push_back(input);
    }

    /// Run a program until an exception is encountered
    fn run(&mut self) -> Exception {
        loop {
            if let Some(exception) = self.step() {
                return exception;
            }
        }
    }

    /// Try to step a single instruction
    fn step(&mut self) -> Option<Exception> {
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
                if let Some(input) = self.input.pop_front() {
                    *self.store(self.p1()) = input;
                    self.ip += 2;
                } else {
                    // Upcall for input
                    return Some(Exception::Input);
                }
            },
            OP_OUTPUT => {
                let output = self.load(self.p1());
                self.ip += 2;
                // Upcall for output
                return Some(Exception::Output(output));
            },
            OP_JUMP_IF_TRUE => {
                if self.load(self.p1()) != 0 {
                    self.ip = self.load(self.p2()) as usize;
                } else {
                    self.ip += 3;
                }
            },
            OP_JUMP_IF_FALSE => {
                if self.load(self.p1()) == 0 {
                    self.ip = self.load(self.p2()) as usize;
                } else {
                    self.ip += 3;
                }
            },
            OP_LT => {
                *self.store(self.p3()) = if self.load(self.p1()) < self.load(self.p2()) { 1 } else { 0 };
                self.ip += 4;
            },
            OP_EQ => {
                *self.store(self.p3()) = if self.load(self.p1()) == self.load(self.p2()) { 1 } else { 0 };
                self.ip += 4;
            },
            OP_HALT => return Some(Exception::Halt),
            _ => panic!("Unknown opcode {} @ {:08x}", op, self.ip),
        };

        None
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

    /// Return the mnemonic for an opcode
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

/// Exception status
enum Exception {
    Halt,
    Input,
    Output(Word),
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
