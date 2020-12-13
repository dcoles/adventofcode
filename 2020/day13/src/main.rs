use std::path::Path;
use std::fs;

type Input = (u64, Vec<Option<u64>>);

fn main() {
    let input = read_input("input.txt");

    let (wait, earliest_bus) = part1(&input);
    println!("Part 1: Bus {} (wait: {} min) = {}", earliest_bus, wait, earliest_bus * wait);

    let input = read_input("input.txt");
    println!("Part 2: {}", part2(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");
    let mut lines = input.lines();

    let timestamp = lines.next().expect("Failed to read line")
        .parse().expect("Failed to parse number");
    let timetable = lines.next().expect("Failed to read line")
        .split(",").map(|val| val.parse().ok()).collect();

    (timestamp, timetable)
}

fn part1((timestamp, timetable): &Input) -> (u64, u64) {
    let mut buses = Vec::new();
    for bus in timetable {
        if let &Some(bus) = bus {
            let wait = bus - (timestamp % bus);
            buses.push((wait, bus))
        }
    }

    *buses.iter().min().unwrap()
}

fn part2(input: &Input) -> u64 {
    let mut numbers = enumerate(&input.1);

    while numbers.len() > 1 {
        let front = &numbers[..2];
        let rest = &numbers[2..];

        let mut new = vec![common(front)];
        new.extend(rest);

        numbers = new;
    }

    let (n, p) = numbers[0];

    p - n
}

/// Turn list of `Option<val>` into list of `(index, val)`.
fn enumerate(timetable: &[Option<u64>]) -> Vec<(u64, u64)> {
    timetable.into_iter().enumerate()
        .flat_map(|(n, &bus)| if let Some(bus) = bus { Some((n as u64, bus)) } else { None }).collect()
}

/// Take a list of `(index, val)` and find the first `t0` mod `p` such that the factors are sequential.
fn common(numbers: &[(u64, u64)]) -> (u64, u64) {
    let mut t: u64 = 0;
    let p = numbers.iter().map(|&(_, x)| x).product();

    'outer: loop {
        for &(n, bus) in numbers {
            let x = (t + n as u64) % bus;
            if x != 0 {
                t += bus - x;
                continue 'outer;
            }
        }

        break;
    }

    (p - t, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), (5, 59));
    }

    #[test]
    fn test_part2_common() {
        let (_, timetable) = read_input("sample1.txt");
        let n = common(&enumerate(&timetable));

        assert_eq!(n.1 - n.0, 1068781);
    }

    #[test]
    fn test_part2() {
        let input = read_input("sample1.txt");
        assert_eq!(part2(&input), 1068781);
    }
}

