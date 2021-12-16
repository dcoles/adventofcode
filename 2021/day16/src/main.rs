//! Advent of Code 2021: Day 16
//! https://adventofcode.com/2021/day/16

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file("day16/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut parser = Parser::new(input.values.clone());
    let _ = parser.parse_packet();

    parser.version_sum
}

fn part2(input: &Input) -> usize {
    let mut parser = Parser::new(input.values.clone());
    let packet = parser.parse_packet().expect("failed to parse packet");

    packet.evaluate()
}

struct Parser<I: IntoIterator<Item=char>> {
    stream: I::IntoIter,
    depth: usize,
    version_sum: usize,
}

impl<I: IntoIterator<Item=char>> Parser<I> {
    fn new(bits: I) -> Self {
        Parser {
            stream: bits.into_iter(),
            depth: 0,
            version_sum: 0,
        }
    }

    fn parse_packet(&mut self) -> io::Result<Packet> {
        let version = decode(&take(&mut self.stream, 3)?);
        let type_id = decode(&take(&mut self.stream, 3)?);

        println!("[{}] # Version: {}, Type-ID: {}", self.depth, version, type_id);

        self.version_sum += version;

        let packet = Packet {
            version,
            packet_type: match type_id {
                0 => PacketType::Sum(self._read_operator()?),
                1 => PacketType::Product(self._read_operator()?),
                2 => PacketType::Min(self._read_operator()?),
                3 => PacketType::Max(self._read_operator()?),
                4 => PacketType::Literal(self._read_literal()),
                5 => PacketType::GreaterThan(self._read_operator()?),
                6 => PacketType::LessThan(self._read_operator()?),
                7 => PacketType::EqualTo(self._read_operator()?),
                _ => panic!("Unknown Type ID: {}", type_id),
            },
        };

        Ok(packet)
    }

    fn _read_literal(&mut self) -> usize {
        let mut bits = Vec::new();

        loop {
            let more = self.stream.next().unwrap();
            for _ in 0..4 {
                bits.push(self.stream.next().unwrap());
            }

            if more == '0' {
                break;
            }
        }

        decode(&bits)
    }

    fn _read_operator(&mut self) -> io::Result<Vec<Packet>> {
        let mut packets = Vec::new();

        let length_type_id = self.stream.next().unwrap();
        if length_type_id == '0' {
            // Next 15 bits are a number that represents the total length in bits of subpackets
            let bits = take(&mut self.stream, 15)?;
            let value = decode(&bits);

            // Slurp these bits!
            println!("[{}] - Type-0: {} bits", self.depth, value);
            let bits = take(&mut self.stream, value)?;

            let mut parser = Parser::new(bits.into_iter());
            parser.depth = self.depth + 1;
            while let Ok(packet) = parser.parse_packet() {
                packets.push(packet);
            }
            self.version_sum += parser.version_sum;
        } else {
            // Next 11 bits are a number of subpackets immediately contained by this packet
            let bits = take(&mut self.stream, 11)?;
            let value = decode(&bits);

            println!("[{}] - Type-1: {} packets", self.depth, value);
            self.depth += 1;
            for _ in 0..value {
                packets.push(self.parse_packet()?);
            }
            self.depth -= 1;
        }

        Ok(packets)
    }
}

/// Take `n` bits from a bit-stream, returning an [`io::Error`] if the stream is exhausted.
fn take(stream: &mut impl Iterator<Item=char>, n: usize) -> io::Result<Vec<char>> {
    let mut value = Vec::new();
    for _ in 0..n {
        value.push(stream.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "End of stream"))?);
    }

    Ok(value)
}

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    packet_type: PacketType,
}

impl Packet {
    fn evaluate(&self) -> usize {
        match &self.packet_type {
            PacketType::Sum(packets) => {
                packets.iter().map(Packet::evaluate).sum()
            },
            PacketType::Product(packets) => {
                packets.iter().map(Packet::evaluate).product()
            },
            PacketType::Min(packets) => {
                packets.iter().map(Packet::evaluate).min().unwrap()
            },
            PacketType::Max(packets) => {
                packets.iter().map(Packet::evaluate).max().unwrap()
            },
            PacketType::Literal(value) => {
                *value
            },
            PacketType::GreaterThan(packets) => {
                if packets[1..].iter().all(|p| packets[0].evaluate() > p.evaluate()) { 1 } else { 0 }
            },
            PacketType::LessThan(packets) => {
                if packets[1..].iter().all(|p| packets[0].evaluate() < p.evaluate()) { 1 } else { 0 }
            },
            PacketType::EqualTo(packets) => {
                if packets[1..].iter().all(|p| packets[0].evaluate() == p.evaluate()) { 1 } else { 0 }
            },
        }
    }
}

#[derive(Debug, Clone)]
enum PacketType {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(usize),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

/// Decode a bit-vector
fn decode(bits: &[char]) -> usize {
    bits.iter().fold(0, |acc, &b| (acc << 1) + if b == '1' { 1 } else { 0 })
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        Ok(Input { values: hex2bin(input.trim()) })
    }
}

/// Convert a hex coded string into a bit-vector
fn hex2bin(s: &str) -> Vec<char> {
    s.chars().flat_map(|c| {
        match c {
            '0' => ['0', '0', '0', '0'],
            '1' => ['0', '0', '0', '1'],
            '2' => ['0', '0', '1', '0'],
            '3' => ['0', '0', '1', '1'],
            '4' => ['0', '1', '0', '0'],
            '5' => ['0', '1', '0', '1'],
            '6' => ['0', '1', '1', '0'],
            '7' => ['0', '1', '1', '1'],
            '8' => ['1', '0', '0', '0'],
            '9' => ['1', '0', '0', '1'],
            'A' => ['1', '0', '1', '0'],
            'B' => ['1', '0', '1', '1'],
            'C' => ['1', '1', '0', '0'],
            'D' => ['1', '1', '0', '1'],
            'E' => ['1', '1', '1', '0'],
            'F' => ['1', '1', '1', '1'],
            _ => panic!("Unknown digit {}", c),
        }
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = hex2bin("8A004A801A8002F478");
        let mut parser = Parser::new(input.into_iter());
        let _ = parser.parse_packet();

        assert_eq!(parser.version_sum, 16);
    }

    #[test]
    fn test_part2() {
        let input = hex2bin("C200B40A82");
        let mut parser = Parser::new(input.into_iter());
        let packet = parser.parse_packet().unwrap();

        assert_eq!(packet.evaluate(), 3);
    }
}
