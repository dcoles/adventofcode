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

fn split_layers(data: &[Pixel], width: usize, height: usize) -> Vec<Pixmap> {
    let mut layers = Vec::new();
    let npixels = width * height;
    let mut iter = data.iter();

    'outer: loop {
        let mut pixdata = Vec::new();
        for i in 0..npixels {
            if let Some(&value) = iter.next() {
                pixdata.push(value);
            } else {
                if i != 0 {
                    panic!("Truncated data");
                }
                // No more layers
                break 'outer;
            }
        }
        layers.push(Pixmap::new(&pixdata, width, height).expect("Invalid dimensions"));
    }

    layers
}

fn count_digits(layers: &[Pixmap]) -> Vec<HashMap<Pixel, u32>> {
    layers.iter().map(|layer| {
        let mut count = HashMap::new();
        for &pixel in layer.data() {
            *count.entry(pixel).or_default() += 1;
        }

        count
    }).collect()
}

fn draw(layers: &[Pixmap]) {
    let nlayers = layers.len();
    let height = layers[0].height();
    let width = layers[0].width();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let mut pixel = 'X';
            for l in (0..nlayers).rev() {
                match layers[l].pixel(x, y) {
                    0 => pixel = ' ',  // Black
                    1 => pixel = '█',  // White
                    2 => (),  // Transparent
                    pixel => panic!("Unknown pixel {}", pixel),
                }
            }
            line.push(pixel);
        }
        println!("{}", line);
    }
}

struct Pixmap {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Pixmap {
    fn new(data: &[Pixel], width: usize, height: usize) -> Option<Pixmap> {
        if data.len() != width * height {
            return None;
        }
        Some(Pixmap { data: data.to_owned(), width, height })
    }

    fn data(&self) -> &[Pixel] {
        &self.data
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixel(&self, x: usize, y: usize) -> Pixel {
        if x > self.width {
            panic!("x is out of bounds: {} > {} (width)", x, self.width)
        }
        if y > self.height {
            panic!("y is out of bounds: {} > {} (height)", x, self.height)
        }
        self.data[self.width * y + x]
    }
}
