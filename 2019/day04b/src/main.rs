// Version that operates on strings.

use std::collections::HashMap;

const INPUT: std::ops::RangeInclusive<u32> = 372037..=905157;

fn main() {
    // Part 1
    assert!(valid1(111111));
    assert!(!valid1(223450));
    assert!(!valid1(123789));

    let count = INPUT.filter(|&n| valid1(n)).count();
    println!("Part 1: Number of passwords that meet the criteria: {}", count);

    // Part 2
    assert!(valid2(112233));
    assert!(!valid2(123444));
    assert!(valid2(111122));

    let count = INPUT.filter(|&n| valid2(n)).count();
    println!("Part 2: Number of passwords that meet the criteria: {}", count);
}

fn valid1(n: u32) -> bool {
    let s = n.to_string();

    digits_never_decrease(&s) && count_digits(&s).values().any(|&n| n >= 2)
}

fn valid2(n: u32) -> bool {
    let s = n.to_string();

    digits_never_decrease(&s) && count_digits(&s).values().any(|&n| n == 2)
}

fn digits_never_decrease(s: &str) -> bool {
    // ASCII-codes for digits 0-9 are in increasing order (codes 48-57)
    let mut lastc = '\0';
    for c in s.chars() {
        if lastc > c {
            return false;
        }
        lastc = c;
    }

    true
}

fn count_digits(s: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for c in s.chars() {
        *counter.entry(c).or_default() += 1;
    }

    counter
}
