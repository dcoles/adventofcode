use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

fn main() {
    let input = parse_input("input.txt").unwrap();

    // Part 1
    println!("Checksum: {}", checksum(&input));

    // Part 2
    println!("Sum of evenly divisible values: {}", sum_even_divisible(&input));
}

fn parse_input<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<i32>>, Error> {
    let input = read_to_string(path)?;

    let mut rows = Vec::new();
    for line in input.lines() {
        rows.push(
            line.split_whitespace()
                .map(|v| v.parse().expect("Could not parse"))
                .collect());
    }

    Ok(rows)
}

fn checksum(input: &Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;
    for row in input {
        let mut min = row[0];
        let mut max = row[0];
        for &val in row {
            min = val.min(min);
            max = val.max(max);
        }
        checksum += max - min;
    }

    checksum
}

fn sum_even_divisible(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for row in input {
        let mut row = row.clone();
        row.sort();

        for val1 in row.iter().rev() {
            for val2 in &row {
                if val1 == val2 {
                    continue;
                }
                if val1 % val2 == 0 {
                    sum += val1 / val2;
                    break;
                }
            }
        }
    }

    sum
}

