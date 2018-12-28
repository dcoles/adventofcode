use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let mut grid = Grid::new(350, 350);
    let input= read_input();

    // Part 1

    let mut infinite_points = HashSet::new();
    let mut point_area: HashMap<u8, u32> = HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let c1 = Coord(x as i32, y as i32);
            let mut distance: Vec<(i32, u8)> = Vec::new();
            for (n, c2) in (1u8..).zip(&input) {
                distance.push((c1.distance(c2), n));
            }
            distance.sort();

            let n = distance[0].1;
            if distance[0].0 != distance[1].0 {
                grid.set(&c1, Value::Some(n));
                *point_area.entry(n).or_default() += 1;
                if x == 0 || y == 0 || x == grid.width - 1 || y == grid.height - 1 {
                    infinite_points.insert(n);
                }
            } else {
                grid.set(&c1, Value::Conflict);
            }
        }
    }

    grid.print();

    for k in &infinite_points {
        point_area.remove(k);
    }

    let mut area_point: Vec<(_, _)> = point_area.iter().map(|(k,v)| (v, k)).collect();
    area_point.sort();

    let (area, point) = area_point.last().unwrap();
    println!("Point #{} has the largest non-infinite area: {}", point, area);

}

fn read_input() -> Vec<Coord> {
    let mut result = Vec::new();

    let input = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    for line in input.lines() {
        let v: Vec<i32> = line.splitn(2, ",")
            .map(|s| s.trim().parse().expect("Failed to parse coord"))
            .collect();
        result.push(Coord(v[0], v[1]));
    }

    result
}

#[derive(Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
struct Coord(i32, i32);

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Clone)]
enum Value {
    None,
    Some(u8),
    Conflict,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<Vec<Value>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {width, height, values: vec![vec![Value::None; width]; height] }
    }

    fn print(&self) {
        for row in &self.values {
            println!("{}", row.iter()
                .map(|v| match v {
                    Value::None => String::from(" ."),
                    Value::Some(v) => format!("{:2}", v),
                    Value::Conflict => String::from(" *"),
                })
                .collect::<Vec<String>>()
                .join(" "))
        }
    }

    fn set(&mut self, coord: &Coord, value: Value) {
        self.values[coord.1 as usize][coord.0 as usize] = value;
    }
}
