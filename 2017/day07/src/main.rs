extern crate regex;

use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;
use std::io;
use regex::Regex;

fn main() {
    let mut input = read_input("input.txt").expect("Failed to read input");
    let keys: Vec<String> = input.keys().map(|k| k.to_string()).collect();

    let mut bits = HashMap::new();
    for name in &keys {
        if input.contains_key(name) {
            let program = add_program(name, &mut input, &mut bits);
            bits.insert(name.to_string(), program);
        }
    }

    // Part 1
    let (name, program) = bits.iter().last().unwrap();
    println!("The bottom program's name is {}", name);

    // Part 2
    check_weight(program);
}

fn check_weight(program: &Program) {
    println!("{} (total_weight: {})", program, program.total_weight());

    for child in &program.children {
        println!("- {} ({}) (total_weight:{})", child.name, child.weight, child.total_weight());
    }

    let count = program.children.iter().filter(|p| p.total_weight() == program.children[0].total_weight()).count();
    if count == program.children.len() {
        // Balanced
        return;
    } else if count == 1 {
        check_weight(&program.children[0]);
    } else {
        check_weight(&program.children.iter().filter(|p| p.total_weight() != program.children[0].total_weight()).last().unwrap());
    }
}

fn add_program(name: &str, input: &mut HashMap<String, (i32, Vec<String>)>, bits: &mut HashMap<String, Program>) -> Program {
    let (weight, children) = input.remove(name).unwrap();
    let mut program = Program::new(&name, weight);

    for child_name in &children {
        program.add_child(if let Some(child) = bits.remove(child_name) {
            child
        } else {
            add_program(child_name, input, bits)
        })
    }

    program
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, (i32, Vec<String>)>> {
    let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();

    let mut result = HashMap::new();
    for line in read_to_string(path)?.lines() {
        if let Some(cap) = re.captures(line) {
            let name = &cap[1];
            let weight: i32 = cap[2].parse().unwrap();
            let children: Vec<String> = if let Some(m) = cap.get(3) {
                m.as_str().split(", ").map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            };
            result.insert(name.to_string(), (weight, children));
        }
    }

    Ok(result)
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: i32,
    children: Vec<Box<Program>>,
    cached_total_weight: Cell<i32>,
}

impl Program {
    fn new(name: &str, weight: i32) -> Program {
        Program { name: name.to_string(), weight, children: Vec::new(), cached_total_weight: Cell::new(-1) }
    }

    fn add_child(&mut self, child: Program) {
        self.children.push(Box::new(child));
    }

    fn total_weight(&self) -> i32 {
        if self.cached_total_weight.get() != -1 {
            return self.cached_total_weight.get();
        }

        let total_weight = self.weight + self.children.iter().map(|c| c.total_weight()).sum::<i32>();
        self.cached_total_weight.set(total_weight);
        total_weight
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let children: Vec<&str> = self.children.iter().map(|p| &p.name[..]).collect();
        write!(f, "{} ({}) -> {}", self.name, self.weight, children.join(", "))
    }
}
