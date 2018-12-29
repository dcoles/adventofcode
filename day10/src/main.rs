use std::fs;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut input = read_input();

    let mut t = 0;
    while input[0].distance(&input[1]) > 1 {
        for point in &mut input {
            point.tick();
        }
        t += 1;
    }

    println!("{:?}", input);
    println!("After {} ticks:", t);
    draw(&input);
}

fn read_input() -> Vec<Point> {
    let mut result = Vec::new();
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    for line in input.lines() {
        let p1: i32 = (&line[10..16]).trim().parse().expect("Failed to parse");
        let p2: i32 = (&line[18..24]).trim().parse().expect("Failed to parse");
        let v1: i32 = (&line[36..38]).trim().parse().expect("Failed to parse");
        let v2: i32 = (&line[40..42]).trim().parse().expect("Failed to parses");
        result.push(Point { position: (p1, p2), velocity: (v1, v2)})
    }

    result
}

#[derive(Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn tick(&mut self) {
        self.position = (self.position.0 + self.velocity.0, self.position.1 + self.velocity.1);
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.position.0 - other.position.0)^2 + (self.position.1 - other.position.1)^2
    }
}

fn draw(input: &Vec<Point>) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; WIDTH]; HEIGHT];

    for point in input {
        if point.position.0 < 0 || point.position.0 >= WIDTH as i32 || point.position.1 < 0 || point.position.1 >= HEIGHT as i32 {
            continue;
        }

        grid[point.position.1 as usize][point.position.0 as usize] = '#';
    }

    for row in &grid {
        let line: String = row.into_iter().map(|c| format!("{} ", c)).collect();
        println!("{}", line);
    }
}
