use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let graph = read_input();

    for (step, node) in &graph.nodes {
        println!("{} -> {:?}", step, node);
    }

    let steps: String = graph.walk().into_iter().collect();
    println!("Steps: {}", steps);
}

fn read_input() -> Graph {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let mut graph = Graph::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let s2 = chars.nth(5).unwrap();
        let s1 = chars.nth(30).unwrap();

        // S1 depends on S2
        graph.add_dependency(s1, s2);
    }

    graph
}

struct Graph {
    nodes: HashMap<char, Vec<char>>
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: HashMap::new() }
    }

    // `s1` depends on `s2`
    fn add_dependency(&mut self, s1: char, s2: char) {
        self.nodes.entry(s2).or_default();
        self.nodes.entry(s1).or_default().push(s2);
    }

    fn walk(&self) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        let mut todo: HashSet<char> = HashSet::new();
        let mut finished = HashSet::new();

        todo.extend(self.nodes.keys());

        while ! todo.is_empty() {
            let mut avail: Vec<char> = todo.iter().cloned()
                .filter(|&s| self.nodes.get(&s).unwrap().iter().all(|&d| finished.contains(&d)))
                .collect();
            avail.sort();

            let &s = avail.first().unwrap();
            todo.remove(&s);
            finished.insert(s);
            result.push(s);
        }

        result
    }
}

