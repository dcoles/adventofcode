mod emulator;

use std::{fs, env, process, io};
use std::path::Path;
use crate::emulator::{Program, IntcodeEmulator, Exception, Word, MODE_POSITION, MODE_IMMEDIATE};
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

                cpu.add_input(input);
            },
            Exception::Output(out) => {
                println!("{}", out);
            },
            Exception::IllegalInstruction(opcode) => {
                eprintln!("ERROR: Illegal instruction {} (ip: 0x{:08x})", opcode, cpu.ip());
                if debug {
                    attach_debugger(&mut cpu);
                } else {
                    cpu.dump_memory();
                }
                process::exit(4);
            },
            Exception::SegmentationFault(addr) => {
                eprintln!("Segmentation fault at 0x{:08x} (ip: 0x{:08x})", addr, cpu.ip());
                if debug {
                    attach_debugger(&mut cpu);
                } else {
                    cpu.dump_memory();
                }
                process::exit(11);
            },
        }
    }
}

fn read_input() -> Result<Word, String> {
    let mut inbuf = String::new();
    io::stdin().read_line(&mut inbuf).map_err(|err| format!("Failed to read from STDIN: {}", err))?;
    let input = inbuf.trim().parse::<Word>().map_err(|err| format!("Could parse STDIN: {}", err))?;

    Ok(input)
}

fn attach_debugger(cpu: &mut IntcodeEmulator) {
    // Read from TTY, even if STDIN is redirected
    let mut tty = match fs::File::open("/dev/tty") {
        Err(err) => {
            eprintln!("ERROR: Could not open TTY: {}", err);
            return;
        },
        Ok(file) => io::BufReader::new(file),
    };

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

        let args: Vec<_> = line.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        if let Err(err) = match args[0] {
            "p" | "print" => print(&cpu, &args),
            "c" | "continue" => break,
            "j" | "jump" => read_param(&args, 1).map(|addr| cpu.set_ip(addr)),
            "q" | "quit" => process::exit(0),
            "d" | "disassemble" => disassemble(&cpu).map(|s| eprintln!("{:08x} {}", cpu.ip(), s)),
            "s" | "step" => {
                match cpu.step() {
                    Err(Exception::Input) => read_input().map(|input| cpu.add_input(input)),
                    Err(Exception::Output(val)) => {
                        println!("{}", val);
                        Ok(())
                    },
                    Err(except) => Err(except.to_string()),
                    Ok(()) => Ok(()),
                }.map(|_| eprintln!("{:08x} {}", cpu.ip(), disassemble(&cpu).unwrap_or_else(|_| String::from("???"))))
            },
            "i" | "input" => read_param(&args,1).map(|input| cpu.add_input(input)),
            "dump" => { cpu.dump_memory(); Ok(()) },
            "h" | "help" => {
                eprintln!("p|print [addr]  Print contents of address");
                eprintln!("c|continue      Continue execution");
                eprintln!("q|quit          Exit debugger and terminate program");
                eprintln!("d|disassemble   Disassemble current instruction");
                eprintln!("s|step          Step to the next instruction");
                eprintln!("i|input         Write input to the CPU");
                eprintln!("dump            Dump memory to console");
                eprintln!("h|help          Print this help");
                Ok(())
            },
            arg => Err(format!("ERROR: Unknown command '{}'", arg))
        } {
            eprintln!("ERROR: {}", err);
        };
    }
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

    if arg1.starts_with("$") {
        // p $ip
        match &arg1[1..] {
            "ip" => eprintln!("0x{:08}", cpu.ip()),
            name => Err(format!("Unknown register %{}", name))?,
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

        let value = cpu.mem().get(addr).ok_or_else(|| format!("Address out of range"))?;
        eprintln!("{}", value);
    }

    Ok(())
}

fn disassemble(cpu: &IntcodeEmulator) -> Result<String, String> {
    let addr = cpu.ip();
    let instruction = cpu.current_instruction().map_err(|err| format!("Failed to decode instruction: {}", err))?;
    let nparams = instruction.op().nparams();
    let params: Vec<_> = cpu.mem()[addr+1..].iter()
        .chain([0].iter().cycle())
        .take(nparams)
        .enumerate()
        .map(|(n, &p)| (instruction.mode_for(nparams - n), p))
        .collect();

    let params_str: Vec<_> = params.iter().map(|&(m, p)| {
        match m {
            MODE_POSITION => format!("0x{:08x}", p),
            MODE_IMMEDIATE => format!("${}", p),
            _ => format!("?{}", p),
        }
    }).collect();

    Ok(format!("{} {}", instruction.op(), params_str.join(" ")))
}

struct Args {
    debug: bool,
    break_at_start: bool,
    program: String,
}
