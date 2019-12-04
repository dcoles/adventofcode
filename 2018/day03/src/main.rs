use std::fs;
use regex::Regex;

fn main() {
    let claims = read_claims();
    let mut fabric =  vec![vec![0u32; 1000]; 1000];

    let mut total_overlap = 0;
    for claim in claims.iter() {
        for y in claim.top..claim.top+claim.height {
            for x in claim.left..claim.left+claim.width {
                // Only count the first overlap
                if fabric[y][x] == 1 {
                    total_overlap += 1;
                }
                fabric[y][x] += 1;
            }
        }
    }

    println!("Overlap: {}", total_overlap);

    for claim in claims.iter() {
        let mut overlap = 0;
        for y in claim.top..claim.top+claim.height {
            for x in claim.left..claim.left+claim.width {
                if fabric[y][x] > 1 {
                    overlap += 1;
                }
            }
        }
        if overlap == 0 {
            println!("Claim {} has no overlap", claim.id);
        }
    }
}

fn read_claims() -> Vec<Claim> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let mut claims = Vec::new();
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    for line in input.lines() {
        let caps = re.captures(line)
            .expect("Could not parse claim");
        let claim = Claim {
            id: (&caps[1]).parse()
                .expect("Couldn't parse ID"),
            left: (&caps[2]).parse()
                .expect("Couldn't parse left"),
            top: (&caps[3]).parse()
                .expect("Couldn't parses top"),
            width: (&caps[4]).parse()
                .expect("Couldn't parse width"),
            height: (&caps[5]).parse()
                .expect("Couldn't parse height"),
        };
        claims.push(claim);
    }

    claims
}

#[derive(Debug)]
struct Claim {
    id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
