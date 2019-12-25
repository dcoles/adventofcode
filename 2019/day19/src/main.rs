use intcode::emulator::{Program, IntcodeEmulator, Word, Context};
use std::collections::VecDeque;
use std::io;
use std::cell::Cell;
use std::rc::Rc;

// Drone status
const INVALID: Word = -1;
const PULLED: Word = 1;

// Starting must be at least height of the ship
const MIN_Y: Word = HEIGHT - 1;

// Size of Santa's ship
const WIDTH: Word = 100;
const HEIGHT: Word = 100;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let mut pulled = 0;
    for y in 0..50 {
        for x in 0..50 {
            if scan(&program, x, y) == PULLED {
                pulled += 1;
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!("Part 1: Number of points affected by tractor beam: {}", pulled);

    // Part 2
    let (x0, y0) = fit(&program, WIDTH, HEIGHT);

    // Show the box
    for y in y0-5..y0+HEIGHT+5  {
        print!("{:5} ", y);
        for x in x0-5..x0+WIDTH+5 {
            if (x0..x0+WIDTH).contains(&x) && (y0..y0+HEIGHT).contains(&y) {
                assert_eq!(scan(&program, x, y), PULLED);
                print!("O");
            } else if scan(&program, x, y) == PULLED {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Part 2: Top left coords: {:?} (answer: {})", (x0, y0), x0 * 10_000 + y0);
}

fn fit(program: &Program, min_width: Word, min_height: Word) -> (Word, Word) {
    assert!(min_width > 1);
    assert!(min_height > 1);

    let mut top = 0;
    let mut left = 0;
    for y in MIN_Y.. {

        // Find the min-x side of the beam (moves right as y increases)
        for x in left.. {
            if scan(program, x, y) == PULLED {
                left = x;
                break
            }
        }

        // Check the height of the column above this position
        // If we've found a column > min_height all the following rows will be too!
        if top == 0 {
            if scan(program, left, y - (min_height - 1)) != PULLED {
                // Not tall enough
                continue;
            }
        }
        top = y + 1 - min_width;

        // Check the width from the top of the column
        if scan(program, left + min_width - 1, top) != PULLED {
            // Not wide enough
            continue;
        }

        // Found!
        break;
    }

    (left, top)
}

fn scan(program: &Program, x: Word, y: Word) -> Word {
    assert!(x >= 0);
    assert!(y >= 0);
    let mut input: VecDeque<Word> = [x, y].iter().copied().collect();
    let input_handler = Box::new(move |_: &mut Context| {
        input.pop_front()
            .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "No more input!"))
    });

    let output = Rc::new(Cell::new(INVALID));
    let output_ = Rc::clone(&output);
    let output_handler = Box::new(move |_: &mut Context, word| {
        output_.set(word);
        Ok(())
    });

    let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
    cpu.load_program(&program);
    cpu.run().expect("Failed to run program");

    output.get()
}
