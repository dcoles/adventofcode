use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fs;

fn main() {
    let graph = read_input();

    for (step, node) in &graph.nodes {
        println!("{} -> {:?}", step, node);
    }

    let steps: String = graph.walk(1).into_iter().collect();
    println!("Steps with one worker: {}", steps);

    let n = 5;
    let steps: String = graph.walk(n).into_iter().collect();
    println!("Steps with {} workers: {}", n, steps);
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

    fn walk(&self, workers: u32) -> Vec<char> {
        assert_ne!(workers, 0);

        let mut result: Vec<char> = Vec::new();
        let mut todo: HashSet<char> = HashSet::new();
        let mut finished = HashSet::new();
        let mut workers: Vec<u32> = (1..=workers).collect();
        let mut queue: BinaryHeap<Entry> = BinaryHeap::new();

        todo.extend(self.nodes.keys());

        for t in 0.. {
            while !queue.is_empty() && queue.peek().unwrap().t <= t {
                let Entry { t: _, worker, step } = queue.pop().unwrap();
                workers.push(worker);
                finished.insert(step);
                result.push(step);
            }

            if todo.is_empty() && queue.is_empty() {
                break;
            }

            let mut avail: Vec<char> = todo.iter().cloned()
                .filter(|&s| self.nodes.get(&s).unwrap().iter().all(|&d| finished.contains(&d)))
                .collect();
            avail.sort();
            avail.reverse();

            while let Some(step) = avail.pop() {
                if let Some(worker) = workers.pop() {
                    let end_t = t + (step as u32) - (b'A' as u32) + 61;
                    todo.remove(&step);
                    queue.push(Entry { t: end_t, worker, step });
                } else {
                    // No more work available
                    break;
                }
            }

            println!("t={} todo={:?} queue={:?}", t, todo, queue);
        }

        result
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Entry {
    t: u32,
    worker: u32,
    step: char,
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        other.t.cmp(&self.t)
            .then_with(|| other.step.cmp(&self.step))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
