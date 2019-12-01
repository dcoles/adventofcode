use std::path::Path;
use std::fs;

fn main() {
    let input = read_input("input.txt");

    // Part 1
    assert_eq!(2, fuel1(12));
    assert_eq!(2, fuel1(14));
    assert_eq!(654, fuel1(1969));
    assert_eq!(33583, fuel1(100756));

    let total_fuel1: u32 = input.iter().copied().map(fuel1).sum();
    println!("Part 1: Sum of fuel requirements: {}", total_fuel1);

    // Part 2
    assert_eq!(2, fuel(14));
    assert_eq!(966, fuel(1969));
    assert_eq!(50346, fuel(100756));

    let total_fuel2: u32 = input.iter().copied().map(fuel).sum();
    println!("Part 2: Sum of fuel requirements: {}", total_fuel2);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<u32> {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.lines().map(|line| line.parse::<u32>().expect("Failed to parse input")).collect()
}

/// Fuel required to launch a given module is based on its mass.
/// Specifically, to find the fuel required for a module,
/// take its mass, divide by three, round down, and subtract 2.
fn fuel1(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

/// Fuel itself requires fuel just like a module.
/// So, for each module mass, calculate its fuel and add it to the total.
/// Then, treat the fuel amount you just calculated as the input mass
/// and repeat the process, continuing until a fuel requirement is zero or negative.
fn fuel(mass: u32) -> u32 {
    let mut result = 0;
    let mut partial_mass = mass;

    while partial_mass > 0 {
        let partial_fuel = fuel1(partial_mass);
        result += partial_fuel;
        partial_mass = partial_fuel;
    }

    result
}


