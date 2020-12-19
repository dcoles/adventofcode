use std::path::Path;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = read_input("input.txt");
    let rules: HashMap<_, _> = input.rules.keys()
        .map(|&k| {
            let p = String::from("^") + &rule_as_pattern(&input.rules, k) + "$";

            (k, Regex::new(&p).expect("Bad regex"))
        }).collect();

    let rule0 = rules.get(&0).unwrap();
    let count = input.text.iter().filter(|l| rule0.is_match(l)).count();

    println!("Part 1: {}", count);
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let input = fs::read_to_string(path).expect("Failed to read input");
    let mut sections = input.split("\n\n");

    let rule_re = Regex::new(r"^(\d+): (.*)$").unwrap();
    let rules = sections.next().expect("Missing section")
        .lines().map(|line| {
            let m = rule_re.captures(&line).expect("Failed to parse rule");
            let n = m[1].parse().unwrap();
            let rule = Rule::from_str(&m[2]);

            (n, rule)
        }).collect();

    let text = sections.next().expect("Missing section")
        .lines().map(|line| line.to_owned()).collect();

    Input { rules, text }
}

#[derive(Debug)]
struct Input {
    rules: HashMap<u32, Rule>,
    text: Vec<String>,
}

fn rule_as_pattern(rules: &HashMap<u32, Rule>, rule: u32) -> String {
    use Rule::*;

    let rule = rules.get(&rule).expect("Unknown rule");

    match rule {
        Literal(c) => c.to_string(),
        Subrule(rs) => rs.iter().map(|&r| rule_as_pattern(rules, r)).collect(),
        PipeSubrule(rss) => {
            let subrules: Vec<String> = rss.iter().map(|rs| rs.iter().map(|&r| rule_as_pattern(rules, r)).collect()).collect();

            String::from("(") + &subrules.join("|") + ")"
        },
    }
}

#[derive(Debug)]
enum Rule {
    Literal(char),
    Subrule(Vec<u32>),
    PipeSubrule(Vec<Vec<u32>>)
}

impl Rule {
    fn from_str(s: &str) -> Self {
        use Rule::*;

        if s.starts_with('"') {
            Literal(s.chars().skip(1).next().unwrap())
        } else if s.contains('|') {
            PipeSubrule(s.split('|').map(|s| s.split_whitespace().map(|v| v.parse().unwrap()).collect()).collect())
        } else {
            Subrule(s.split_whitespace().map(|v| v.parse().unwrap()).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 0);
    }
}

