use std::path::Path;
use std::fs;

type Input = (u32, Vec<Option<u32>>);

fn main() {
    let input = read_input("input.txt");

    let (wait, earliest_bus) = part1(&input);
    println!("Part 1: Bus {} (wait: {} min) = {}", earliest_bus, wait, earliest_bus * wait);
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

fn part1((timestamp, timetable): &Input) -> (u32, u32) {
    let mut buses = Vec::new();
    for bus in timetable {
        if let &Some(bus) = bus {
            let wait = bus - (timestamp % bus);
            buses.push((wait, bus))
        }
    }

    *buses.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), (5, 59));
    }
}

