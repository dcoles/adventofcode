use std::path::Path;
use std::fs;
use std::collections::HashMap;

type Label = u32;

fn main() {
    let cups = read_input("input.txt");

    println!("Part 1: {}", cups_as_string(&part1(&cups, 100)));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Label> {
    let input = fs::read_to_string(path).expect("Failed to read input");

    input.trim().chars()
        .map(|c| c.to_digit(10).expect("Failed to parse number"))
        .collect()
}

fn part1(cups: &[Label], turns: usize) -> Vec<Label> {
    let mut circle = CupCircle::new(cups);

    // First cup is the current cup
    let mut current = cups[0];

    for n in 1..=turns {
        println!("-- move {} --", n);
        println!("cups: {:?}", circle.cups_from(current));
        println!("current: {}", current);

        // Pick up 3 cups immediately clockwise of the current cup
        // These are removed from the circle.
        let pickup = circle.remove(current, 3);
        println!("pick up: {:?}", pickup);

        // Select a destination cup - the one with label minus 1
        // Skip cups that have been picked up and the labels wrap around
        let mut destination = current;
        loop {
            destination -= 1;
            if destination == 0 {
                destination = cups.len() as u32;
            }

            if !pickup.contains(&destination) {
                break;
            }
        }

        // Place the picked up cups immediately clockwise of the destination cup
        circle.insert(destination, pickup);

        // Select a new current cup - the one immediately clockwise of the current cup
        current = circle.next_cup(current);
        println!();
    }

    println!("-- final --");
    println!("cups: {:?}", circle.cups_from(current));

    circle.cups_from(1).into_iter().skip(1).collect()
}

fn cups_as_string(cups: &[Label]) -> String {
    cups.iter().map(|c| c.to_string()).collect()
}

struct CupCircle {
    next: HashMap<Label, Label>,
}

impl CupCircle {
    fn new(cups: &[Label]) -> Self {
        let next = (0..cups.len()).map(|i| (cups[i], cups[(i + 1) % cups.len()])).collect();

        CupCircle { next }
    }

    /// Get the next cup clockwise in the circle.
    fn next_cup(&self, cup: Label) -> Label {
        *self.next.get(&cup).expect("Unknown cup label")
    }

    /// Insert `cups` immediately clockwise of `cup`
    fn insert(&mut self, cup: Label, cups: Vec<Label>) {
        let mut next = self.next_cup(cup);
        for label in cups.into_iter().rev() {
            self.next.insert(label, next);
            next = label;
        }

        self.next.insert(cup, next);
    }

    /// All cups starting at `cup`.
    fn cups_from(&self, cup: Label) -> Vec<Label> {
        let mut cups = vec![cup];

        let mut next = self.next_cup(cup);
        while next != cup {
            cups.push(next);
            next = self.next_cup(next);
        }

        cups
    }

    /// Remove `n` cups immediately clockwise of `cup`.
    fn remove(&mut self, cup: Label, n: usize) -> Vec<Label> {
        let mut cups = Vec::new();

        let mut next = self.next_cup(cup);
        for _ in 0..n {
            cups.push(next);
            next = self.next_cup(next);
        }

        self.next.insert(cup, next);

        cups
    }
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

