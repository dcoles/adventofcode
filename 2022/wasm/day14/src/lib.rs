//! Advent of Code 2022: Day 14
//! https://adventofcode.com/2022/day/14

mod page;
mod util;

use std::collections::HashMap;
use std::io;
use std::time::Duration;

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

use crate::util::sleep;

const WIDTH: usize = 1000;
const HEIGHT: usize = 400;
const SAND: char = 'o';
const ROCK: char = '#';
const SOURCE: (usize, usize) = (500, 0);

#[wasm_bindgen]
pub async fn run(input: String) {
    let map = match read_input(&input) {
        Err(err) => {
            log!("ERROR: Failed to parse input: {}", err);
            return;
        },
        Ok(m) => m,
    };

    simulate(&map).await;
}

/// Simulate falling of sand
async fn simulate(map: &HashMap<(usize, usize), char>) {
    let mut map = map.clone();
    let floor = map.keys().map(|p| p.1 + 2).max().unwrap();

    for unit in 1.. {
        // Sand starts at the source
        let mut pos = SOURCE;

        loop {
            loop {
                if map.contains_key(&(pos.0, pos.1 + 1)) || pos.1 + 1 == floor {
                    // We hit something!
                    break;
                }

                // Keep falling...
                pos = (pos.0, pos.1 + 1);
            }

            // See if we can slide diagonally
            if !map.contains_key(&(pos.0 - 1, pos.1 + 1)) && pos.1 + 1 != floor {
                // Diagonal left was empty
                pos = (pos.0 - 1, pos.1 + 1);

                continue;
            } else if !map.contains_key(&(pos.0 + 1, pos.1 + 1)) && pos.1 + 1 != floor {
                // Diagonal right was empty
                pos = (pos.0 + 1, pos.1 + 1);

                continue;
            }

            // We must be blocked - this is where we come to rest
            map.insert((pos.0, pos.1), SAND);
            display(&map);

            if pos == SOURCE {
                // We blocked the source
                log!("{} units of sand", unit);
                return;
            }

            sleep(Duration::from_millis(16)).await;
            break;
        }
    }
}

/// Display map on HTML Canvas
fn display(map: &HashMap<(usize, usize), char>) {
    let mut data = vec![0; 4 * WIDTH * HEIGHT];

    // Draw map
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match map.get(&(x, y)) {
                Some(&ROCK) => {
                    // 2x scale
                    for y in 2*y..2*y+2 {
                        for x in 2*x..2*x+2 {
                            set_pixel(&mut data, (x - WIDTH / 2, y), 0xff0000ff);
                        }
                    }
                },
                Some(&SAND) => {
                    // Do a dithering effect
                    set_pixel(&mut data, ((2 * x) - WIDTH / 2, (2 * y)), 0xffff00ff);
                    set_pixel(&mut data, ((2 * x) - WIDTH / 2 + 1, (2 * y) + 1), 0xffff00ff);
                },
                _ => (),
            }
        }
    }

    // Draw source
    set_pixel(&mut data, SOURCE, 0x00ffffff);

    let ctx = page::canvas_context_2d();

    let imagedata = match ImageData::new_with_u8_clamped_array(Clamped(&data), WIDTH as u32) {
        Err(err) => {
            log!("ERROR: Failed to create ImageData: {:?}", err);
            return;
        },
        Ok(d) => d,
    };

    ctx.put_image_data(&imagedata, 0.0, 0.0).unwrap();
}

/// Set pixel data
fn set_pixel(data: &mut [u8], (x, y): (usize, usize), color: u32) {
    data[4 * (y * WIDTH + x) + 0] = (color >> 24) as u8;
    data[4 * (y * WIDTH + x) + 1] = (color >> 16) as u8;
    data[4 * (y * WIDTH + x) + 2] = (color >> 8) as u8;
    data[4 * (y * WIDTH + x) + 3] = color as u8;
}

/// Read input
fn read_input(input: &str) -> io::Result<HashMap<(usize, usize), char>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let trace = parse_line(&line)?;

        for window in trace.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];

            for y in y1.min(y2)..=y1.max(y2) {
                for x in x1.min(x2)..=x1.max(x2) {
                    map.insert((x, y), ROCK);
                }
            }
        }
    }

    Ok(map)
}

/// Parse input of the format `x,y -> x,y -> x,y`.
fn parse_line(line: &str) -> io::Result<Vec<(usize, usize)>> {
    let mut path = Vec::new();

    for value in line.split(" -> ") {
        let (x, y) = value.split_once(",").ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to parse coordinate"))?;

        let x = x.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, format!("not a number {:?}", x)))?;
        let y = y.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, format!("not a number {:?}", y)))?;

        path.push((x, y));
    }

    Ok(path)
}
