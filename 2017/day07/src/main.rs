extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;
use std::io;
use regex::Regex;

fn main() {
    let tower = read_input("input.txt").expect("Failed to read input");

    // Part 1
    let &bottom = tower.topological_sort().last().unwrap();
    println!("The bottom program's name is {}", tower.node(bottom).unwrap());

    // Part 2
    for node_idx in tower.topological_sort() {
        let adjacent = tower.adjacent(node_idx).unwrap();
        if !adjacent.iter().all(|&n| total_weight(&tower, n) == total_weight(&tower, *adjacent.iter().next().unwrap())) {
            println!("The unbalanced disk is held by {}", tower.node(node_idx).unwrap());
            for adj_idx in adjacent {
                let total_weight = total_weight(&tower, adj_idx);
                println!("- [{}] {}", total_weight, tower.node(adj_idx).unwrap());
            }
            break;
        }
    }
}

fn total_weight(tower: &Graph<Program>, node: NodeIndex) -> i32 {
    let prog = tower.node(node).unwrap();
    let adjacent = tower.adjacent(node).unwrap();
    adjacent.iter().fold(prog.weight, |w, &i| w + total_weight(tower, i))
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Graph<Program>> {
    let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();

    let mut node_weights = HashMap::new();
    let mut edges = Vec::new();
    for line in read_to_string(path)?.lines() {
        if let Some(cap) = re.captures(line) {
            let name = &cap[1];
            let weight: i32 = cap[2].parse().unwrap();
            node_weights.insert(name.to_string(), weight);
            if let Some(m) = cap.get(3) {
                for child in m.as_str().split(", ") {
                    edges.push((name.to_string(), child.to_string()));
                }
            };
        }
    }

    let mut graph = Graph::new();
    let mut indexes = HashMap::new();
    for (name, &weight) in &node_weights {
        indexes.insert(name, graph.add_node(Program::new(name, weight)));
    }

    for (from, to) in &edges {
        graph.add_edge(indexes[from], indexes[to]);
    }

    Ok(graph)
}

#[derive(Debug)]
struct Graph<T> {
    nodes: HashMap<NodeIndex, T>,
    edges: HashMap<NodeIndex, HashSet<NodeIndex>>,
    next_node_index: NodeIndex,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct NodeIndex(u32);

impl<T> Graph<T> {
    fn new() -> Graph<T> {
        Graph { nodes: HashMap::new(), edges: HashMap::new(), next_node_index: NodeIndex(0) }
    }

    fn add_node(&mut self, value: T) -> NodeIndex {
        let index = self.next_node_index;
        self.nodes.insert(index, value);
        self.next_node_index = NodeIndex(self.next_node_index.0 + 1);
        index
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.edges.entry(from).or_default().insert(to);
    }

    fn adjacent(&self, node: NodeIndex) -> Option<HashSet<NodeIndex>>  {
        if !self.nodes.contains_key(&node) {
            return None;
        }

        if let Some(adjacent) = self.edges.get(&node) {
            Some(adjacent.iter().map(|&n| n).collect())
        } else {
            Some(HashSet::new())
        }
    }

    fn node(&self, node: NodeIndex) -> Option<&T>  {
        self.nodes.get(&node)
    }

    fn topological_sort(&self) -> Vec<NodeIndex> {
        let mut result = Vec::new();

        let keys: HashSet<NodeIndex> = self.edges.keys().map(|k| *k).collect();
        let mut seen: HashSet<NodeIndex> = HashSet::new();
        while let Some(&start) = keys.difference(&seen).last() {
            let mut path: Vec<NodeIndex> = Vec::new();
            let mut edge: Vec<NodeIndex> = Vec::new();
            edge.push(start);
            while !edge.is_empty() {
                let node = edge.pop().unwrap();
                if seen.contains(&node) {
                    continue
                }
                if self.edges.contains_key(&node) {
                    let children = self.edges.get(&node).unwrap();
                    for &child in children {
                        edge.push(child);
                    }
                }
                seen.insert(node);
                path.push(node);
            }
            result.extend(path.iter().rev());
        }

        result
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: i32,
}

impl Program {
    fn new(name: &str, weight: i32) -> Program {
        Program { name: name.to_string(), weight }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.weight)
    }
}
