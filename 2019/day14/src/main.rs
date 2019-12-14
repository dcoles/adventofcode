use std::{fs, fmt};
use std::path::Path;
use std::collections::{HashMap, VecDeque};

type Chemical = String;
type ChemicalRef = str;

const ORE: &'static ChemicalRef = "ORE";
const FUEL: &'static ChemicalRef = "FUEL";

fn main() {
    // Part 1
    assert_eq!(31, required_ore(&read_input("sample1.txt")));
    assert_eq!(165, required_ore(&read_input("sample2.txt")));

    println!("Part 1: Required Ore {}", required_ore(&read_input("input.txt")));
}

fn required_ore(reactions: &[Reaction]) -> u32 {
    let reactant_map = build_reactant_map(reactions);

    let mut requirements = VecDeque::new();
    requirements.push_back((FUEL, 1));

    let mut required_ore = 0;
    let mut chemical_surplus = HashMap::new();
    while let Some((chemical, quantity)) = requirements.pop_front() {
        if chemical == ORE {
            required_ore += quantity;
        } else {
            let surplus = chemical_surplus.get(chemical).copied().unwrap_or(0);
            let consumed = quantity.min(surplus);
            chemical_surplus.insert(chemical, surplus - consumed);
            let quantity = quantity - consumed;

            if quantity == 0 {
                // No need to run a reaction!
                continue;
            }

            let reaction = &reactant_map[chemical];
            let n_reactions = (quantity - 1) / reaction.output.1 + 1;

            for (chem, &n) in &reaction.inputs {
                let amount = n * n_reactions;
                println!("{} {} requires {} {}", quantity, chemical, amount, chem);
                requirements.push_back((chem, amount));
            }

            let surplus = reaction.output.1 * n_reactions - quantity;
            *chemical_surplus.entry(chemical).or_default() += surplus;
        }
    }

    required_ore
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Reaction> {
    let contents = fs::read_to_string(path).expect("Failed to read input");

    let mut reactions = Vec::new();
    for line in contents.lines() {
        let line: Vec<_> = line.split("=>").collect();
        let input = split_chemical_list(&line[0]);
        let output = split_chemical(&line[1]);

        reactions.push(Reaction { inputs: input, output })
    }

    reactions
}

fn split_chemical_list(list: &str) -> HashMap<Chemical, u32> {
    let mut result = HashMap::new();
    for (c, n) in list.split(',').map(|value| split_chemical(value)) {
        result.insert(c, n);
    }

    result
}

fn split_chemical(value: &str) -> (Chemical, u32) {
    let value: Vec<_> = value.trim().split(' ').collect();
    let n: u32 = value[0].parse().expect("Failed to parse quantity");

    (value[1].to_owned(), n)
}

fn build_reactant_map(reactions: &[Reaction]) -> HashMap<Chemical, &Reaction> {
    let mut map = HashMap::new();

    for reaction in reactions {
        let chemical = reaction.output.0.to_owned();
        map.insert(chemical, reaction);
    }

    map
}

#[derive(Debug)]
struct Reaction {
    inputs: HashMap<Chemical, u32>,
    output: (Chemical, u32),
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs = self.inputs.iter().map(|(c, &n)| format!("{} {}", n, c)).collect::<Vec<_>>().join(", ");
        write!(f, "{} => {} {}", inputs, self.output.1, self.output.0)
    }
}
