//! Advent of Code 2021: Day 18
//! https://adventofcode.com/2021/day/18

use std::fmt::Debug;
use std::fs;
use std::io;
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;

fn main() {
    let input = Input::from_file("day18/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u32 {
    let sum: Number = input.values.iter().cloned().sum();
    
    sum.magnitude()
}

fn part2(input: &Input) -> u32 {
    let mut max = 0;
    for i in 0..input.values.len() {
        for j in 0..input.values.len() {
            if i == j {
                continue;
            }

            let sum = input.values[i].clone() + input.values[j].clone();
            let magnitude = sum.magnitude();

            if magnitude > max {
                max = magnitude;
            }
        }
    }

    max
}

#[derive(Clone)]
struct Number(Vec<[Element; 2]>, Vec<u32>);

impl Number {
    fn parse(s: &str) -> Self {
        let mut pairs = Vec::new();
        let mut numbers = Vec::new();
        let mut stack = vec![Vec::new()];
        for c in s.chars() {
            match c {
                '[' => {
                    stack.push(Vec::new());
                },
                ']' => {
                    let mut elements = stack.pop().unwrap();
                    let b = elements.pop().unwrap();
                    let a = elements.pop().unwrap();

                    pairs.push([a, b]);
                    stack.last_mut().unwrap().push(Element::Pair(pairs.len() - 1));

                },
                ',' => (),
                x => {
                    let number = x.to_digit(10).unwrap();

                    numbers.push(number);
                    stack.last_mut().unwrap().push(Element::Value(numbers.len() - 1));
                },
            }
        }

        Number(pairs, numbers)
    }

    fn magnitude(&self) -> u32 {
        do_magnitude(self, self.0.len() - 1)
    }
}

fn do_magnitude(number: &Number, n: usize) -> u32 {
    let a = match number.0[n][0] {
        Element::Value(i) => number.1[i],
        Element::Pair(m) => do_magnitude(number, m),
    };

    let b = match number.0[n][1] {
        Element::Value(x) => number.1[x],
        Element::Pair(m) => do_magnitude(number, m),
    };

    3 * a + 2 * b
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_pair(self, self.0.len() - 1, f)
    }
}

fn format_pair(pair: &Number, n: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;

    match &pair.0[n][0] {
        &Element::Value(n) => write!(f, "{}", pair.1[n])?,
        &Element::Pair(p) => format_pair(pair, p, f)?,
    };

    write!(f, ",")?;

    match &pair.0[n][1] {
        &Element::Value(n) => write!(f, "{}", pair.1[n])?,
        &Element::Pair(p) => format_pair(pair, p, f)?,
    };

    write!(f, "]")
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut value = iter.next().unwrap();
        for number in iter {
            value = value + number;
        }

        value
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        //println!("  {:?}", self);
        //println!("+ {:?}", rhs);
        let n = self.0.len();
        let m = self.1.len();
        let mut pairs = self.0;
        pairs.extend(rhs.0);

        let mut numbers = self.1;
        numbers.extend(rhs.1);

        // Fix up indexes
        for i in n..pairs.len() {
            for j in 0..2 {
                match &mut pairs[i][j] {
                    Element::Value(x) => {
                        *x += m;
                    },
                    Element::Pair(x) => {
                        *x += n;
                    }
                }
            }
        }

        pairs.push([Element::Pair(n - 1), Element::Pair(pairs.len() - 1)]);

        let mut pair = Number(pairs, numbers);
        //println!("= {:?}", pair);
        reduce(&mut pair);

        pair
    }
}

fn reduce(pair: &mut Number) {
    loop {
        if explode(pair) {
            //println!("Explode: {:?}", pair);
            continue;
        }

        if split(pair) {
            //println!("Split: {:?}", pair);
            continue;
        }

        break;
    }
}

fn explode(pair: &mut Number) -> bool {
    do_explode(pair, pair.0.len() - 1, 0)
}

fn do_explode(pair: &mut Number, n: usize, depth: usize) -> bool {
    for i in 0..2 {
        match pair.0[n][i] {
            Element::Pair(new_n) => {
                let new_depth = depth + 1;
                if new_depth == 4 {
                    // Explode
                    let [a, b] = pair.0[new_n];
                    let ax = a.value().unwrap();
                    let bx = b.value().unwrap();
                    let a = pair.1[ax];
                    pair.1[ax] = 0;
                    let b = pair.1[bx];
                    pair.0[n][i] = Element::Value(ax);

                    if ax > 0 {
                        pair.1[ax - 1] += a;
                    }

                    if bx + 1 < pair.1.len() {
                        pair.1[bx + 1] += b;
                    }

                    pair.0.remove(new_n);
                    pair.1.remove(bx);

                    // Fix up indexes
                    for m in 0..pair.0.len() {
                        for i in 0..2 {
                            match &mut pair.0[m][i] {
                                Element::Pair(p) => {
                                    if *p > new_n {
                                        *p -= 1;
                                    }
                                },
                                Element::Value(n) => {
                                    if *n > bx {
                                        *n -= 1;
                                    }
                                }
                            }
                        }
                    }


                    return true;
                } else if do_explode(pair, new_n, new_depth) {
                    return true;
                }
            },
            _ => (),
        }
    }

    false
}

fn split(pair: &mut Number) -> bool {
    do_split(pair, pair.0.len() - 1)
}

fn do_split(pair: &mut Number, n: usize) -> bool {
    for i in 0..2 {
        match pair.0[n][i] {
            Element::Pair(p) => {
                if do_split(pair, p) {
                    return true;
                }
            },
            Element::Value(ix) if pair.1[ix] >= 10 => {
                // Fix up indexes
                for m in 0..pair.0.len() {
                    for i in 0..2 {
                        match &mut pair.0[m][i] {
                            Element::Pair(p) => {
                                if *p > n {
                                    *p += 1;
                                }
                            },
                            Element::Value(n) => {
                                if *n > ix {
                                    *n += 1;
                                }
                            },
                        }
                    }
                }

                // Split
                let x = pair.1[ix];
                let a = x / 2;
                let b = x / 2 + (x % 2);

                pair.1[ix] = a;
                pair.1.insert(ix + 1, b);
                pair.0.insert(n + 1, [Element::Value(ix), Element::Value(ix + 1)]);
                pair.0[n][i] = Element::Pair(n + 1);

                return true;
            },
            _ => (),
        }
    }

    false
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Value(usize),
    Pair(usize),
}

impl Element {
    fn value(&self) -> Option<usize> {
        match self {
            &Self::Value(n) => Some(n),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Number>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            values.push(Number::parse(line));
        }

        Ok(Input { values })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example5.txt").expect("failed to read input");

        assert_eq!(part1(&input), 4140);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example5.txt").expect("failed to read input");

        assert_eq!(part2(&input), 3993);
    }
}
