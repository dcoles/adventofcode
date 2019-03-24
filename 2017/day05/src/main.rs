use std::fs::read_to_string;
use std::path::Path;
use std::io;

fn main() {
    let input = read_input("input.txt").expect("Failed to read input");
    println!("[Part 1] Exits in {} steps", run1(&input));
    println!("[Part 2] Exits in {} steps", run2(&input));
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<isize>> {
    Ok(read_to_string(path)?.lines().filter_map(|v| v.parse().ok()).collect())
}

fn run1(instructions: &Vec<isize>) -> u32 {
    let mut instructions = instructions.clone();
    let mut ip = 0;
    let mut step = 1;
    loop {
        let next_ip = ip as isize + instructions[ip];
        instructions[ip] += 1;

        if next_ip >= instructions.len() as isize || next_ip < 0 {
            return step;
        }

        ip = next_ip as usize;
        step += 1;
    }
}

fn run2(instructions: &Vec<isize>) -> u32 {
    let mut instructions = instructions.clone();
    let mut ip = 0;
    let mut step = 1;
    loop {
        let next_ip = ip as isize + instructions[ip];
        if instructions[ip] >= 3 {
            instructions[ip] -= 1;
        } else {
            instructions[ip] += 1;
        }

        if next_ip >= instructions.len() as isize || next_ip < 0 {
            return step;
        }

        ip = next_ip as usize;
        step += 1;
    }
}
