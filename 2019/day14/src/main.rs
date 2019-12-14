use std::{fs, fmt};
use std::path::Path;
use std::collections::{HashMap, VecDeque};

type Chemical = String;

const ORE: &str = "ORE";
const FUEL: &str = "FUEL";
const TRILLION: u64 = 1_000_000_000_000;

fn main() {
    let input = read_input("input.txt");

    // Part 1
    assert_eq!(31, required_ore(&read_input("sample1.txt"), 1));
    assert_eq!(165, required_ore(&read_input("sample2.txt"), 1));
    assert_eq!(13312, required_ore(&read_input("sample3.txt"), 1));
    assert_eq!(180697, required_ore(&read_input("sample4.txt"), 1));
    assert_eq!(2210736, required_ore(&read_input("sample5.txt"), 1));

    println!("Part 1: Required ore: {}", required_ore(&input, 1));

    // Part 2
    assert_eq!(82892753, maximum_fuel(&read_input("sample3.txt"), TRILLION));
    assert_eq!(5586022, maximum_fuel(&read_input("sample4.txt"), TRILLION));
    assert_eq!(460664, maximum_fuel(&read_input("sample5.txt"), TRILLION));

    println!("Part 2: Maximum fuel: {}", maximum_fuel(&input, TRILLION));
}

/// Find the amount of ore required for n-units of fuel
fn required_ore(reactions: &[Reaction], fuel: u64) -> u64 {
    let reactant_map = build_reactant_map(reactions);
    let mut required_ore = 0;

    // Queue of required chemicals
    let mut requirements = VecDeque::new();
    requirements.push_back((FUEL, fuel));

    // Surplus from previous reactions
    let mut chemical_surplus = HashMap::new();

    // Run required reactions
    while let Some((chemical, mut quantity)) = requirements.pop_front() {
        if chemical == ORE {
            required_ore += quantity;
        } else {
            // Can we use some surplus?
            if let Some(&surplus) = chemical_surplus.get(chemical) {
                let consumed = quantity.min(surplus);
                chemical_surplus.insert(chemical, surplus - consumed);
                quantity -= consumed;

                // No need to run if we have enough surplus
                if quantity == 0 {
                    continue;
                }
            }

            // How many times do we need to run a reaction?
            let reaction = &reactant_map[chemical];
            let n_reactions = (quantity - 1) / reaction.quantity + 1;

            // How much reactants do we need?
            for (chem, &n) in &reaction.reactants {
                let amount = n * n_reactions;
                requirements.push_back((chem, amount));
            }

            // Collect surplus
            let surplus = reaction.quantity * n_reactions - quantity;
            *chemical_surplus.entry(chemical).or_default() += surplus;
        }
    }

    required_ore
}

/// Find the maximum amount of fuel a fixed amount of ore can produce
fn maximum_fuel(reactions: &[Reaction], ore: u64) -> u64 {
    let mut fuel = 0;

    // Do an exponential search for the required fuel
    loop {
        let mut n = 0;
        while required_ore(reactions, fuel + 2u64.pow(n)) < ore {
            n += 1;
        }

        if n > 0 {
            fuel += 2u64.pow(n - 1);
        } else {
            // Found the maximum
            break;
        }
    }

    fuel
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Reaction> {
    let contents = fs::read_to_string(path).expect("Failed to read input");

    let mut reactions = Vec::new();
    for line in contents.lines() {
        let line: Vec<_> = line.split("=>").collect();
        let reactants = parse_chemical_list(&line[0]);
        let product_quantity = parse_chemical(&line[1]);

        reactions.push(Reaction { reactants, product: product_quantity.0, quantity: product_quantity.1 })
    }

    reactions
}

/// Parse a list of chemicals quantities, `N CHEMICAL, ...`
fn parse_chemical_list(list: &str) -> HashMap<Chemical, u64> {
    let mut result = HashMap::new();
    for (c, n) in list.split(',').map(|value| parse_chemical(value)) {
        result.insert(c, n);
    }

    result
}

/// Parse a single chemical quantity, `N CHEMICAL`
fn parse_chemical(value: &str) -> (Chemical, u64) {
    let value: Vec<_> = value.trim().split(' ').collect();
    let n: u64 = value[0].parse().expect("Failed to parse quantity");

    (value[1].to_owned(), n)
}

/// Build a mapping from Chemical to its associated reaction
fn build_reactant_map(reactions: &[Reaction]) -> HashMap<Chemical, &Reaction> {
    let mut map = HashMap::new();

    for reaction in reactions {
        let chemical = reaction.product.to_owned();
        map.insert(chemical, reaction);
    }

    map
}

/// A chemical reaction
#[derive(Debug)]
struct Reaction {
    reactants: HashMap<Chemical, u64>,
    product: Chemical,
    quantity: u64,
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs = self.reactants.iter().map(|(c, &n)| format!("{} {}", n, c)).collect::<Vec<_>>().join(", ");
        write!(f, "{} => {} {}", inputs, self.quantity, self.product)
    }
}
