use std::fs;
use std::path::Path;

const OP_ADD: usize = 1;  // r[x3] = r[x1] + r[x2]
const OP_MUL: usize = 2;  // r[x3] = r[x1] * r[x2]
const OP_HALT: usize = 99;

fn main() {
    let input = read_input("input.txt");

    // Part 1
    assert_eq!(3500, test(&vec![1,9,10,3,2,3,11,0,99,30,40,50]));

    let mut program = input.clone();
    program[1] = 12;
    program[2] = 2;
    run(&mut program);
    println!("Part 1: Position 0 = {}", program[0]);

    // Part 2
    let target = 19690720;
    'outer: for noun in 0..=99usize {
        for verb in 0..=99usize {
            let mut program = input.clone();
            program[1] = noun;
            program[2] = verb;
            run(&mut program);

            if program[0] == target {
                let answer = 100 * noun + verb;
                println!("Part 2: Inputs {}, {} give {} (answer: {})", noun, verb, target, answer);
                break 'outer;
            }
        }
    }
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<usize> {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.trim().split(",").map(|line| line.parse::<usize>().expect("Failed to parse input")).collect()
}

fn run(program: &mut Vec<usize>) {
    for ip in (0..program.len()).step_by(4) {
        let (op, x1, x2, x3) = (program[ip], program[ip+1], program[ip+2], program[ip+3]);
        match op {
            OP_ADD => program[x3] = program[x1] + program[x2],
            OP_MUL => program[x3] = program[x1] * program[x2],
            OP_HALT => break,
            _ => panic!("Unknown opcode {}", op),
        }
    }
}

fn test(program: &Vec<usize>) -> usize {
    let mut program = program.clone();
    run(&mut program);

    program[0]
}
