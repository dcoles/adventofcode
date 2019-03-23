use std::fs::read_to_string;
use std::path::Path;
use std::io::Error;

fn main() {
    let input = parse_input("input.txt").unwrap();

    // Part 1
    let mut sum = 0;
    for idx in 0..input.len() {
        if input[idx] == input[(idx + 1) % input.len()] {
            sum += input[idx];
        }
    }
    println!("Part 1: Sum = {}", sum);

    // Part 2
    let mut sum = 0;
    for idx in 0..input.len() {
        if input[idx] == input[(idx + input.len() / 2) % input.len()] {
            sum += input[idx];
        }
    }
    println!("Part 2: Sum = {}", sum);
}

fn parse_input<P: AsRef<Path>>(path: P) -> Result<Vec<u32>, Error> {
    let input = read_to_string(path)?;

    Ok(input.chars().filter_map(|c| c.to_digit(10)).collect())
}
