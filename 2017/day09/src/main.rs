use std::path::Path;
use std::fs::read_to_string;
use std::io::Result;
use std::str::Chars;

fn main() {
    let s = read_to_string("input.txt").expect("Failed to read string");
    println!("{}", score(&s));
}

fn score(s: &str) -> u32
{
    let mut score = 0;
    let mut cur = 0;
    let mut iter = garbage_filtered(s.chars());
    while let Some(c) = iter.next() {
        match c {
            '{' => {cur += 1; score += cur},
            '}' => {cur -= 1},
            _ => (),
        }
    }
    println!("Filtered {}", iter.count);
    score
}

fn garbage_filtered(chars: Chars) -> GarbageFiltered {
    GarbageFiltered { iter: chars, skip: false, garbage: false, count: 0 }
}

struct GarbageFiltered<'a> {
    iter: Chars<'a>,
    skip: bool,
    garbage: bool,
    count: u32,
}

impl<'a> Iterator for GarbageFiltered<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        while let Some(c) = self.iter.next() {
            if self.skip {
                self.skip = false;
                continue;
            }

            if c == '!' {
                self.skip = true;
            } else if self.garbage {
                if c == '>' {
                    self.garbage = false;
                } else {
                    self.count += 1;
                }
            } else if c == '<' {
                self.garbage = true;
            } else {
                return Some(c);
            }
        }

        None
    }
}

#[test]
fn test_score() {
    assert_eq!(score("{}"), 1);
    assert_eq!(score("{{{}}}"), 6);
    assert_eq!(score("{{},{}}"), 5);
    assert_eq!(score("{{{},{},{{}}}}"), 16);
    assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}

