mod emulator;

use std::{fs, env, process, io};
use std::path::Path;
use crate::emulator::{Program, IntcodeEmulator, Exception, Word};
use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
    let args = parse_args();

    let program = match read_input(args.program) {
        Err(err) => {
            eprintln!("ERROR: {}", err);
            process::exit(1);
        },
        Ok(program) => program,
    };

    run(&program, args.debug);
}

fn parse_args() -> Args {
    let mut debug = false;
    let mut posargs = VecDeque::new();

    let args: Vec<_> = env::args().collect();
    for arg in &args[1..] {
        match arg.as_str() {
            "-d" | "--debug" => debug = true,
            arg if arg.starts_with("-") => {
                eprintln!("ERROR: Unknown argument '{}'", arg);
                print_usage();
                process::exit(2);
            },
            arg => posargs.push_back(arg),
        }
    }

    let program = if let Some(arg) = posargs.pop_front() {
        arg.to_owned()
    } else {
        print_usage();
        process::exit(2);
    };

    if !posargs.is_empty() {
        print_usage();
        process::exit(2)
    }

    Args { debug, program }
}

fn print_usage() {
    eprintln!("USAGE: intcode [-d | --debug] PROGRAM");
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

fn run(program: &Program, debug: bool) {
    let mut cpu = IntcodeEmulator::new();
    cpu.load_program(&program);
    cpu.set_debug(debug);

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
                eprintln!("ERROR: Illegal instruction {} (ip: 0x{:08x})", opcode, cpu.ip());
                cpu.dump_memory();
                process::exit(4);
            },
            Exception::SegmentationFault(addr) => {
                eprintln!("Segmentation fault at 0x{:08x} (ip: 0x{:08x})", addr, cpu.ip());
                cpu.dump_memory();
                process::exit(11);
            },
        }
    }
}

struct Args {
    debug: bool,
    program: String,
}
