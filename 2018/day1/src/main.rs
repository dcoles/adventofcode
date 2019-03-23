use std::fs;
use std::collections;

fn main() {
    let values = parse_input();

    // Part 1
    let freq: i32 = values.iter().sum();
    println!("Summed frequency: {}Hz", freq);

    // Part 2
    let mut freq = 0;
    let mut seen = collections::HashSet::new();
    for val in values.iter().cycle() {
        freq += val;
        if seen.contains(&freq) {
            break;
        };
        seen.insert(freq);
    }

    println!("First repeated frequency: {}Hz", freq);
}

fn parse_input() -> Vec<i32> {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut values = Vec::new();
    for line in contents.lines() {
        let val: i32 = line.trim().parse()
            .expect("Could not parse value");
        values.push(val);
    }
    return values;
}
