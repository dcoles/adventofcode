use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let input = read_input("input.txt");
    println!("Part 1: {}", part1(&input));

    let input = read_input("input2.txt");
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    // Rule 0: 8 11 -> 42 42 31
    let set42 = input.expand(42);
    let n = set42.iter().next().unwrap().len();
    let set31 = input.expand(31);

    input.text.iter()
        .filter(|t| matches(&set42, &t[..2*n], 2) && matches(&set31, &t[2*n..], 1))
        .count()
}

fn matches(set: &HashSet<String>, s: &str, n: usize) -> bool {
    if n == 0 {
        return s == "";
    }

    for x in set {
        let len = x.len();
        if s.len() >= len && &s[..len] == x && matches(set, &s[len..], n - 1) {
            return true;
        }
    }

    false
}

fn part2(input: &Input) -> usize {
    // Rule 0: 8 11 -> 42+ 42(42(...)31)31
    let set42 = input.expand(42);
    let n = set42.iter().next().unwrap().len();
    let set31 = input.expand(31);

    input.text.iter()
        .filter(|s| {
            for n31 in 1.. {
                if n31 * n > s.len() {
                    // Too long
                    break;
                }

                // Must match 42 more times than 31
                for n42 in n31 + 1.. {
                    if (n31 + n42) * n > s.len() {
                        // Too long
                        break;
                    }

                    if matches(&set42, &s[..n42*n], n42) && matches(&set31, &s[n42*n..], n31) {
                        return true;
                    }
                }
            }

            false
        }).count()
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

impl Input {
    /// Expand rule into all possible matches
    fn expand(&self, id: u32) -> HashSet<String> {
        let rule = self.rules.get(&id).expect("Unknown rule");

        match rule {
            Rule::Literal(c) => [c.to_string()].iter().cloned().collect(),
            Rule::Subrule(subrule) => {
                let mut expanded = HashSet::new();

                for rs in subrule {
                    match rs.len() {
                        1 => expanded.extend(self.expand(rs[0])),
                        2 => {
                            for a in self.expand(rs[0]) {
                                for b in self.expand(rs[1]) {
                                    expanded.insert(a.clone() + b.as_str());
                                }
                            }
                        }
                        _ => panic!("Unsupported number of terms: {}", rs.len()),
                    }
                }

                expanded
            }
        }
    }
}

#[derive(Debug)]
enum Rule {
    Literal(char),
    Subrule(Vec<Vec<u32>>)
}

impl Rule {
    fn from_str(s: &str) -> Self {
        use Rule::*;

        if s.starts_with('"') {
            Literal(s.chars().nth(1).unwrap())
        } else {
            Subrule(
                s.split('|')
                    .map(|s| {
                        s.split_whitespace().map(|v| v.parse().unwrap()).collect()
                    }).collect())
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_hashset(values: &[&str]) -> HashSet<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_expand() {
        let input = read_input("sample1.txt");

        assert_eq!(input.expand(5), to_hashset(&["b"]));
        assert_eq!(input.expand(4), to_hashset(&["a"]));
        assert_eq!(input.expand(3), to_hashset(&["ab", "ba"]));
        assert_eq!(input.expand(2), to_hashset(&["aa", "bb"]));
        assert_eq!(input.expand(1), to_hashset(&["aaab", "aaba", "bbab", "bbba", "abaa", "abbb", "baaa", "babb"]));
    }
}

