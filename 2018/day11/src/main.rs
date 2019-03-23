const SERIAL_NUMBER: i32 = 5791;
const MAX_SIZE: usize = 300;

fn main() {
    // Example
    assert_eq!(power((3, 5), 8), 4);
    assert_eq!(power((122, 79), 57), -5);
    assert_eq!(power((217, 196), 39), 0);
    assert_eq!(power((101, 153), 71), 4);

    let mut grid = [[0; MAX_SIZE]; MAX_SIZE];
    for (y, row) in (1..=300).zip(&mut grid[..]) {
        for (x, val) in (1..=300).zip(&mut row[..]) {
            *val = power((x, y), SERIAL_NUMBER);
        }
    }

    // Part 1

    let mut max_coord = (0, 0);
    let mut max_power = 0;
    for y in 1..=MAX_SIZE - 2 {
        for x in 1..=MAX_SIZE - 2 {
            let mut power = 0;
            for yoff in 0..3 {
                for xoff in 0..3 {
                    power += &grid[y+yoff-1][x+xoff-1];
                }
            }

            if power > max_power {
                max_coord = (x, y);
                max_power = power;
            }
        }
    }

    println!("Max power for 3x3 is {}JW at {:?}", max_power, max_coord);

    // Part 2

    let mut max_coord = (0, 0);
    let mut max_size = 0;
    let mut max_power = 0;
    for size in 1..=MAX_SIZE {
        println!("{}...", size);
        for y in 1..=MAX_SIZE - size + 1 {
            for x in 1..=MAX_SIZE - size + 1 {
                let mut power = 0;
                for yoff in 0..size {
                    for xoff in 0..size {
                        power += &grid[y+yoff-1][x+xoff-1];
                    }
                }

                if power > max_power {
                    max_coord = (x, y);
                    max_size = size;
                    max_power = power;
                }
            }
        }
    }

    println!("Max power {}JW at {:?} (size: {})", max_power, max_coord, max_size);
}

fn power(coord: (i32, i32), serial: i32) -> i32 {
    let rack_id = coord.0 + 10;
    let mut power = rack_id * coord.1;
    power += serial;
    power *= rack_id;
    power = power % 1000 / 100;
    power -= 5;

    power
}
