use std::path::Path;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = read_input("input.txt");
    println!("Part 1: {}", input.count_matches(0));
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
    fn count_matches(&self, rule: u32) -> usize {
        self.text.iter().filter(|t| self.match_rule(t, rule) == Some(t.as_str())).count()
    }

    fn match_rule<'a>(&self, s: &'a str, rule: u32) -> Option<&'a str> {
        let rule = self.rules.get(&rule).expect("Unknown rule");
        match rule {
            Rule::Literal(c) => {
                if s.starts_with(*c) { Some(&s[..1]) } else { None }
            },
            Rule::Subrule(srs) => {
                'outer: for sr in srs {
                    let mut i = 0;
                    for &r in sr {
                        if let Some(m) = self.match_rule(&s[i..], r) {
                            i += m.len();
                        } else {
                            continue 'outer;
                        }
                    }

                    return Some(&s[..i]);
                }

                None
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
            Literal(s.chars().skip(1).next().unwrap())
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

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(input.match_rule("aaaabb", 0), Some("aaaabb"));
        assert_eq!(input.match_rule("aaabab", 0), Some("aaabab"));
        assert_eq!(input.match_rule("abbabb", 0), Some("abbabb"));
        assert_eq!(input.match_rule("abbbab", 0), Some("abbbab"));
        assert_eq!(input.match_rule("aabaab", 0), Some("aabaab"));
        assert_eq!(input.match_rule("aabbbb", 0), Some("aabbbb"));
        assert_eq!(input.match_rule("abaaab", 0), Some("abaaab"));
        assert_eq!(input.match_rule("ababbb", 0), Some("ababbb"));

        assert_eq!(input.count_matches(0), 2);
    }
}

