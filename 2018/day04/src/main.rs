use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let mut input: Vec<_> = input.lines().collect();
    input.sort();

    let guard_sleeping = parse_schedule(input);
    for (guard, sleeping) in guard_sleeping.iter() {
        println!("Guard {}: {:?}", guard, &sleeping[..])
    }

    // Part 1
    let (&max_guard, max_value) = guard_sleeping.iter()
        .map(|(k, v)| (k, v.iter().sum::<u32>()))
        .max_by_key(|(_k, v)| *v)
        .unwrap();
    println!("Guard {} slept for {} minutes", max_guard, max_value);

    let mut minutes_days: HashMap<u32, u32> = HashMap::new();
    let sleeping = guard_sleeping.get(&max_guard).unwrap();
    for (minute, &minutes) in sleeping.iter().enumerate() {
        *minutes_days.entry(minute as u32).or_default() += minutes;
    }

    let (&max_minute, _) = minutes_days.iter().max_by_key(|(_k, v)| **v).unwrap();
    println!("Max minute: {}", max_minute);

    // Part 2
    let mut max_guard = 0;
    let mut max_minute = 0;
    let mut max_days = 0;
    for (&guard, sleeping) in guard_sleeping.iter() {
        for (minute, &days) in sleeping.iter().enumerate() {
            if days > max_days {
                max_guard = guard;
                max_minute = minute;
                max_days = days;
            }
        }
    }
    println!("Guard {} is most frequently asleep at 00:{:02}", max_guard, max_minute)


}

fn parse_schedule(input: Vec<&str>) -> HashMap<u32, [u32; 60]> {
    let mut sleeping: HashMap<u32, [u32; 60]> = HashMap::new();

    let mut guard_id = 0;
    let mut last_t: usize = 0;
    for &line in input.iter() {
        println!("{}", line);
        let t = line[15..17].parse()
            .expect("Failed to parse minute");
        let action = &line[19..];

        if action.starts_with("Guard") {
            guard_id = action.split_whitespace().nth(1)
                .expect("Could not parse Guard line")
                [1..].parse()
                    .expect("Could not parse Guard ID");
        } else if action.starts_with("wakes up") {
            for t in last_t..t {
                sleeping.entry(guard_id).or_insert([0u32; 60])[t] += 1;
            }
        }
        last_t = t;
    }
    sleeping
}

