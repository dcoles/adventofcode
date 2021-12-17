//! Advent of Code 2021: Day 17
//! https://adventofcode.com/2021/day/17

use std::ops::RangeInclusive;

type Vec2 = [i32; 2];
type Target = (RangeInclusive<i32>, RangeInclusive<i32>);

// Input
const TARGET_AREA: (RangeInclusive<i32>, RangeInclusive<i32>) = (211..=232, -124..=-69);

fn main() {
    // Part 1
    println!("Part 1: {}", part1(&TARGET_AREA));

    // Part 2
    println!("Part 2: {}", part2(&TARGET_AREA));
}

fn part1(target_area: &Target) -> i32 {
    let mut max = 0;
    for dx in -1000..1000 {
        for dy in -1000..1000 {
            if let Some(path) = simulate([dx, dy], target_area) {
                let height = path.into_iter().map(|[_, y]| y).max().unwrap();

                if height > max {
                    max = height;
                }
            }
        }
    }

    max
}

fn part2(target_area: &Target) -> usize {
    let mut count = 0;
    for dx in -1000..1000 {
        for dy in -1000..1000 {
            if simulate([dx, dy], target_area).is_some() {
                count += 1;
            }
        }
    }

    count
}

fn simulate(velocity: Vec2, target_area: &Target) -> Option<Vec<Vec2>> {
    let mut pos = [0; 2];
    let mut vel = velocity;

    let mut positions = Vec::new();
    loop {
        pos = [pos[0] + vel[0], pos[1] + vel[1]];
        positions.push(pos);

        if vel[0] > 0 {
            vel[0] -= 1;
        } else if vel[0] < 0 {
            vel[0] += 1;
        }
        
        vel[1] -= 1;

        if target_area.0.contains(&pos[0]) && target_area.1.contains(&pos[1]) {
            return Some(positions);
        } else if pos[0] > *target_area.0.end() || pos[1] < *target_area.1.start() {
            // Overshoot
            return None
        } else if pos[0] < *target_area.0.start() && vel[0] <= 0 {
            // Stalled
            return None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let target_area = (20..=30, -10..=-5);

        assert_eq!(simulate([7, 2], &target_area).unwrap().len(), 7);
        assert_eq!(simulate([6, 3], &target_area).unwrap().len(), 9);
        assert_eq!(simulate([9, 0], &target_area).unwrap().len(), 4);
        assert!(simulate([17, -4], &target_area).is_none());

        assert_eq!(part1(&target_area), 45);
    }

    #[test]
    fn test_part2() {
        let target_area = (20..=30, -10..=-5);

        assert_eq!(part2(&target_area), 112);
    }
}
