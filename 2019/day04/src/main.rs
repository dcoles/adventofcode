const INPUT: (u32, u32) = (372037, 905157);

fn main() {
    assert!(valid(111111));
    assert!(!valid(223450));
    assert!(!valid(123789));

    let mut count = 0;
    for n in INPUT.0..=INPUT.1 {
        if valid(n) {
           count += 1;
        }
    }
    println!("Part 1: Number of passwords that meet the criteria: {}", count);
}

fn valid(n: u32) -> bool {
    // The digits never decrease
    let mut prev_digit = 0;
    for pos in (0..6).rev() {
        let digit = digit(n, pos);
        if digit < prev_digit {
            return false;
        }
        prev_digit = digit;
    }

    // Two adjacent digits are the same
    let mut two_adjacent = false;
    let mut last_digit = digit(n, 0);
    for pos in 1..6 {
        let digit = digit(n, pos);
        if digit == last_digit {
            two_adjacent = true;
            break
        };
        last_digit = digit;
    }
    if ! two_adjacent {
        return false;
    }

    return true;
}

fn digit(n: u32, pos: u32) -> u32 {
    n / 10u32.pow(pos) % 10
}
