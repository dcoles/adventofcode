use std::fs;
use std::path::Path;
use std::collections::HashMap;

type Pixel = u8;

fn main() {
    let input = read_input("input.txt");
    let layers = split_layers(&input, 25, 6);
    let mut counts = count_digits(&layers);

    // Part 1
    counts.sort_by_key(|m| *m.get(&0).unwrap_or(&0));
    let layer = counts.first().expect("No first layer");
    let checksum = layer[&1] * layer[&2];
    println!("Part 1: Number of 1 digits multiplied by number of 2 digits: {}", checksum);

    // Part 2
    println!("Part 2:");
    draw(&layers);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Pixel> {
    let contents = fs::read_to_string(path).expect("Failed to read input");
    contents.trim().chars().map(|c| c.to_digit(10).expect("Not a digit") as Pixel).collect()
}

fn split_layers(data: &[Pixel], width: usize, height: usize) -> Vec<Vec<Vec<Pixel>>> {
    let mut layers = Vec::new();
    let mut iter = data.iter();

    'outer: loop {
        let mut layer = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                if let Some(&value) = iter.next() {
                    row.push(value);
                } else {
                    break 'outer;
                }
            }
            layer.push(row);
        }
        layers.push(layer);
    }

    layers
}

fn count_digits(layers: &[Vec<Vec<Pixel>>]) -> Vec<HashMap<Pixel, u32>> {
    let mut counts = Vec::new();

    for layer in layers.iter() {
        let mut count: HashMap<Pixel, u32> = HashMap::new();
        for row in layer {
            for &val in row {
                *count.entry(val).or_default() += 1;
            }
        }
        counts.push(count);
    }

    counts
}

fn draw(layers: &[Vec<Vec<Pixel>>]) {
    let nlayers = layers.len();
    let height = layers[0].len();
    let width = layers[0][0].len();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let mut pixel = 'X';
            for l in (0..nlayers).rev() {
                match layers[l][y][x] {
                    0 => pixel = ' ',  // Black
                    1 => pixel = '█',  // White
                    2 => (),  // Transparent
                    _ => panic!("Unknown pixel {}", pixel),
                }
            }
            line.push(pixel);
        }
        println!("{}", line);
    }
}
