use std::path::Path;
use std::fs;
use regex::Regex;
use std::collections::{HashSet, HashMap};

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");
    let mut sections = input.split("\n\n");

    let fields_re = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let fields: HashMap<String, HashSet<u64>> = sections.next().expect("Failed to read section")
        .lines().map(|line| {
            let m = fields_re.captures(line).expect("Failed to parse field");
            let key = m[1].to_owned();
            let a = m[2].parse().unwrap();
            let b = m[3].parse().unwrap();
            let c = m[4].parse().unwrap();
            let d = m[5].parse().unwrap();
            let values: HashSet<u64> = (a..=b).chain(c..=d).collect();

            (key, values)
        }).collect();

    let ticket = sections.next().expect("Failed to read section")
        .lines().skip(1).next().expect("Failed to read line")
        .split(',').map(|val| val.parse().expect("Failed to parse number"))
        .collect();

    let nearby = sections.next().expect("Failed to read section")
        .lines().skip(1).map(|line| {
            line.split(',').map(|val| val.parse().expect("Failed to parse number")).collect()
        }).collect();

    Input { fields, ticket, nearby }
}

fn part1(input: &Input) -> u64 {
    input.nearby.iter()
        .flat_map(|ticket| ticket.iter().filter(|&val| invalid_value(&input.fields, *val)))
        .sum()
}

fn invalid_ticket(fields: &HashMap<String, HashSet<u64>>, values: &[u64]) -> bool {
    values.iter().any(|val| invalid_value(fields, *val))
}

fn invalid_value(fields: &HashMap<String, HashSet<u64>>, value: u64) -> bool {
    !fields.iter().any(|(_, set)| set.contains(&value))
}

fn part2(input: &Input) -> u64 {
    let nearby: Vec<Vec<u64>> = input.nearby.iter().filter(|ticket| !invalid_ticket(&input.fields, ticket)).cloned().collect();
    let n_fields = input.fields.len();

    // Map of fields to possible ticket positions
    let mut possibilities: HashMap<String, HashSet<usize>> = input.fields.keys().map(|f| (f.to_owned(), (0..n_fields).collect())).collect();

    // Remove invalid possibilities
    for (name, valid_values) in &input.fields {
        let possible = possibilities.get_mut(name).unwrap();
        for ticket in &nearby {
            for (j, &val) in ticket.iter().enumerate() {
                if !valid_values.contains(&val) {
                    possible.remove(&j);
                }
            }
        }
    }

    // Solve the rest by deduction
    let mut found = HashMap::new();
    while found.len() < n_fields {
        let field = possibilities.iter().filter_map(|(f, p)| if p.len() == 1 { Some(f.to_owned()) } else { None }).next().unwrap();
        let ticket_position = possibilities.remove(&field).unwrap().iter().next().copied().unwrap();
        found.insert(field, ticket_position);

        for p in possibilities.values_mut() {
            p.remove(&ticket_position);
        }
    }

    // Look up ticket fields by index
    found.into_iter()
        .filter_map(|(f, n)| if f.starts_with("departure") { Some(input.ticket[n]) } else { None })
        .product()
}

#[derive(Debug)]
struct Input {
    fields: HashMap<String, HashSet<u64>>,
    ticket: Vec<u64>,
    nearby: Vec<Vec<u64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 71);
    }
}

