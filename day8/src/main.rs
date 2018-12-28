use std::fs;

fn main() {
    let input = read_input();
    let root = parse(&mut input.into_iter());

    println!("Metadata sum: {}", root.metadata_sum());
    println!("Value: {}", root.value());
}

fn read_input() -> Vec<u32> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    input.split_whitespace()
        .map(|v| v.parse::<u32>().expect("Failed to parse value"))
        .collect()
}

fn parse(input: &mut impl Iterator<Item=u32>) -> Node {
    // Read header
    let n_children = input.next().expect("Unexpected EOF");
    let n_metadata = input.next().expect("Unexpected EOF");

    // For each child, parse it
    let mut children: Vec<Node> = Vec::new();
    for _ in 0..n_children {
        children.push(parse(input));
    }

    // Read metadata
    let mut metadata: Vec<u32> = Vec::new();
    for _ in 0..n_metadata {
        metadata.push(input.next().expect("Unexpected EOF"));
    }

    // Return node
    Node { children, metadata }
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn metadata_sum(&self) -> u32 {
        self.children.iter().map(|c| c.metadata_sum()).sum::<u32>()
            + self.metadata.iter().sum::<u32>()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum::<u32>()
        } else {
            let mut sum = 0;
            for &idx in &self.metadata {
                if let Some(child) = self.children.get((idx as usize) - 1) {
                    sum += child.value();
                }
            }
            sum
        }
    }
}
