use std::fs;

fn main() {
    let mut nanobots = read_input("input.txt");
    nanobots.sort_by_key(|n| n.r);

    let strongest = nanobots.last().unwrap();
    println!("Strongest Nanobot: {:?}", strongest);
    let in_range = nanobots.iter().filter(|n| strongest.distance(n) <= strongest.r).count();
    println!("In range: {}", in_range);
}

fn read_input(filename: &str) -> Vec<Nanobot> {
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    let mut result = Vec::new();
    for line in input.lines() {
        let mut pos = (0, 0, 0);
        let mut r = 0;
        for col in line.split_whitespace() {
            let mut iter = col.split("=");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();

            match key {
                "pos" => {
                    let mut iter = value[1..value.len()-2].split(",");
                    pos = (
                        parse_int(iter.next().unwrap()),
                        parse_int(iter.next().unwrap()),
                        parse_int(iter.next().unwrap()),
                    );
                },
                "r" => r = parse_int(value),
                _ => (),
            }
        }

        result.push(Nanobot { pos, r })
    }

    result
}

fn parse_int(s: &str) -> i32 {
    s.parse().expect("Failed to parse int")
}

type Pos = (i32, i32, i32);

#[derive(Debug, Copy, Clone)]
struct Nanobot {
    pos: Pos,
    r: i32,
}

impl Nanobot {
    // Manhattan distance
    fn distance(&self, other: &Nanobot) -> i32 {
        (self.pos.0 - other.pos.0).abs()
            + (self.pos.1 - other.pos.1).abs()
            + (self.pos.2 - other.pos.2).abs()
    }
}
