use intcode::emulator::{Program, IntcodeEmulator, Word};
use std::collections::VecDeque;
use std::io;
use std::cell::Cell;
use std::rc::Rc;

const INVALID: Word = -1;
const PULLED: Word = 1;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let mut pulled = 0;
    for x in 0..50 {
        for y in 0..50 {
            if scan(&program, x, y) == PULLED {
                pulled += 1;
            }
        }
    }
    println!("Part 1: Number of points affected by tractor beam: {}", pulled);
}

fn scan(program: &Program, x: Word, y: Word) -> Word {
    let mut input: VecDeque<Word> = [x, y].iter().copied().collect();
    let input_handler = Box::new(move || {
        input.pop_front()
            .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "No more input!"))
    });

    let output = Rc::new(Cell::new(INVALID));
    let output_ = Rc::clone(&output);
    let output_handler = Box::new(move |word| {
        output_.set(word);
        Ok(())
    });

    let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
    cpu.load_program(&program);
    cpu.run();

    output.get()
}
