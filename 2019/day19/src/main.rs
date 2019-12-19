use intcode::emulator::{Program, IntcodeEmulator, Word};
use std::collections::VecDeque;
use std::io;
use std::cell::Cell;
use std::rc::Rc;

// Drone status
const INVALID: Word = -1;
const PULLED: Word = 1;

// Starting distance to ensure beam dispersal
const MIN_Y: Word = 10;

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
    let mut width = 0;  // Keeps track of the maximum beam width seen
    let mut height = 0;  // Keeps track of the maximum beam height seen
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
            height += (0..=y-height).rev().take_while(|&y| scan(program, left, y) == PULLED).count() as Word;
            if height < min_height {
                // Not tall enough
                continue;
            }
        }
        top = y + 1 - min_width;

        // Check the width from the top of the column
        width += (left+width..).take_while(|&x| scan(program, x, top) == PULLED).count() as Word;
        if width < min_width {
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
