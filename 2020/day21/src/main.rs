use std::path::Path;
use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input("input.txt");

    // Frequency counts of allergens
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    for food in &input {
        for allergen in &food.allergens {
            allergens.entry(allergen.clone())
                .and_modify(|e| *e = e.intersection(&food.ingredients).cloned().collect())
                .or_insert_with(|| food.ingredients.clone());
        }
    }

    let foods_with_allergens: HashSet<_> = allergens.iter().flat_map(|(_, f)| f.iter().cloned()).collect();

    let part1 = input.iter()
        .flat_map(|f| f.ingredients.iter().cloned())
        .filter(|ig| !foods_with_allergens.contains(ig))
        .count();

    println!("Part 1: {}", part1);

    let mut allergen_ingredients: HashMap<String, String> = HashMap::new();
    while !allergens.is_empty() {
        let allergen = allergens.iter()
            .filter_map(|(a, is)| if is.len() == 1 { Some(a) } else { None })
            .next().unwrap().clone();
        let ingredient = allergens.remove(&allergen).unwrap().iter().next().unwrap().clone();

        for ingredients in allergens.values_mut() {
            ingredients.remove(&ingredient);
        }

        allergen_ingredients.insert(allergen, ingredient);
    }

    let mut allergens: Vec<_> = allergen_ingredients.keys().cloned().collect();
    allergens.sort();

    let part2: Vec<_> = allergens.iter().map(|a| allergen_ingredients[a].clone()).collect();
    println!("Part 2: {}", part2.join(","));
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Food> {
    let re = Regex::new(r"(.*) \(contains (.*)\)").unwrap();

    fs::read_to_string(path).expect("Failed to read input")
        .lines()
        .map(|line| {
            let m = re.captures(&line).expect("Failed to parse line");
            let ingredients = m[1].split_whitespace().map(|s| s.to_owned()).collect();
            let allergens = m[2].split(", ").map(|s| s.to_owned()).collect();

            Food { ingredients, allergens }
        }).collect()
}

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
