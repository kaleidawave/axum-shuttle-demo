use std::io::{Seek, Write};

use image::{ImageBuffer, ImageOutputFormat, ImageResult, Rgb};
use noise::{NoiseFn, Perlin};

/// Stable, imperfect, *simple*, hash function for [str]s
fn get_seed(string: &str) -> u32 {
    string
        .chars()
        .take(10)
        .fold(0, |a, b| u32::wrapping_add(a, b as u32))
}

const IMAGE_SIZE: u32 = 64;
const BLOCK_SIZE: u32 = 4;
const SCALE: f64 = 8.;
const COLORS: [(u8, u8, u8); 5] = [
    (217, 30, 65),
    (115, 50, 92),
    (38, 36, 115),
    (30, 28, 89),
    (242, 58, 41),
];
const IMAGE_FORMAT: ImageOutputFormat = ImageOutputFormat::Png;

pub fn get_image<W: Write + Seek>(string: &str, writer: &mut W) -> ImageResult<()> {
    let perlin = Perlin::new();
    let offset = get_seed(string) as f64;

    let mut image_buffer = ImageBuffer::new(IMAGE_SIZE, IMAGE_SIZE);

    // Iterate over blocks:
    for x in 0..(IMAGE_SIZE / BLOCK_SIZE) {
        for y in 0..(IMAGE_SIZE / BLOCK_SIZE) {
            let perlin_value = perlin.get([x as f64 / SCALE, y as f64 / SCALE + offset]);
            // perlin.get returns values [-1..1].
            // Here it adjust the value to be in the range [0..COLORS.len()]:
            const INDEX_OFFSET: f64 = (COLORS.len() - 1) as f64 / 2.;
            let value = ((perlin_value + 1.) * INDEX_OFFSET).floor() as usize;
            let (red, green, blue) = COLORS[value];
            // Write in all the pixel values for the blocks:
            for x_offset in 0..BLOCK_SIZE {
                for y_offset in 0..BLOCK_SIZE {
                    image_buffer[(x * BLOCK_SIZE + x_offset, y * BLOCK_SIZE + y_offset)] =
                        Rgb([red, green, blue]);
                }
            }
        }
    }

    image_buffer.write_to(writer, IMAGE_FORMAT)
}
