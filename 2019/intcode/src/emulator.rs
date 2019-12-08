use std::collections::VecDeque;
use std::convert::TryInto;

pub type Word = i32;

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

/// An Intcode program
pub struct Program(Vec<Word>);

impl Program {
    pub fn new(instructions: &[Word]) -> Program {
        Program(instructions.to_owned())
    }
}

/// Emulates an Intcode computer
pub struct IntcodeEmulator {
    ip: usize,
    mem: Vec<Word>,
    input: VecDeque<Word>,
}

impl IntcodeEmulator {
    /// Create a new IntcodeEmulator
    pub fn new() -> IntcodeEmulator {
        IntcodeEmulator { ip: 0, mem: vec![OP_HALT], input: VecDeque::new() }
    }

    /// Load a program into memory
    pub fn load_program(&mut self, program: &Program) {
        self.ip = 0;
        self.mem = program.0.to_owned();
    }

    /// Queue input
    pub fn add_input(&mut self, input: Word) {
        self.input.push_back(input);
    }

    /// Run a program until an exception is encountered
    pub fn run(&mut self) -> Exception {
        loop {
            if let Err(exception) = self.step() {
                return exception;
            }
        }
    }

    /// Try to step a single instruction
    pub fn step(&mut self) -> Result<(), Exception> {
        if self.ip >= self.mem.len() {
            return Err(Exception::SegmentationFault(self.ip));
        }

        let op = self.op();
        if DEBUG {
            eprintln!("{:08x} {}", self.ip, IntcodeEmulator::opcode_to_str(op));
        }
        match op {
            OP_ADD => {
                *self.store(3)? = self.load(1)? + self.load(2)?;
                self.ip += 4;
            },
            OP_MUL => {
                *self.store(3)? = self.load(1)? * self.load(2)?;
                self.ip += 4;
            },
            OP_INPUT => {
                if let Some(input) = self.input.pop_front() {
                    *self.store(1)? = input;
                    self.ip += 2;
                } else {
                    // Upcall to request input
                    return Err(Exception::Input);
                }
            },
            OP_OUTPUT => {
                let output = self.load(1)?;
                self.ip += 2;
                // Upcall for output
                return Err(Exception::Output(output));
            },
            OP_JUMP_IF_TRUE => {
                if self.load(1)? != 0 {
                    self.ip = self.load(2)?.try_into()  // must not be negative
                        .or(Err(Exception::IllegalInstruction(op)))?;
                } else {
                    self.ip += 3;
                }
            },
            OP_JUMP_IF_FALSE => {
                if self.load(1)? == 0 {
                    self.ip = self.load(2)?.try_into()  // must not be negative
                        .or(Err(Exception::IllegalInstruction(op)))?;
                } else {
                    self.ip += 3;
                }
            },
            OP_LT => {
                *self.store(3)? = if self.load(1)? < self.load(2)? { 1 } else { 0 };
                self.ip += 4;
            },
            OP_EQ => {
                *self.store(3)? = if self.load(1)? == self.load(2)? { 1 } else { 0 };
                self.ip += 4;
            },
            OP_HALT => return Err(Exception::Halt),
            _ => return Err(Exception::IllegalInstruction(op)),
        };

        Ok(())
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
    fn load(&self, param: usize) -> Result<Word, Exception> {
        let mode = self.mode(param)?;
        let addr = self.ip + param;
        let value = self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))?;
        match mode {
            MODE_POSITION => {
                // Must not be negative
                let addr = value.try_into().or_else(|_| Err(Exception::IllegalInstruction(self.op())))?;
                self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))
            },
            MODE_IMMEDIATE => Ok(value),
            _ => Err(Exception::IllegalInstruction(self.op()))
        }
    }

    /// Store a value to memory
    fn store(&mut self, param: usize) -> Result<&mut Word, Exception> {
        let mode = self.mode(param)?;
        let addr = self.ip + param;
        let value = self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))?;
        match mode {
            MODE_POSITION => {
                // Must not be negative
                let addr = value.try_into().or_else(|_| Err(Exception::IllegalInstruction(self.op())))?;
                self.mem.get_mut(addr).ok_or(Exception::SegmentationFault(addr))
            },
            MODE_IMMEDIATE => {
                // Illegal store in immediate mode
                Err(Exception::IllegalInstruction(self.op()))
            },
            _ => Err(Exception::IllegalInstruction(self.op())),
        }
    }

    /// Mode for parameter
    #[allow(clippy::identity_conversion)]
    fn mode(&self, param: usize) -> Result<Word, Exception> {
        if param == 0 {
            // Can't have a 0-th parameter
            return Err(Exception::IllegalInstruction(self.op()));
        }
        let exponent = param.checked_sub(1).unwrap() as u32;

        Ok(self.modes() / Word::from(10).pow(exponent) % 10)
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
pub enum Exception {
    Halt,
    IllegalInstruction(Word),
    SegmentationFault(usize),
    Input,
    Output(Word),
}
