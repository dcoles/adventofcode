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
    for digit in digits(n) {
        if digit < last_digit {
            return false;
        }
        last_digit = digit;
    }

    true
}

fn count_digits(n: u32) -> HashMap<u32, u32> {
    let mut counter = HashMap::new();
    for digit in digits(n) {
        *counter.entry(digit).or_default() += 1;
    }
    counter
}

fn digits(mut n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.reverse();
    digits
}
