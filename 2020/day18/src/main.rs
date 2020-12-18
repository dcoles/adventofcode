use std::path::Path;
use std::fs;

type Token = char;
type Expr = Vec<Token>;
type Input = Vec<Expr>;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Expr> {
    fs::read_to_string(path).expect("Failed to read input")
        .lines().map(|line| parse(line))
        .collect()
}

fn part1(input: &Input) -> u64 {
    input.iter().map(eval).sum()
}

fn parse(s: &str) -> Vec<Token> {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn eval<'a, I>(tokens: I) -> u64
where
    I: IntoIterator<Item=&'a Token>
{
    eval_(&mut tokens.into_iter())
}

fn eval_<'a, I>(tokens: &mut I) -> u64
where
    I: Iterator<Item=&'a Token>
{
    let mut lhs = match *tokens.next().expect("Expected LHS") {
        '(' => eval_(tokens),
        t if t.is_numeric() => t.to_digit(10).unwrap() as u64,
        x => panic!("Unexpected token: {}", x),
    };

    while let Some(&op) = tokens.next() {
        let op = match op {
            '+' => |a, b| a + b,
            '*' => |a, b| a * b,
            ')' => return lhs,
            x => panic!("Unexpected token: {}", x),
        };

        let rhs = match *tokens.next().expect("Expected RHS") {
            '(' => eval_(tokens),
            t if t.is_numeric() => t.to_digit(10).unwrap() as u64,
            x => panic!("Unexpected token: {}", x),
        };

        lhs = op(lhs, rhs)
    }

    lhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(eval(&parse("1 + 2 * 3 + 4 * 5 + 6")), 71);
    }

    #[test]
    fn test_example2() {
        assert_eq!(eval(&parse("1 + (2 * 3) + (4 * (5 + 6))")), 51);
    }

    #[test]
    fn test_example3() {
        assert_eq!(eval(&parse("2 * 3 + (4 * 5)")), 26);
    }

    #[test]
    fn test_example4() {
        assert_eq!(eval(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3")), 437);
    }

    #[test]
    fn test_example5() {
        assert_eq!(eval(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")), 12240);
    }

    #[test]
    fn test_example6() {
        assert_eq!(eval(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")), 13632);
    }
}

