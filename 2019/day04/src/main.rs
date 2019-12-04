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
    digits_never_decrease(n) && count_digits(n).values().any(|&n| n >= 2)
}

fn valid2(n: u32) -> bool {
    digits_never_decrease(n) && count_digits(n).values().any(|&n| n == 2)
}

fn digits_never_decrease(n: u32) -> bool {
    let mut last_digit = 0;
    for digit in Digits::new(n) {
        if last_digit > digit {
            return false;
        }
        last_digit = digit;
    }

    true
}

fn count_digits(n: u32) -> HashMap<u32, u32> {
    let mut counter = HashMap::new();
    for digit in Digits::new(n) {
        *counter.entry(digit).or_default() += 1;
    }
    counter
}

// Iterates over digits in left-to-right order.
struct Digits {
    n: u32,
    ndigits: u32,
    pos: u32,
}

impl Digits {
    fn new(n: u32) -> Self {
        let mut ndigits = 0;
        let mut m = n;
        while m > 0 {
            ndigits += 1;
            m /= 10;
        }

        Digits { n, ndigits, pos: 1 }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.pos > self.ndigits {
            return None
        }

        let digit = self.n / 10u32.pow(self.ndigits - self.pos) % 10;
        self.pos += 1;

        Some(digit)
    }
}
