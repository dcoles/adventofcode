//! Advent of Code 2023: Day 19 "Aplenty"
//! https://adventofcode.com/2023/day/19

use std::collections::HashMap;
use std::{fs, io};
use std::path::Path;

const IN: &str = "in";

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    let mut accepted = vec![];
    for ratings in &input.part_ratings {
        let mut workflow_name = IN;
        loop {
            match input.workflows[workflow_name].pick_action(ratings) {
                Action::Accept => {
                    accepted.push(ratings);
                    break;
                },
                Action::Reject => {
                    break;
                },
                Action::Redirect(name) => workflow_name = name,
            }
        }
    }

    accepted.into_iter().flat_map(|p| p).sum()
}

fn part2(input: &Input) -> u64 {
    let mut accepted = vec![];

    // [min, max] for [X, M, A, S]
    let start = [[1, 4000]; 4];
    let mut edge = vec![(IN, start)];

    while let Some((workflow_name, ratings)) = edge.pop() {
        let workflow = &input.workflows[workflow_name];

        // We filter the range while evaluating the workflow
        let mut ratings = ratings.clone();

        // Match against the rules
        for rule in &workflow.rules {
            match rule {
                Rule::GreaterThan(cat, val, action) => {
                    let i = *cat as usize;

                    let mut matched_ratings = ratings.clone();
                    let range = &mut matched_ratings[i];

                    range[0] = range[0].max(*val + 1);

                    // Check if range has at least one valid value
                    if range[1] >= range[0] {
                        match action {
                            Action::Accept => accepted.push(matched_ratings),
                            Action::Reject => (),
                            Action::Redirect(name) => edge.push((name.as_str(), matched_ratings)),
                        }
                    }

                    // The unmatched remainder
                    ratings[i][1] = ratings[i][1].min(*val);
                },
                Rule::LessThan(cat, val, action) => {
                    let i = *cat as usize;

                    let mut matched_ratings = ratings.clone();
                    let range = &mut matched_ratings[i];

                    range[1] = range[1].min(*val - 1);

                    // Check if range has at least one valid value
                    if range[1] >= range[0] {
                        match action {
                            Action::Accept => accepted.push(matched_ratings),
                            Action::Reject => (),
                            Action::Redirect(name) => edge.push((name.as_str(), matched_ratings)),
                        }
                    }

                    // The unmatched remainder
                    ratings[i][0] = ratings[i][0].max(*val);
                },
            }
        }

        // Default case (if there is at least one valid value)t
        if ratings.iter().any(|r| r[1] >= r[0]) {
            match &workflow.default {
                Action::Accept => accepted.push(ratings),
                Action::Reject => (),
                Action::Redirect(name) => edge.push((name.as_str(), ratings)),
            }
        }
    }

    accepted.into_iter()
    .map(|rating_ranges| rating_ranges.into_iter().map(|r| (r[1] + 1).saturating_sub(r[0])).product::<u64>())
    .sum()
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    default: Action,
}

impl Workflow {
    fn pick_action(&self, ratings: &[u64; 4]) -> &Action {
        for rule in &self.rules {
            match rule {
                Rule::GreaterThan(cat, val, action) if ratings[*cat as usize] > *val => return action,
                Rule::LessThan(cat, val, action) if ratings[*cat as usize] < *val => return action,
                _ => (),
            }
        }

        &self.default
    }
}

#[derive(Debug, Clone)]
enum Rule {
    GreaterThan(Category, u64, Action),
    LessThan(Category, u64, Action),
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let category = Category::from_str(&s[0..1]);
        let (value, action) = s[2..].split_once(':').unwrap();
        let value = value.parse::<u64>().unwrap();
        let action = Action::from_str(action);

        match &s[1..2] {
            ">" => Self::GreaterThan(category, value, action),
            "<" => Self::LessThan(category, value, action),
            t => panic!("unknown rule type: {t:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Accept,
    Reject,
    Redirect(String),
}

impl Action {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Redirect(s.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::ExtremelyCoolLooking,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => panic!("unknown category: {s:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    workflows: HashMap<String, Workflow>,
    part_ratings: Vec<[u64; 4]>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let (chunk1, chunk2) = input.split_once("\n\n").unwrap();

        let mut workflows = HashMap::new();
        for line in chunk1.lines() {
            let (name, rules) = line.split_once('{').unwrap();
            let rules: Vec<_> = rules.trim_end_matches('}').split(',').collect();
            let default = Action::from_str(rules.last().unwrap());
            let rules: Vec<_> = rules[..rules.len() - 1].iter().map(|&r| Rule::from_str(r)).collect();

            workflows.insert(name.to_owned(), Workflow { rules, default });
        }

        let mut part_ratings = vec![];
        for line in chunk2.lines() {
            // Ratings are always in order `[X, M, A, S]``, so we just trim off the `*=` prefix to get the value
            let mut ratings = [0; 4];
            for (i, value) in line.trim_matches(|c| c == '{' || c == '}').split(',').map(|value| value[2..].parse::<u64>().unwrap()).enumerate() {
                ratings[i] = value;
            }

            part_ratings.push(ratings);
        }

        Ok(Self { workflows, part_ratings })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 19114);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 348378);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 167409079868000);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 121158073425385);
    }
}
