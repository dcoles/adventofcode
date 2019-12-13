use std::collections::VecDeque;
use std::convert::{TryInto, TryFrom};
use std::{fmt, fs, io};
use std::path::Path;
use std::io::BufRead;

pub type Word = i64;

pub const MEMSIZE: usize = 2 << 11;  // 4 KiB

const MODE_POSITION: Word = 0;
const MODE_IMMEDIATE: Word = 1;
const MODE_RELATIVE: Word = 2;

/// An Intcode program
pub struct Program(Vec<Word>);

impl Program {
    pub fn new(instructions: &[Word]) -> Program {
        Program(instructions.to_owned())
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Program, String> {
        let file = fs::File::open(&path).map_err(|err| format!("Failed to open file: {}", err))?;

        let mut reader = io::BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).map_err(|err| format!("Failed to read line: {}", err))?;

        let instructions: Result<Vec<Word>, String> = line.trim()
            .split(',')
            .map(|val| val.parse::<Word>().map_err(|err| { format!("Failed to parse value {:?}: {}", val, err) }))
            .collect();

        Ok(Program::new(&instructions?))
    }
}

/// Emulates an Intcode computer
pub struct IntcodeEmulator {
    ip: usize,
    relbase: Word,
    current_instruction: Instruction,
    mem: Vec<Word>,
    input: VecDeque<Word>,
    debug: bool,
}

impl IntcodeEmulator {
    /// Create a new IntcodeEmulator
    pub fn new() -> IntcodeEmulator {
        let current_instruction = Instruction::new(Opcode::Halt.into()).ok().unwrap();
        let mem = vec![current_instruction.into()];
        let input = VecDeque::new();

        IntcodeEmulator { ip: 0, relbase: 0, current_instruction, mem, input, debug: false }
    }

    /// The current instruction pointer address
    pub fn ip(&self) -> usize {
        self.ip
    }

    /// Set the current instruction pointer
    pub fn set_ip(&mut self, ip: usize) {
        self.ip = ip;
    }

    /// The current relative base
    pub fn rb(&self) -> Word {
        self.relbase
    }

    /// Set the current relative base
    pub fn set_rb(&mut self, rb: Word) {
        self.relbase = rb;
    }

    /// The current decoded instruction
    pub fn current_instruction(&self) -> Result<Instruction, Exception> {
        Instruction::new(*self.mem.get(self.ip).ok_or_else(|| Exception::SegmentationFault(self.ip))?)
    }

    /// The current memory contents
    pub fn mem(&self) -> &[Word] {
        &self.mem
    }

    /// The current memory contents
    pub fn mem_mut(&mut self) -> &mut [Word] {
        &mut self.mem
    }

    /// Load a program into memory
    pub fn load_program(&mut self, program: &Program) {
        self.ip = 0;
        self.mem = vec![0; MEMSIZE];
        self.mem.splice(..program.0.len(), program.0.iter().copied());
    }

    /// Get debugging flag
    pub fn get_debug(&self) -> bool {
        self.debug
    }

    /// Set debugging flag
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
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

        self.current_instruction = self.current_instruction().map_err(|_| Exception::IllegalInstruction(self.mem[self.ip]))?;
        if self.debug {
            self.print_disassembled();
        }

        if self.ip + self.current_instruction.op.nparams() >= self.mem.len() {
            return Err(Exception::SegmentationFault(self.ip));
        }

