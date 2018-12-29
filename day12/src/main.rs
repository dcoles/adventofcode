use std::fs;

const N_POTS: usize = 300;
const OFFSET: usize = 100;
const WINDOW: usize = 5;

fn main() {
    let mut world = read_input();

    // Part 1
    while world.generation < 20 {
        world.print();
        world.tick();
    }
    world.print();

    println!("Sum of pots with plants: {}", world.sum());
}

fn read_input() -> World {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let mut state = [false; N_POTS];
    let mut rules = Vec::new();

    for (n, line) in input.lines().enumerate() {
        if n == 0 {
            let input = parse(&line[15..]);
            state[OFFSET..OFFSET+input.len()].copy_from_slice(&input);
        } else if n >= 2 {
            let mut input = [false; WINDOW];
            input.copy_from_slice(&parse(&line[..5]));
            let output = &line[9..10] == "#";
            rules.push((input, output));
            println!("{} -> {}", &line[..5], &line[9..10]);
        }
    }

    World { state, rules, generation: 0 }
}

fn parse(input: &str) -> Vec<bool> {
    input.trim().chars().map(|c| c == '#').collect()
}

struct World {
    state: [bool; N_POTS],
    rules: Vec<([bool; WINDOW], bool)>,
    generation: u32,
}

impl World {
    fn tick(&mut self) {
        let mut state = [false; N_POTS];
        for idx in 0..=N_POTS-WINDOW {
            let last_window = &self.state[idx..idx+WINDOW];
            let window = &mut state[idx..idx+WINDOW];
            for (rule, result) in &self.rules {
                if last_window == rule {
                    window[2] = *result;
                    break
                }
            }
        }
        self.state = state;
        self.generation += 1;
    }

    fn print(&self) {
        let line: Vec<&str> = self.state.iter().map(|&b| if b { "#" } else { "." }).collect();
        println!("{:2}: {}", self.generation, line.join(""))
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
