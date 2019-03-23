use std::collections::HashSet;
use std::fs;

const ORIGIN: Pos = [0; 3];

fn main() {
    let mut nanobots = read_input("input.txt");
    nanobots.sort_by_key(|n| n.r);

    // Part 1
    let strongest = nanobots.last().unwrap();
    let in_range = nanobots.iter().filter(|n| strongest.in_range(n)).count();
    println!("Strongest is {:?} with {} in range", strongest, in_range);

    // Part 2
    let n = i32::pow(2, 30);
    let cube = Cube::new([-n, -n, -n], u32::pow(2, 31));
    search(cube, &nanobots);
}

fn search(cube: Cube, nanobots: &Vec<Nanobot>) {
    let mut edge = Vec::new();
    edge.push((0, cube));

    let mut max_count = 0;
    let mut result = HashSet::new();
    while ! edge.is_empty() {
        edge.sort_by_key(|(d, _)| *d);
        let (count, cube) = edge.pop().unwrap();

        //println!("Cube {:?} count {}", cube, count);

        if cube.width == 1 {
            if count < max_count {
                break;
            }
            result.insert(cube.pos);
            max_count = count;
            continue;
        }

        for oct in octants(cube) {
            let oct_count = nanobots.iter().filter(|n|
                in_range_within_interval(n, oct)
            ).count();

            if count >= max_count {
                //println!("- {:?} count {}", oct, oct_count);
                edge.push((oct_count, oct));
            }
        }
    }

    let result: Vec<_> = result.into_iter().map(|cube| (distance(ORIGIN, cube), cube)).collect();
    let &(best_distance, best_cube) = result.first().unwrap();

    println!("Best position is {:?} with {} nanobots in range (distance-from-origin: {})", best_cube, max_count, best_distance);
}

fn octants(cube: Cube) -> Vec<Cube> {
    let mut result = Vec::new();
    for &x in &[cube.pos[0], cube.pos[0] + (cube.width / 2) as i32] {
        for &y in &[cube.pos[1], cube.pos[1] + (cube.width / 2) as i32] {
            for &z in &[cube.pos[2], cube.pos[2] + (cube.width / 2) as i32] {
                result.push(Cube::new([x, y, z], cube.width / 2));
            }
        }
    }

    result
}

fn in_range_within_interval(nanobot: &Nanobot, cube: Cube) -> bool {
    let mut distance = 0;
    for d in 0..3 {
        let min = cube.pos[d] as i64;
        let max = min + cube.width as i64;
        let pos = nanobot.pos[d] as i64;
        if pos < min {
            distance += min - pos;
        } else if pos >= max {
            distance += pos - max + 1;
        }
    }

    distance <= nanobot.r as i64
}

fn read_input(filename: &str) -> Vec<Nanobot> {
    let input = fs::read_to_string(filename)
        .expect("Failed to read input");

    let mut result = Vec::new();
    for line in input.lines() {
        let mut pos = [0; 3];
        let mut r = 0;
        for col in line.split_whitespace() {
            let mut iter = col.split("=");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();

            match key {
                "pos" => {
                    let mut iter = value[1..value.len()-2].split(",");
                    pos = [
                        parse_int(iter.next().unwrap()),
                        parse_int(iter.next().unwrap()),
                        parse_int(iter.next().unwrap()),
                    ];
                },
                "r" => r = parse_int(value),
                _ => (),
            }
        }

        result.push(Nanobot { pos, r })
    }

    result
}

fn parse_int(s: &str) -> i32 {
    s.parse().expect("Failed to parse int")
}

type Pos = [i32; 3];

#[derive(Debug, Copy, Clone)]
struct Nanobot {
    pos: Pos,
    r: i32,
}

impl Nanobot {
    fn distance(&self, pos: Pos) -> i32 {
        distance(self.pos, pos)
    }

    fn in_range(&self, other: &Nanobot) -> bool {
        self.distance(other.pos) <= self.r
    }
}

#[derive(Debug, Copy, Clone)]
struct Cube {
    pos: Pos,
    width: u32,
}

impl Cube {
    fn new(pos: Pos, width: u32) -> Cube {
        Cube { pos, width }
    }
}

fn distance(pos1: Pos, pos2: Pos) -> i32 {
    let mut result = 0;
    for n in 0..pos1.len() {
        result += (pos1[n] - pos2[n]).abs()
    }

    result
}
