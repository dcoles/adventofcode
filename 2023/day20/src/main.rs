//! Advent of Code 2023: Day 20 "Pulse Propagation"
//! https://adventofcode.com/2023/day/20

use std::collections::{HashMap, VecDeque};
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    input.show_graphviz();

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

const N: usize = 1000;
const BUTTON_MODULE: &str = "button";
const BROADCASTER_MODULE: &str = "broadcaster";

fn part1(input: &Input) -> usize {
    let mut network = ModuleNetwork::new(input.modules.clone());

    for _ in 1..=N {
        network.press_button();
    }

    let high_pulses: usize = network.high_pulses().values().sum();
    let low_pulses: usize = network.low_pulses().values().sum();

    high_pulses * low_pulses
}

fn part2(input: &Input) -> usize {
    // Note: This only works for a network with a set of flip-flop chain matchers
    // (this can be seen in the graphviz display of the input)

    // Find the flip-flop chains
    let mut flipflop_chains = vec![];
    for name in &input.modules[BROADCASTER_MODULE].targets {
        let mut cur = name;

        let mut chain = vec![cur];
        while let Some(next) = input.modules[cur].targets.iter().find(|&m| input.modules[m].is_flipflop()) {
            chain.push(next);
            cur = next;
        }

        flipflop_chains.push(chain);
    }

    // Determine the period detected by the conjunction module
    let periods: Vec<_> = flipflop_chains.into_iter().map(|chain| {
        chain.into_iter()
        .rev()
        .fold(0, |acc, k| (acc << 1) + if input.module_targets_conjunction(&k) { 1 } else { 0 } )
    }).collect();

    periods.into_iter().product()
}


struct ModuleNetwork {
    modules: HashMap<String, Module>,
    flip_flops: HashMap<String, bool>,
    conjunctions: HashMap<String, HashMap<String, Pulse>>,
    low_pulses: HashMap<String, usize>,
    high_pulses: HashMap<String, usize>,
}

impl ModuleNetwork {
    fn new(modules: HashMap<String, Module>) -> Self {
        let mut flip_flops: HashMap<String, bool> = HashMap::new();
        let mut conjunctions: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
        let low_pulses: HashMap<String, usize> = HashMap::new();
        let high_pulses: HashMap<String, usize> = HashMap::new();

        // Initialize flip flops
        for (name, _) in modules.iter().filter(|(_, m)| m.is_flipflop()) {
            flip_flops.insert(name.to_owned(), false);
        }

        // Initialize conjunction
        for (name, module) in &modules {
            for target in module.targets.iter().filter(|t| modules.get(t.as_str()).map(|m| m.is_conjunction()).unwrap_or_default()) {
                let target = target.to_owned();
                let name = name.to_owned();
                conjunctions.entry(target).or_default().insert(name, Pulse::Low);
            }
        }

        Self {
            modules,
            flip_flops,
            conjunctions,
            low_pulses,
            high_pulses,
        }
    }

    fn high_pulses(&self) -> &HashMap<String, usize> {
        &self.high_pulses
    }

    fn low_pulses(&self) -> &HashMap<String, usize> {
        &self.low_pulses
    }

    fn press_button(&mut self) {
        // Initial pulse is low
        *self.low_pulses.entry(BROADCASTER_MODULE.to_owned()).or_default() += 1;

        let mut queue: VecDeque<_> = [(BUTTON_MODULE, BROADCASTER_MODULE, Pulse::Low)].into_iter().collect();
        while let Some((from, to, pulse_in)) = queue.pop_front() {
            if !self.modules.contains_key(to) {
                continue;
            }

            let module = &self.modules[to];
            let pulse_out = match module.module_type {
                ModuleType::Broadcaster => Some(pulse_in),
                ModuleType::FlipFlop => {
                    let state = self.flip_flops.entry(to.to_owned()).or_default();
                    if matches!(pulse_in, Pulse::Low) {
                        // Flip state
                        *state = !*state;

                        // If now on, then output High otherwise output Low
                        Some(if *state { Pulse::High } else { Pulse::Low })
                    } else {
                        None
                    }
                },
                ModuleType::Conjunction => {
                    let memory = self.conjunctions.entry(to.to_owned()).or_default();
                    memory.insert(from.to_owned(), pulse_in);

                    // If all past inputs are High, output Low otherwise output High
                    Some(if memory.values().all(|&p| matches!(p, Pulse::High)) { Pulse::Low } else { Pulse::High })
                },
            };

            if let Some(pulse_out) = pulse_out {
                for target in &module.targets {
                    match pulse_out {
                        Pulse::High => *self.high_pulses.entry(target.to_owned()).or_default() += 1,
                        Pulse::Low => *self.low_pulses.entry(target.to_owned()).or_default() += 1,
                    }

                    queue.push_back((to, target, pulse_out));
                }
            }
        }

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    targets: Vec<String>,
}

impl Module {
    fn is_flipflop(&self) -> bool {
        matches!(self.module_type, ModuleType::FlipFlop)
    }

    fn is_conjunction(&self) -> bool {
        matches!(self.module_type, ModuleType::Conjunction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

impl ModuleType {
    fn from_str(s: &str) -> (Self, String) {
        match s {
            "broadcaster" => (Self::Broadcaster, s.to_owned()),
            ff if ff.starts_with('%') => (Self::FlipFlop, ff[1..].to_owned()),
            c if c.starts_with('&') => (Self::Conjunction, c[1..].to_owned()),
            _ => panic!("unknown module type {s:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    modules: HashMap<String, Module>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut modules = HashMap::new();
        for line in input.lines() {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let (module_type, name) = ModuleType::from_str(lhs);
            let targets: Vec<_> = rhs.split(", ").map(str::to_string).collect();

            modules.insert(name, Module { module_type, targets });
        }

        Ok(Self { modules })
    }

    fn module_targets_conjunction(&self, name: &str) -> bool {
        self.modules[name].targets.iter().any(|n| matches!(self.modules[n].module_type, ModuleType::Conjunction))
    }

    /// Show graphviz representation of input.
    /// Usage: `dot -T png -o day20.png day20.dot`
    fn show_graphviz(&self) {
        println!("digraph {{");
        for (name, module) in &self.modules {
            match module.module_type {
                ModuleType::Broadcaster => println!("  {name} [shape=Mdiamond]"),
                ModuleType::FlipFlop => println!("  {name} [shape=box]"),
                ModuleType::Conjunction => {
                    if module.targets.len() == 1 {
                        // Inverter
                        println!("  {name} [shape=diamond color=red]")
                    } else {
                        println!("  {name} [shape=circle color=blue]")
                    }
                },
            }

            for target in &module.targets {
                println!("  {name} -> {target}")
            }
        }
        println!("}}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 32000000);
    }

    #[test]
    fn test_part1_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 11687500);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 949764474);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 243221023462303);
    }
}
