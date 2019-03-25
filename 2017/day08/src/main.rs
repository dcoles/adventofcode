use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

fn main() {
    let input = run("input.txt").expect("Failed to read input");
}

fn run(path: &str) -> io::Result<()> {
    let mut running_max = 0;
    let mut regs = HashMap::new();
    for line in read_to_string(path)?.lines() {
        let mut tokens = line.split_whitespace();
        let reg = tokens.next().unwrap().to_string();
        let op = match tokens.next().unwrap() {
            "inc" => inc,
            "dec" => dec,
            name => panic!("Unknown op: {}", name),
        };
        let val: i32 = tokens.next().unwrap().parse().unwrap();
        assert_eq!(tokens.next().unwrap(), "if");
        let cond_reg = tokens.next().unwrap().to_string();
        let cond = match tokens.next().unwrap() {
            ">" => i32::gt,
            "<" => i32::lt,
            ">=" => i32::ge,
            "<=" => i32::le,
            "==" => i32::eq,
            "!=" => i32::ne,
            name => panic!("Unknown cmp: {}", name),
        };
        let cond_val: i32 = tokens.next().unwrap().parse().unwrap();

        if cond(regs.get(&cond_reg).unwrap_or(&0), &cond_val) {
            let n = regs.entry(reg).or_default();
            op(n, val);
            if *n > running_max {
                running_max = *n;
            }
        }
    }

    let max = regs.values().fold(0, |a, &b| if a > b { a } else { b });
    println!("Largest value in any register: {}", max);
    println!("Largest value held at any time: {}", running_max);
    Ok(())
}

fn inc(val: &mut i32, amount: i32) {
    *val += amount;
}

fn dec(val: &mut i32, amount: i32) {
    *val -= amount;
}
