use std::path::Path;
use std::fs;
use std::collections::VecDeque;

type Circle = VecDeque<u32>;

const MAX_LABEL: u32 = 9;
const N_CUPS: usize = 9;

fn main() {
    let input = read_input("input.txt");
    assert_eq!(input.len(), N_CUPS);

    println!("Part 1: {}", cups_to_str(&part1(&input, 100)));
}

fn read_input<T: AsRef<Path>>(path: T) -> Circle {
    let input = fs::read_to_string(path).expect("Failed to read input");

    input.trim().chars()
        .map(|c| c.to_digit(10).expect("Failed to parse number"))
        .collect()
}

fn part1(cups: &Circle, turns: usize) -> Circle {
    let mut cups = cups.clone();

    // First cup is the current cup
    let mut current = 0;

    for n in 1..=turns {
        println!("-- move {} --", n);
        println!("cups: {:?}", cups);

        let current_label = cups[current];
        println!("current: {}", current_label);

        // Pick up 3 cups immediately clockwise of the current cup
        // These are removed from the circle.
        let pickedup = pickup(&mut cups, current, 3);
        println!("pick up: {:?}", pickedup);

        // Select a destination cup - the one with label minus 1
        // Skip cups that have been picked up and the labels wrap around
        let destination = find_destination_cup(&cups, current_label);

        // Place the picked up cups immediately clockwise of the destination cup
        place(&mut cups, &pickedup, destination);

        // Select a new current cup - the one immediately clockwise of the current cup
        current = (find_cup(&cups, current_label) + 1) % cups.len();
        println!();
    }

    println!("-- final --");
    println!("cups: {:?}", cups);

    // Cups after cup 1
    let cup1 = find_cup(&cups, 1);
    cups.into_iter().cycle().skip(cup1 + 1).take(N_CUPS - 1).collect()
}

/// Find the position of the cup with `label`.
fn find_cup(cups: &Circle, label: u32) -> usize {
    cups.iter().position(|&l| l == label).expect("Could not find cup")
}

/// Pickup `n` cups immediately clockwise of `index`.
fn pickup(cups: &mut Circle, index: usize, n: usize) -> Vec<u32> {
    let pickedup: Vec<_> = cups.iter().cycle().skip(index + 1).take(n).copied().collect();

    *cups = cups.iter().filter(|c| !pickedup.contains(c)).copied().collect();

    pickedup
}

/// Select a destination cup - the one with label minus 1.
fn find_destination_cup(cups: &Circle, label: u32) -> usize {
    let mut label = label;
    loop {
        label = if label == 1 { MAX_LABEL } else { label - 1 };
        if let Some(p) = cups.iter().position(|&l| l == label) {
            println!("destination: {}", label);
            return p;
        }
    }
}

/// Insert `new_cups` immediately clockwise of `index`.
fn place(cups: &mut Circle, new_cups: &[u32], index: usize) {
    for &l in new_cups.iter().rev() {
        cups.insert(index + 1, l);
    }
}

fn cups_to_str(cups: &Circle) -> String {
    cups.iter().map(|c| c.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input, 10), [9, 2, 6, 5, 8, 3, 7, 4]);
        assert_eq!(part1(&input, 100), [6, 7, 3, 8, 4, 5, 2, 9]);
    }
}

