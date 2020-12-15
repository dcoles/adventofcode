use std::path::Path;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

type Input = Vec<Instruction>;
const ZERO: char = '0';
const ONE: char = '1';
const FLOATING: char = 'X';

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn read_input<T: AsRef<Path>>(path: T) -> Input {
    let mask_re = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| {
            if let Some(m) = mask_re.captures(&line) {
                let mask = mask_to_bitmasks(&m[1]);
                Instruction::Mask(mask.0, mask.1, mask.2)
            } else if let Some(m) = mem_re.captures(&line) {
                Instruction::Mem(m[1].parse().unwrap(), m[2].parse().unwrap())
            } else {
                panic!("Failed to parse line: {}", line);
            }
        }).collect()
}

fn mask_to_bitmasks(s: &str) -> (u64, u64, u64) {
    let mut mask_on = 0x0000000000000000_u64;
    let mut mask_off = 0xffffffffffffffff_u64;
    let mut mask_floating = 0x0000000000000000_u64;

    for c in s.chars() {
        mask_on <<= 1;
        mask_off <<= 1;
        mask_floating <<= 1;

        match c {
            ZERO => {
                // Nothing to do, left shift did everything!
            },
            ONE => {
                mask_on |= 1;
                mask_off |= 1;
            },
            FLOATING => {
                mask_floating |= 1;
                mask_off |= 1;
            },
            _ => panic!("Unknown mask: {}", c),
        }
    }

    (mask_on, mask_off, mask_floating)
}

fn part1(input: &Input) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = (0x0000000000000000_u64, 0xffffffffffffffff_u64);
    for instruction in input {
        match instruction {
            &Instruction::Mask(m_on, m_off, _) => {
                mask = (m_on, m_off);
            },
            &Instruction::Mem(addr, val) => {
                //println!("val:   {:064b}", val);
                //println!("mask1: {:064b}", mask_on);
                //println!("mask0: {:064b}", mask_off);
                let result = (val | mask.0) & mask.1;
                //println!("res:   {:064b}", result);
                mem.insert(addr, result);
            },
        }
    }

    mem.values().sum()
}

fn part2(input: &Input) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    // on-mask and floating-mask
    let mut mask = (0x0000000000000000_u64, 0x0000000000000000_u64);
    for instruction in input {
        match instruction {
            &Instruction::Mask(m_on, _, m_floating) => {
                // Update masks
                mask = (m_on, m_floating);
            },
            &Instruction::Mem(addr, val) => {
                // Determine all possible floating values
                for mut n in 0..2_u64.pow(mask.1.count_ones()) {
                    let mut addr = addr | mask.0;
                    let mut mask_floating = mask.1;

                    // Set floating bits
                    for _ in 0..64 {
                        if mask_floating & 1_u64 == 1 {
                            addr = (addr & 0xfffffffffffffffe) | (n & 1_u64);
                            n >>= 1;
                        }

                        addr = addr.rotate_right(1);
                        mask_floating = mask_floating.rotate_right(1);
                    }

                    // Write to affected address
                    mem.insert(addr, val);
                }
            },
        }
    }

    mem.values().sum()
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("sample1.txt");
        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn test_part2() {
        let input = read_input("sample2.txt");
        assert_eq!(part2(&input), 208);
    }
}

