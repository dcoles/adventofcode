use std::path::Path;
use std::fs::read_to_string;
use std::io::Error;

fn main() {
    let layers = read_input("input.txt").expect("failed to read input");

    // Part 1
    let severity: u32 = layers.iter()
        .filter(|l| l.caught(0))
        .map(|l| l.severity())
        .sum();

    println!("Part 1: Trip severity is {}", severity);

    // Part 2
    for delay in 1.. {
        if layers.iter().all(|l| !l.caught(delay)) {
            println!("Part 2: Delay {} psec", delay);
            break;
        }
    }


}

fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<Layer>, Error> {
    let mut result = Vec::new();

    for line in read_to_string(path)?.lines() {
        let mut split = line.split(": ");
        let depth: u32 = split.next().unwrap().parse().expect("Failed to parse int");
        let range: u32 = split.next().unwrap().parse().expect("Failed to parse int");
        result.push(Layer::new(depth, range));
    }

    Ok(result)
}

#[derive(Debug, Clone)]
struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn new(depth: u32, range: u32) -> Layer{
        Layer { depth, range }
    }

    fn severity(&self) -> u32 {
        self.depth * self.range
    }

    fn caught(&self, delay: u32) -> bool {
        (delay + self.depth) % (2 * self.range - 2) == 0
    }
}