        match self.current_instruction.op {
            Opcode::Add => {
                *self.store(3)? = self.load(1)? + self.load(2)?;
            },
            Opcode::Mul => {
                *self.store(3)? = self.load(1)? * self.load(2)?;
            },
            Opcode::Input => {
                if let Some(input) = self.input.pop_front() {
                    *self.store(1)? = input;
                } else {
                    // Upcall to request input
                    if self.debug {
                        eprintln!("Waiting for input...")
                    }
                    return Err(Exception::Input);
                }
            },
            Opcode::Output => {
                let output = self.load(1)?;
                self.ip += self.current_instruction.op.nparams() + 1;

                // Upcall for output
                return Err(Exception::Output(output));
            },
            Opcode::JumpIfTrue => {
                if self.load(1)? != 0 {
                    self.ip = self.load(2)?.try_into()  // must not be negative
                        .or(Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                    return Ok(());
                }
            },
            Opcode::JumpIfFalse => {
                if self.load(1)? == 0 {
                    self.ip = self.load(2)?.try_into()  // must not be negative
                        .or(Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                    return Ok(());
                }
            },
            Opcode::LessThan => {
                *self.store(3)? = if self.load(1)? < self.load(2)? { 1 } else { 0 };
            },
            Opcode::Equal => {
                *self.store(3)? = if self.load(1)? == self.load(2)? { 1 } else { 0 };
            },
            Opcode::SetRBOffset => {
                self.relbase += self.load(1)?;
            }
            Opcode::Halt => return Err(Exception::Halt),
        };
        self.ip += self.current_instruction.op.nparams() + 1;

        Ok(())
    }

    /// Dump registers to console
    pub fn dump_registers(&self) {
        eprintln!("ip:0x{:08x} rb:{}", self.ip, self.relbase);
    }

    /// Dump memory to console
    pub fn dump_memory(&self) {
        eprintln!("Dumping memory...");
        for addr in (0..self.mem.len()).step_by(8) {
            let flag = if addr == (self.ip & (!0 - 0b111)) { '>' } else { ' ' };
            let mem = &self.mem[addr..self.mem.len().min(addr+8)];
            if mem.iter().all(|&v| v == 0) && flag == ' ' {
                // Don't print empty blocks of memory
                continue;
            }

            let line: Vec<_> = mem.iter().enumerate()
                .map(|(offset, &val)| {
                    let flag = if addr + offset == self.ip { '←' } else { ' ' };
                    format!("{:-11}{}", val, flag)
                }).collect();
            eprintln!("{} {:08x} {}", flag, addr, line.join(" "));
        }
    }

    /// Print the disassembled current instruction to the console
    pub fn print_disassembled(&self) {
        eprintln!("{:08x} {}", self.ip, self.disassemble().unwrap_or_else(|_| String::from("???")));
    }

    /// Disassemble the current instruction
    pub fn disassemble(&self) -> Result<String, String> {
        let instruction = self.current_instruction().map_err(|err| format!("Failed to decode instruction: {}", err))?;
        let params: Vec<_> = self.mem[self.ip+1..].iter()
            .chain([0].iter().cycle())
            .take(instruction.op().nparams())
            .enumerate()
            .map(|(n, &p)| (instruction.mode_for(n + 1), p))
            .collect();

        let params_str: Vec<_> = params.iter().map(|&(m, p)| {
            match m {
                MODE_POSITION => format!("0x{:08x}", p),
                MODE_IMMEDIATE => format!("${}", p),
                MODE_RELATIVE => format!("%rb{:+}", p),
                _ => format!("?{}", p),
            }
        }).collect();

        Ok(format!("{} {}", instruction.op(), params_str.join(" ")))
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
            MODE_RELATIVE => {
                let addr = (self.relbase + value).try_into().or_else(|_| Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                self.mem.get(addr).copied().ok_or(Exception::SegmentationFault(addr))
            },
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
            MODE_RELATIVE => {
                let addr = (self.relbase + value).try_into().or_else(|_| Err(Exception::IllegalInstruction(self.mem[self.ip])))?;
                self.mem.get_mut(addr).ok_or(Exception::SegmentationFault(addr))
            },
            // NOTE: Immediate mode is invalid for store
            _ => Err(Exception::IllegalInstruction(self.mem[self.ip])),
        }
    }
}

/// Instruction
#[derive(Copy, Clone)]
pub struct Instruction {
    op: Opcode,
    modes: Word,
}

impl Instruction {
    fn new(instruction: Word) -> Result<Instruction, Exception> {
        let op = (instruction % 100).try_into().map_err(|_| Exception::IllegalInstruction(instruction))?;  // Lower 2 digits
        let modes = instruction / 100;  // Upper digits

        Ok(Instruction { op, modes })
    }

    /// Opcode of instruction
    pub fn op(self) -> Opcode {
        self.op
    }

    /// Mode for parameter `n`
    pub fn mode_for(self, param: usize) -> Word {
        assert!(param >= 1);
        let exponent = param.checked_sub(1).unwrap() as u32;

        let base: Word = 10;  // Ensure correct type
        (self.modes / base.pow(exponent)) % 10
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
    Add,  // 1: [p3] = [p1] + [p2]
    Mul,  // 2: [p3] = [p1] * [p2]
    Input,  // 3: [p1] = read(STDIN)
    Output,  // 4: write(STDOUT) = [p1]
    JumpIfTrue,  // 5: if [p1] != 0 { ip = [p2] }
    JumpIfFalse,  // 6: if [p1] == 0 { ip = [p2] }
    LessThan,  // 7: [p3] = if [p1] < [p2] { 1 } else { 0 }
    Equal,  // 8: [p3] = if [p1] == [p2] { 1 } else { 0 }
    SetRBOffset,  // 9: relbase += [p1]
    Halt,  // 99: ...but don't catch fire
}

impl Opcode {
    /// Number of parameters this opcode takes
    pub fn nparams(self) -> usize {
        use Opcode::*;
        match self {
            Add => 3,
            Mul => 3,
            Input => 1,
            Output => 1,
            JumpIfTrue => 2,
            JumpIfFalse => 2,
            LessThan => 3,
            Equal => 3,
            SetRBOffset => 1,
            Halt => 0,
        }
    }
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
            SetRBOffset => "RBOFFSET",
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
            9 => Ok(SetRBOffset),
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
            SetRBOffset => 9,
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

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Exception::*;
        f.write_str(&match &self {
            Halt => String::from("Halt"),
            IllegalInstruction(word) => format!("Illegal instruction {}", word),
            SegmentationFault(addr) => format!("Segmentation fault at {:08x}", addr),
            Input => String::from("Input required"),
            Output(word) => format!("Output {}", word),
        })
    }
}
