use std::collections::HashSet;
use std::fs;

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const FCURVE: char = '/';
const BCURVE: char = '\\';
const VTRACK: char = '|';
const HTRACK: char = '-';
const XSECT: char = '+';

fn main() {
    let mut world = World::from_file("input.txt");

    world.print();
    loop {
        if let Err(err) = world.tick() {
            world.print();
            println!("Oh no! {}", err);
            break;
        }
        world.print();
    }

}

struct World {
    map: Vec<Vec<char>>,
    carts: Vec<Cart>,
    t: u32,
}

impl World {
    fn from_file(path: &str) -> World {
        let mut map = Vec::new();
        let mut carts = Vec::new();

        let input = fs::read_to_string(path)
            .expect("Failed to read input");

        for (y, line) in input.lines().enumerate() {
            let mut line_map = Vec::new();
            for (x, val) in line.chars().enumerate() {
                if World::is_cart(val) {
                    line_map.push(World::cart_track(val));
                    carts.push(Cart::new((x, y), val));
                } else {
                    line_map.push(val);
                }
            }
            map.push(line_map);
        }

        World { map, carts, t: 0 }
    }

    fn print(&self) {
        let mut map = self.map.clone();

        // Add carts to map
        for cart in &self.carts {
            map[cart.position.1][cart.position.0] = cart.direction;
        }

        // Show map
        for (y, row) in map.iter().enumerate() {
            print!("{:3} ", y);
            for &val in row {
                if World::is_cart(val) {
                    print!("\x1b[31m{}\x1b[0m", val)  // RED
                } else {
                    print!("{}", val)
                }
            }
            println!()
        }
    }

    fn tick(&mut self) -> Result<(), String> {
        // Sort by row, then column
        self.carts.sort_by_key(|c| (c.position.1, c.position.0));

        let mut positions = HashSet::new();
        for cart in &mut self.carts {
            if positions.contains(&cart.position) {
                return Err(format!("Carts crashed at {},{}", cart.position.0, cart.position.1))
            }
            println!("Moving cart at {},{}", cart.position.0, cart.position.1);
            let track = self.map[cart.position.1][cart.position.0];
            cart.tick(track);
            positions.insert(cart.position);
        }
        self.t += 1;

        Ok(())
    }

    fn is_cart(c: char) -> bool {
        c == UP || c == DOWN || c == LEFT || c == RIGHT
    }

    fn cart_track(c: char) -> char {
        if c == UP || c == DOWN {
            VTRACK
        } else if c == LEFT || c == RIGHT {
            HTRACK
        } else {
            panic!("Unknown value {:?}", c);
        }
    }
}

struct Cart {
    position: (usize, usize),
    direction: char,
    n_xsect: u32,
}

impl Cart {
    fn new(position: (usize, usize), direction: char) -> Cart {
        Cart { position, direction, n_xsect: 0 }
    }

    fn tick(&mut self, track: char) {
        let (x, y) = self.position;
        match track {
            FCURVE => match self.direction {
                UP => self.right(),
                DOWN => self.left(),
                LEFT => self.down(),
                RIGHT => self.up(),
                _ => panic!("Cart derailed at {},{}", x, y),
            },
            BCURVE => match self.direction {
                UP => self.left(),
                DOWN => self.right(),
                LEFT => self.up(),
                RIGHT => self.down(),
                _ => panic!("Cart derailed at {},{}", x, y),
            },
            XSECT => {
                match self.n_xsect % 3 {
                    0 => self.turn_left(),
                    1 => self.straight_ahead(),
                    _ => self.turn_right(),
                }
                self.n_xsect += 1;
            },
            _ => self.straight_ahead(),
        }
    }

    fn up(&mut self) {
        self.direction = UP;
        self.position = (self.position.0, self.position.1 - 1);
    }

    fn down(&mut self) {
        self.direction = DOWN;
        self.position = (self.position.0, self.position.1 + 1);
    }

    fn left(&mut self) {
        self.direction = LEFT;
        self.position = (self.position.0 - 1, self.position.1);
    }

    fn right(&mut self) {
        self.direction = RIGHT;
        self.position = (self.position.0 + 1, self.position.1);
    }

    fn turn_left(&mut self) {
        match self.direction {
            UP => self.left(),
            DOWN => self.right(),
            LEFT => self.down(),
            RIGHT => self.up(),
            _ => panic!("Unknown direction: {:?}", self.direction)
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            UP => self.right(),
            DOWN => self.left(),
            LEFT => self.up(),
            RIGHT => self.down(),
            _ => panic!("Unknown direction: {:?}", self.direction)
        }
    }

    fn straight_ahead(&mut self) {
        match self.direction {
            UP => self.up(),
            DOWN => self.down(),
            LEFT => self.left(),
            RIGHT => self.right(),
            _ => panic!("Unknown direction: {:?}", self.direction)
        }
    }
}

