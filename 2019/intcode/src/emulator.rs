use std::collections::VecDeque;
use std::convert::{TryInto, TryFrom};
use std::fmt;

pub type Word = i32;

const MODE_POSITION: Word = 0;
const MODE_IMMEDIATE: Word = 1;

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
    current_instruction: Instruction,
    mem: Vec<Word>,
    input: VecDeque<Word>,
}

impl IntcodeEmulator {
    /// Create a new IntcodeEmulator
    pub fn new() -> IntcodeEmulator {
        let current_instruction = Instruction::new(Opcode::Halt.into()).ok().unwrap();
        let mem = vec![current_instruction.into()];
        let input = VecDeque::new();
        IntcodeEmulator { ip: 0, current_instruction, mem, input }
    }

    /// The current instruction pointer address
    pub fn ip(&self) -> usize {
        self.ip
    }

    /// The current memory contents
    pub fn mem(&self) -> &[Word] {
        &self.mem
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

        self.current_instruction = Instruction::new(self.mem[self.ip]).map_err(|_| Exception::IllegalInstruction(self.mem[self.ip]))?;

        if DEBUG {
            eprintln!("{:08x} {}", self.ip, self.current_instruction.op);
        }
        match self.current_instruction.op {
            Opcode::Add => {
                *self.store(3)? = self.load(1)? + self.load(2)?;
                self.ip += 4;
            },
            Opcode::Mul => {
                *self.store(3)? = self.load(1)? * self.load(2)?;
                self.ip += 4;
            },
            Opcode::Input => {
                if let Some(input) = self.input.pop_front() {
                    *self.store(1)? = input;
                    self.ip += 2;
                } else {
                    // Upcall to request input
                    return Err(Exception::Input);
                }
            },
            Opcode::Output => {
                let output = self.load(1)?;
                self.ip += 2;
                // Upcall for output
                return Err(Exception::Output(output));
            },
            Opcode::JumpIfTrue => {
                if self.load(1)? != 0 {
                    self.ip = self.load(2)?.try_into()  // must not be negative
                        .or(Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                } else {
                    self.ip += 3;
                }
            },
            Opcode::JumpIfFalse => {
                if self.load(1)? == 0 {
                    self.ip = self.load(2)?.try_into()  // must not be negative
                        .or(Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                } else {
                    self.ip += 3;
                }
            },
            Opcode::LessThan => {
                *self.store(3)? = if self.load(1)? < self.load(2)? { 1 } else { 0 };
                self.ip += 4;
            },
            Opcode::Equal => {
                *self.store(3)? = if self.load(1)? == self.load(2)? { 1 } else { 0 };
                self.ip += 4;
            },
            Opcode::Halt => return Err(Exception::Halt),
        };

        Ok(())
    }

    /// Dump memory to console
    pub fn dump_memory(&self) {
        eprintln!("Dumping memory...");
        for addr in (0..self.mem.len()).step_by(8) {
            let flag = if addr == (self.ip & 0xfffffff8) { '>' } else { ' ' };
            let line: Vec<_> = (addr..self.mem.len().min(addr+8))
                .map(|addr| {
                    let flag = if addr == self.ip { '←' } else { ' ' };
                    format!("{:-11}{}", self.mem[addr], flag)
                }).collect();
            eprintln!("{} {:08x} {}", flag, addr, line.join(" "));
        }
    }

    /// Load a value from memory
    fn load(&self, param: usize) -> Result<Word, Exception> {
        assert!(param >= 1);
        let mode = self.current_instruction.mode_for(param);
        let addr = self.ip + param;
        let value = self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))?;
        match mode {
            MODE_POSITION => {
                // Must not be negative
                let addr = value.try_into().or_else(|_| Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))
            },
            MODE_IMMEDIATE => Ok(value),
            _ => Err(Exception::IllegalInstruction(self.mem[self.ip]))
        }
    }

    /// Store a value to memory
    fn store(&mut self, param: usize) -> Result<&mut Word, Exception> {
        assert!(param >= 1);
        let mode = self.current_instruction.mode_for(param);
        let addr = self.ip + param;
        let value = self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))?;
        match mode {
            MODE_POSITION => {
                // Must not be negative
                let addr = value.try_into().or_else(|_| Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                self.mem.get_mut(addr).ok_or(Exception::SegmentationFault(addr))
            },
            // NOTE: Immediate mode is invalid for store
            _ => Err(Exception::IllegalInstruction(self.mem[self.ip])),
        }
    }
}

/// Instruction
#[derive(Copy, Clone)]
struct Instruction {
    op: Opcode,
    modes: Word,
}

impl Instruction {
    fn new(instruction: Word) -> Result<Instruction, Exception> {
        let op = (instruction % 100).try_into().map_err(|_| Exception::IllegalInstruction(instruction))?;  // Lower 2 digits
        let modes = instruction / 100;  // Upper digits

        Ok(Instruction { op, modes })
    }

    /// Mode for parameter `n`
    fn mode_for(self, param: usize) -> Word {
        assert!(param >= 1);
        let exponent = param.checked_sub(1).unwrap() as u32;

        let base: Word = 10;  // Ensure correct type
        self.modes / base.pow(exponent) % 10
    }
}

impl From<Instruction> for Word {
    fn from(instruction: Instruction) -> Self {
        instruction.modes * 100 + Word::from(instruction.op)
    }
}

/// Opcodes
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Opcode {
    Add,  // [p3] = [p1] + [p2]
    Mul,  // [p3] = [p1] * [p2]
    Input,  // [p1] = read(STDIN)
    Output,  // write(STDOUT) = [p1]
    JumpIfTrue,  // if [p1] != 0 { ip = [p2] }
    JumpIfFalse,  // if [p1] == 0 { ip = [p2] }
    LessThan,  // [p3] = if [p1] < [p2] { 1 } else { 0 }
    Equal,  // [p3] = if [p1] == [p2] { 1 } else { 0 }
    Halt,  // ...but don't catch fire
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Opcode::*;
        let s = match self {
            Add => "ADD",
            Mul => "MUL",
            Input => "INPUT",
            Output => "OUTPUT",
            JumpIfTrue => "JMPTRUE",
            JumpIfFalse => "JMPFALSE",
            LessThan => "CMPLT",
            Equal => "CMPEQ",
            Halt => "HALT",
        };

        f.write_str(s)
    }
}

impl TryFrom<Word> for Opcode {
    type Error = ();

    fn try_from(word: Word) -> Result<Self, Self::Error> {
        use Opcode::*;
        match word {
            1 => Ok(Add),
            2 => Ok(Mul),
            3 => Ok(Input),
            4 => Ok(Output),
            5 => Ok(JumpIfTrue),
            6 => Ok(JumpIfFalse),
            7 => Ok(LessThan),
            8 => Ok(Equal),
            99 => Ok(Halt),
            _ => Err(()),
        }
    }
}

impl From<Opcode> for Word {
    fn from(op: Opcode) -> Self {
        use Opcode::*;
        match op {
            Add => 1,
            Mul => 2,
            Input => 3,
            Output => 4,
            JumpIfTrue => 5,
            JumpIfFalse => 6,
            LessThan => 7,
            Equal => 8,
            Halt => 99,
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
