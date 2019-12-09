mod emulator;

use std::{fs, env, process, io};
use std::path::Path;
use crate::emulator::{Program, IntcodeEmulator, Exception, Word};
use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
    let args = parse_args();

    let program = match read_from_file(args.program) {
        Err(err) => {
            eprintln!("ERROR: {}", err);
            process::exit(1);
        },
        Ok(program) => program,
    };

    run(&program, args.debug, args.break_at_start);
}

fn parse_args() -> Args {
    let mut debug = false;
    let mut break_at_start = false;
    let mut posargs = VecDeque::new();

    let args: Vec<_> = env::args().collect();
    for arg in &args[1..] {
        match arg.as_str() {
            "-d" | "--debug" => debug = true,
            "-B" | "--break" => break_at_start = true,
            arg if arg.starts_with('-') => {
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

    Args { debug, break_at_start, program }
}

fn print_usage() {
    eprintln!("USAGE: intcode [-d | --debug] [-B | --break] PROGRAM");
}

fn read_from_file<T: AsRef<Path>>(path: T) -> Result<Program, String> {
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

fn run(program: &Program, debug: bool, break_at_start: bool) {
    let mut cpu = IntcodeEmulator::new();
    cpu.load_program(&program);
    cpu.set_debug(debug);

    if break_at_start {
        attach_debugger(&mut cpu);
    }

    loop {
        match cpu.run() {
            Exception::Halt => break,
            Exception::Input => {
                let input = match read_input() {
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        process::exit(1);
                    },
                    Ok(val) => val,
                };

                for val in input {
                    cpu.add_input(val);
                }
            },
            Exception::Output(out) => {
                println!("{}", out);
            },
            Exception::IllegalInstruction(opcode) => {
                eprintln!("ERROR: Illegal instruction {}", opcode);
                if debug {
                    attach_debugger(&mut cpu);
                } else {
                    cpu.dump_registers();
                    cpu.print_disassembled();
                    cpu.dump_memory();
                }
                process::exit(4);
            },
            Exception::SegmentationFault(addr) => {
                eprintln!("Segmentation fault at 0x{:08x}", addr);
                if debug {
                    attach_debugger(&mut cpu);
                } else {
                    cpu.dump_registers();
                    cpu.print_disassembled();
                    cpu.dump_memory();
                }
                process::exit(11);
            },
        }
    }
}

fn read_input() -> Result<Vec<Word>, String> {
    let mut inbuf = String::new();
    if io::stdin().read_line(&mut inbuf).map_err(|err| format!("Failed to read from stdin: {}", err))? == 0 {
        return Err(String::from("stdin reached EOF"))
    }
    let input: Result<Vec<_>, _> = inbuf.trim().split_whitespace()
        .map(|s| s.parse::<Word>().map_err(|err| format!("Could parse stdin: {}", err)))
        .collect();

    Ok(input?)
}

fn attach_debugger(cpu: &mut IntcodeEmulator) {
    // Read from TTY, even if stdin is redirected
    let mut tty = match fs::File::open("/dev/tty") {
        Err(err) => {
            eprintln!("ERROR: Could not open TTY: {}", err);
            return;
        },
        Ok(file) => io::BufReader::new(file),
    };

    // Disable debug-tracing while running the debugger
    let last_debug = cpu.get_debug();
    cpu.set_debug(false);

    // Disassemble first instruction
    cpu.print_disassembled();

    let mut last_line = String::new();
    loop {
        eprint!("debug> ");
        let mut line = String::new();
        match tty.read_line(&mut line) {
            Err(err) => {
                eprintln!("ERROR: Failed to read input: {}", err);
                continue
            }
            Ok(nbytes) if nbytes == 0 => break,
            Ok(_) => (),
        }

        // Keep track of the last non-empty line to allow easy repeat
        if line.trim().is_empty() {
            line = last_line.clone();
        } else {
            last_line = line.clone();
        }

        let args: Vec<_> = line.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        if let Err(err) = match args[0] {
            "p" | "print" => print(&cpu, &args),
            "c" | "continue" => break,
            "j" | "jump" => {
                read_param(&args, 1)
                    .map(|addr| cpu.set_ip(addr))
                    .map(|_| cpu.print_disassembled())
            },
            "r" | "relbase" => read_param(&args, 1).map(|word| cpu.set_rb(word)),
            "q" | "quit" => process::exit(0),
            "d" | "disassemble" => { cpu.print_disassembled(); Ok(()) },
            "s" | "step" => {
                match cpu.step() {
                    Err(Exception::Input) => read_input().map(|input| for val in input { cpu.add_input(val) }),
                    Err(Exception::Output(val)) => {
                        println!("{}", val);
                        Ok(())
                    },
                    Err(except) => Err(except.to_string()),
                    Ok(()) => Ok(()),
                }.map(|_| cpu.print_disassembled())
            },
            "i" | "input" => read_param(&args,1).map(|input| cpu.add_input(input)),
            "D" | "dump" => { cpu.dump_memory(); Ok(()) },
            "h" | "help" => {
                eprintln!("p|print [ ADDR | $ip | $rb ]");
                eprintln!("                Print contents of address");
                eprintln!("c|continue      Continue execution");
                eprintln!("q|quit          Exit debugger and terminate program");
                eprintln!("d|disassemble   Disassemble current instruction");
                eprintln!("s|step          Step to the next instruction");
                eprintln!("i|input         Write input to the CPU");
                eprintln!("D|dump          Dump memory to console");
                eprintln!("h|help          Print this help");
                Ok(())
            },
            arg => Err(format!("ERROR: Unknown command '{}'", arg))
        } {
            eprintln!("ERROR: {}", err);
        };
    }

    // Re-enable debug-tracing if it was previously enabled
    cpu.set_debug(last_debug);
}

fn read_param<T: std::str::FromStr>(args: &[&str], param: usize) -> Result<T, String> {
    let arg = args.get(param).ok_or_else(|| String::from("Missing parameter"))?;

    arg.parse::<T>().map_err(|_| format!("Failed to parse parameter {}", param))
}

fn print(cpu: &IntcodeEmulator, args: &[&str]) -> Result<(), String> {
    if args.len() > 2 {
        return Err(String::from("Too many arguments"));
    }

    let arg1 = args.get(1).unwrap_or(&"");

    if arg1.starts_with('$') {
        // p $ip
        match &arg1[1..] {
            "ip" => eprintln!("0x{:08x}", cpu.ip()),
            "rb" => eprintln!("{}", cpu.rb()),
            name => return Err(format!("Unknown register %{}", name)),
        }
    } else {
        // p [addr]
        let addr = match arg1 {
            arg if arg.is_empty() => Ok(cpu.ip()),  // Default to $ip
            arg => {
                match arg {
                    arg if arg.starts_with("0x") => usize::from_str_radix(&arg[2..], 16),
                    arg => arg.parse::<usize>(),
                }.map_err(|err| format!("Could not parse address: {}", err))
            },
        }?;

        let value = cpu.mem().get(addr).ok_or_else(|| String::from("Address out of range"))?;
        eprintln!("{}", value);
    }

    Ok(())
}

struct Args {
    debug: bool,
    break_at_start: bool,
    program: String,
}
