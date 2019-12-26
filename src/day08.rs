use std::fs::File;
use std::io::prelude::*;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const IMAGE_SIZE: usize = IMAGE_HEIGHT * IMAGE_WIDTH;

struct SpaceImage {
    layers: Vec<[u8; IMAGE_SIZE]>,
}

impl SpaceImage {
    pub fn read_from_file(file_name: &str) -> Self {
        let mut data = File::open(file_name).expect("Input file missing!");

        let mut img = SpaceImage { layers: Vec::new() };
        loop {
            let mut layer = [0u8; IMAGE_SIZE];
            let res = data.read(&mut layer).unwrap();
            if res == 0 {
                break;
            }
            img.layers.push(layer);
        }

        img
    }
}

pub fn solve(input_data: &str) {
    let image = SpaceImage::read_from_file(input_data);

    let image_stats: Vec<(usize, usize)> =image.layers.iter().map(|&x| {
        (
            x.iter().filter(|&x| *x == 48).count(),
            x.iter().filter(|&x| *x == 49).count() * x.iter().filter(|&x| *x == 50).count(),
        )
    }).collect();
    let layer = image_stats.iter().min_by(|x, y| x.0.cmp(&y.0)).expect("Largest layer");

    println!("Day 08.1: Space image check value is {}", layer.1);
}

fn sum_layers(l1: &[u8; IMAGE_SIZE], l2: &[u8; IMAGE_SIZE]) -> [u8; IMAGE_SIZE] {
    let mut final_layer = [50u8; IMAGE_SIZE];
    for i in 0..IMAGE_SIZE {
        final_layer[i] = if l1[i] == 50 { l2[i] } else { l1[i] };
    }

    final_layer
}

pub fn solve_pt2(input_data: &str, print_result: bool) {
    let image = SpaceImage::read_from_file(input_data);

    let final_image = image.layers.iter().fold([50u8; IMAGE_SIZE], |acc, x| sum_layers(&acc, x));
    println!("Day 08.2: Final image is");
    if print_result {
        for y in 0..IMAGE_HEIGHT {
            print!("\t");
            for x in 0..IMAGE_WIDTH {
                match final_image[y * IMAGE_WIDTH + x] {
                    48 => print!(" "),
                    49 => print!("X"),
                    50 => print!("?"),
                    _ => panic!(format!("Unsupported value => {}", final_image[y*x]) )
                }
            }
            println!("");
        }
    }
}
