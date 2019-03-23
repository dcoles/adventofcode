use std::collections::HashMap;
use std::fs;

const N_POTS: usize = 250;
const OFFSET: usize = 10;
const WINDOW: usize = 5;
const GENERATIONS: u64 = 135;

fn main() {
    let mut world = read_input();

    // Part 1
    println!("{:>3} [{:>5}] {:>width$}", "GEN", "SUM", "0", width=OFFSET+1);
    while world.generation < GENERATIONS {
        world.print();
        world.tick();
    }
    world.print();

    // Part 2
    // Empirically we can see that by generation 129 that the plants have reached a steady-state
    // where the pattern shifts right by one each generation.
    //
    // We can use this to get the formula:
    //   sum = (gen + 36) * 52
    //
    // Thus at gen 5,000,000,000 the sum of pots with plants is 2,600,000,001,872.
}

fn read_input() -> World {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let mut state = [false; N_POTS];
    let mut rules = HashMap::new();

    for (n, line) in input.lines().enumerate() {
        if n == 0 {
            let input = parse(&line[15..]);
            state[OFFSET..OFFSET+input.len()].copy_from_slice(&input);
        } else if n >= 2 {
            let mut input = [false; WINDOW];
            input.copy_from_slice(&parse(&line[..5]));
            let output = &line[9..10] == "#";
            rules.insert(input, output);
            //println!("{} -> {}", &line[..5], &line[9..10]);
        }
    }

    World { state, rules, generation: 0 }
}

fn parse(input: &str) -> Vec<bool> {
    input.trim().chars().map(|c| c == '#').collect()
}

struct World {
    state: [bool; N_POTS],
    rules: HashMap<[bool; WINDOW], bool>,
    generation: u64,
}

impl World {
    fn tick(&mut self) {
        let mut state = [false; N_POTS];
        for idx in 0..=N_POTS-WINDOW {
            let mut last_window = [false; WINDOW];
            last_window.copy_from_slice( &self.state[idx..idx+WINDOW]);
            let window = &mut state[idx..idx+WINDOW];
            if let Some(result) = self.rules.get(&last_window) {
                window[2] = *result;
            }
        }
        self.state = state;
        self.generation += 1;
    }

    fn print(&self) {
        let line: Vec<&str> = self.state.iter().map(|&b| if b { "#" } else { "." }).collect();
        println!("{:3} [{:5}] {}", self.generation, self.sum(), line.join(""))
    }

    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for idx in 0..N_POTS {
            if self.state[idx] {
                sum += idx as i32 - OFFSET as i32;
            }
        }
        sum
    }
}
