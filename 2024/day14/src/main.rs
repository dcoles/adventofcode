//! Advent of Code 2024: Day 14
//! https://adventofcode.com/2024/day/14

use std::{fs, io};
use std::collections::BTreeSet;
use std::path::Path;
use lib::vector::Vector;

const ROOM_SIZE: [i32; 2] = [101, 103];

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input, ROOM_SIZE));

    // Part 2
    println!("Part 2: {}", part2(&input, ROOM_SIZE));
}

fn part1(input: &Input, room_size: [i32; 2]) -> usize {
    let (n, robots) = simulate(&input.values, room_size, &|_| false, 100);

    println!("After {n}:");
    draw_robots(&robots, room_size);

    let quad_size = [room_size[0] / 2, room_size[1] / 2];
    let mut safety_factor = 1;

    for y in 0..2 {
        for x in 0..2 {
            let quad_range = [
                (x * (quad_size[0] + 1))..(x * (quad_size[0] + 1) + quad_size[0]),
                (y * (quad_size[1] + 1))..(y * (quad_size[1] + 1) + quad_size[1]),
            ];

            let count = robots.iter().filter(|r| {
                quad_range[0].contains(&r.0[0])
                && quad_range[1].contains(&r.0[1])
            }).count();

            safety_factor *= count;
        }
    }

    safety_factor
}

fn part2(input: &Input, room_size: [i32; 2]) -> usize {
    let condition = |robots: &[(Vec2, Vec2)] | {
        let count = robots.iter().filter(|r| {
            ((1 * room_size[0] / 3)..(2 * room_size[0] / 3)).contains(&r.0[0])
                && ((1 * room_size[1] / 3)..(2 * room_size[1] / 3)).contains(&r.0[1])
        }).count();

        count > robots.len() / 2
    };

    let (n, robots) = simulate(&input.values, room_size, &condition, 10000);

    println!("After {n}:");
    draw_robots(&robots, room_size);

    n
}

fn simulate(robots: &[(Vec2, Vec2)], room_size: [i32; 2], condition: &dyn Fn(&[(Vec2, Vec2)]) -> bool, max_steps: usize) -> (usize, Vec<(Vec2, Vec2)>) {
    let mut robots= robots.to_vec();

    for n in 0..max_steps {
        for robot in robots.iter_mut() {
            robot.0 += robot.1;

            for n in 0..2 {
                robot.0[n] = robot.0[n].rem_euclid(room_size[n]);
            }
        }

        if condition(&robots) {
            return (n + 1, robots);
        }
    }

    (max_steps, robots)
}

fn draw_robots(robots: &[(Vec2, Vec2)], room_size: [i32; 2]) {
    let positions: BTreeSet<Vec2> = robots.iter().map(|r| r.0).collect();
    for y in 0..room_size[1] {
        for x in 0..room_size[0] {
            if positions.contains(&Vec2::new([x, y])) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

type Vec2 = Vector<i32, 2>;

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(Vec2, Vec2)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines()
            .map(|line| {
                let (p, v) = line.trim().split_once(' ').unwrap();

                let (_, p) = p.split_once('=').unwrap();
                let (px, py) = p.split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap();

                let (_, v) = v.split_once('=').unwrap();
                let (vx, vy) = v.split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap();

                (Vec2::new([px, py]), Vec2::new([vx, vy]))
            })
            .collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input, [11, 7]), 12);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input, ROOM_SIZE), 236628054);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input, ROOM_SIZE), 7584);
    }
}
