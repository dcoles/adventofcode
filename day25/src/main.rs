use std::collections::HashSet;
use std::fs;

type Pos = [i32; 4];
type Constellation = HashSet<Pos>;

fn main() {
    let input = parse_input("input.txt");

    let mut constellations: Vec<Constellation> = Vec::new();
    for &point in &input {
        if constellations.iter().any(|c| c.contains(&point)) {
            continue;
        }

        let mut constellation = HashSet::new();
        constellation.insert(point);

        let mut edge = Vec::new();
        let mut seen = HashSet::new();
        edge.push(point);
        while ! edge.is_empty() {
            let point = edge.pop().unwrap();
            seen.insert(point);

            for adj in all_within_range(&input, point) {
                if seen.contains(&adj) {
                    continue;
                }
                constellation.insert(adj);
                edge.push(adj);
            }
        }

        constellations.push(constellation);
    }

    println!("Number of constellations: {:?}", constellations.len());
}

fn parse_input(filename: &str) -> Vec<Pos> {
    let input = fs::read_to_string(filename)
        .expect("Failed to read file");

    let mut result = Vec::new();
    for line in input.lines() {
        let mut iter = line.split(",");
        result.push([
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ]);
    }

    result
}

fn distance(pos1: Pos, pos2: Pos) -> i32{
    let mut result = 0;
    for d in 0..pos1.len() {
        result += (pos1[d] - pos2[d]).abs();
    }

    result
}

fn all_within_range(points: &Vec<Pos>, pos: Pos) -> Vec<Pos> {
    points.iter().map(|&p| p).filter(|&p| distance(pos, p) <= 3).collect()
}
