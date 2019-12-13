use intcode::emulator::{Program, IntcodeEmulator, Exception, Word};
use std::collections::{VecDeque, HashMap};

const BLOCK: Word = 2;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");
    let mut cpu = IntcodeEmulator::new();
    cpu.load_program(&program);

    let mut grid: HashMap<(Word, Word), Word> = HashMap::new();
    let mut input_queue = VecDeque::new();
    loop {
        match cpu.run() {
            Exception::Halt => break,
            Exception::Output(word) => {
                input_queue.push_back(word);
                if input_queue.len() < 3 {
                    continue;
                }

                let x = input_queue.pop_front().unwrap();
                let y = input_queue.pop_front().unwrap();
                let tile_id = input_queue.pop_front().unwrap();
                grid.insert((x, y), tile_id);
            },
            exception => panic!("Unhandled exceptionL {}", exception),
        }
    }

    let n_block = grid.values().filter(|&&v| v == BLOCK).count();
    println!("Part 1: Tiles on screen: {}", n_block);
}
