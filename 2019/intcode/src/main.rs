mod emulator;

use std::{fs, env, process, io};
use std::path::Path;
use crate::emulator::{Program, IntcodeEmulator, Exception, Word};
use std::io::BufRead;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: No source file provided");
        process::exit(2);
    }

    let program = match read_input(&args[1]) {
        Ok(prog) => prog,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            process::exit(1);
        },
    };

    run(&program);
}

fn read_input<T: AsRef<Path>>(path: T) -> Result<Program, String> {
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

fn run(program: &Program) {
    let mut cpu = IntcodeEmulator::new();
    cpu.load_program(&program);

    loop {
        match cpu.run() {
            Exception::Halt => break,
            Exception::Input => {
                let mut inbuf = String::new();
                if let Err(err) = io::stdin().read_line(&mut inbuf) {
                    eprintln!("ERROR: Failed to read from STDIN: {}", err);
                    process::exit(1);
                };
                let input = match inbuf.trim().parse::<Word>() {
                    Err(err) => {
                        eprintln!("ERROR: Could parse STDIN: {}", err);
                        process::exit(1);
                    },
                    Ok(input) => input,
                };
                cpu.add_input(input);
            },
            Exception::Output(out) => {
                println!("{}", out);
            },
            Exception::IllegalInstruction(opcode) => {
                eprintln!("ERROR: Illegal instruction {} at {:08x}", opcode, cpu.ip());
                cpu.dump_memory();
                process::exit(4);
            },
            Exception::SegmentationFault(addr) => {
                eprintln!("Segmentation fault at {:08x}", addr);
                cpu.dump_memory();
                process::exit(11);
            },
        }
    }
}
